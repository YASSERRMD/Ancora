/// placement - Drag-and-drop node placement on the canvas.
use crate::scaffold::{Id, Position};
use std::collections::HashMap;

/// A node instance placed on the canvas.
#[derive(Debug, Clone)]
pub struct CanvasNode {
    pub id: Id,
    /// The palette kind (e.g. "agent.llm").
    pub kind: String,
    pub label: String,
    pub position: Position,
    pub size: NodeSize,
    /// Per-node configuration overrides.
    pub config: HashMap<String, String>,
}

impl CanvasNode {
    pub fn new(
        id: Id,
        kind: impl Into<String>,
        label: impl Into<String>,
        position: Position,
    ) -> Self {
        CanvasNode {
            id,
            kind: kind.into(),
            label: label.into(),
            position,
            size: NodeSize::default(),
            config: HashMap::new(),
        }
    }

    pub fn with_config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.config.insert(key.into(), value.into());
        self
    }
}

/// Node dimensions.
#[derive(Debug, Clone, Copy)]
pub struct NodeSize {
    pub width: f64,
    pub height: f64,
}

impl Default for NodeSize {
    fn default() -> Self {
        NodeSize {
            width: 160.0,
            height: 48.0,
        }
    }
}

impl NodeSize {
    pub fn new(width: f64, height: f64) -> Self {
        NodeSize { width, height }
    }
}

/// Drag operation state.
#[derive(Debug, Clone)]
pub struct DragState {
    pub node_id: Id,
    pub start_pos: Position,
    pub current_pos: Position,
}

impl DragState {
    pub fn begin(node_id: Id, start_pos: Position) -> Self {
        DragState {
            node_id,
            start_pos,
            current_pos: start_pos,
        }
    }

    pub fn update(&mut self, pos: Position) {
        self.current_pos = pos;
    }

    pub fn delta(&self) -> (f64, f64) {
        (
            self.current_pos.x - self.start_pos.x,
            self.current_pos.y - self.start_pos.y,
        )
    }
}

/// The canvas holds all placed nodes and manages placement/movement.
#[derive(Debug, Default, Clone)]
pub struct Canvas {
    nodes: HashMap<Id, CanvasNode>,
    next_id: u64,
    active_drag: Option<DragState>,
}

impl Canvas {
    pub fn new() -> Self {
        Canvas::default()
    }

    fn gen_id(&mut self) -> Id {
        let id = Id::new(format!("node_{}", self.next_id));
        self.next_id += 1;
        id
    }

    /// Place a new node from the palette onto the canvas.
    pub fn place_node(
        &mut self,
        kind: impl Into<String>,
        label: impl Into<String>,
        position: Position,
    ) -> Id {
        let id = self.gen_id();
        let node = CanvasNode::new(id.clone(), kind, label, position);
        self.nodes.insert(id.clone(), node);
        id
    }

    /// Place a node with a pre-assigned ID (e.g. during import).
    pub fn place_node_with_id(&mut self, node: CanvasNode) -> Result<(), PlacementError> {
        if self.nodes.contains_key(&node.id) {
            return Err(PlacementError::DuplicateId(node.id.0.clone()));
        }
        self.nodes.insert(node.id.clone(), node);
        Ok(())
    }

    pub fn get_node(&self, id: &Id) -> Option<&CanvasNode> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: &Id) -> Option<&mut CanvasNode> {
        self.nodes.get_mut(id)
    }

    pub fn remove_node(&mut self, id: &Id) -> Option<CanvasNode> {
        self.nodes.remove(id)
    }

    pub fn all_nodes(&self) -> impl Iterator<Item = &CanvasNode> {
        self.nodes.values()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Start a drag operation.
    pub fn begin_drag(&mut self, node_id: Id, start_pos: Position) -> Result<(), PlacementError> {
        if !self.nodes.contains_key(&node_id) {
            return Err(PlacementError::NodeNotFound(node_id.0));
        }
        self.active_drag = Some(DragState::begin(node_id, start_pos));
        Ok(())
    }

    /// Update the drag position.
    pub fn update_drag(&mut self, pos: Position) {
        if let Some(d) = &mut self.active_drag {
            d.update(pos);
        }
    }

    /// Commit the drag: move the node to the final position.
    pub fn commit_drag(&mut self) -> Result<Id, PlacementError> {
        let drag = self
            .active_drag
            .take()
            .ok_or(PlacementError::NoDragActive)?;
        let node = self
            .nodes
            .get_mut(&drag.node_id)
            .ok_or_else(|| PlacementError::NodeNotFound(drag.node_id.0.clone()))?;
        node.position = drag.current_pos;
        Ok(drag.node_id)
    }

    /// Cancel an active drag without moving the node.
    pub fn cancel_drag(&mut self) {
        self.active_drag = None;
    }

    /// Move a node directly (e.g. keyboard nudge).
    pub fn move_node(&mut self, id: &Id, new_pos: Position) -> Result<(), PlacementError> {
        let node = self
            .nodes
            .get_mut(id)
            .ok_or_else(|| PlacementError::NodeNotFound(id.0.clone()))?;
        node.position = new_pos;
        Ok(())
    }

    /// Snap a position to a grid.
    pub fn snap_to_grid(pos: Position, grid: f64) -> Position {
        if grid <= 0.0 {
            return pos;
        }
        Position::new((pos.x / grid).round() * grid, (pos.y / grid).round() * grid)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PlacementError {
    NodeNotFound(String),
    DuplicateId(String),
    NoDragActive,
}

impl std::fmt::Display for PlacementError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlacementError::NodeNotFound(id) => write!(f, "node not found: {}", id),
            PlacementError::DuplicateId(id) => write!(f, "duplicate node id: {}", id),
            PlacementError::NoDragActive => write!(f, "no drag operation is active"),
        }
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn place_and_retrieve() {
        let mut canvas = Canvas::new();
        let id = canvas.place_node("agent.llm", "My Agent", Position::new(100.0, 50.0));
        let node = canvas.get_node(&id).expect("node should exist");
        assert_eq!(node.kind, "agent.llm");
        assert_eq!(node.position.x, 100.0);
    }

    #[test]
    fn drag_moves_node() {
        let mut canvas = Canvas::new();
        let id = canvas.place_node("tool.web_search", "Search", Position::new(0.0, 0.0));
        canvas
            .begin_drag(id.clone(), Position::new(0.0, 0.0))
            .unwrap();
        canvas.update_drag(Position::new(50.0, 30.0));
        let moved_id = canvas.commit_drag().unwrap();
        assert_eq!(moved_id, id);
        let node = canvas.get_node(&id).unwrap();
        assert_eq!(node.position.x, 50.0);
        assert_eq!(node.position.y, 30.0);
    }

    #[test]
    fn snap_to_grid_works() {
        let p = Position::new(13.0, 27.0);
        let snapped = Canvas::snap_to_grid(p, 10.0);
        assert_eq!(snapped.x, 10.0);
        assert_eq!(snapped.y, 30.0);
    }

    #[test]
    fn duplicate_id_rejected() {
        let mut canvas = Canvas::new();
        let node = CanvasNode::new(Id::new("dup"), "agent.llm", "A", Position::origin());
        canvas.place_node_with_id(node.clone()).unwrap();
        let err = canvas.place_node_with_id(node).unwrap_err();
        assert_eq!(err, PlacementError::DuplicateId("dup".into()));
    }
}
