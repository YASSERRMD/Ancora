/// scaffold - Graph builder frontend scaffold: canvas, viewport, and project state.
use std::collections::HashMap;

/// Unique identifier for nodes and edges in the canvas.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub String);

impl Id {
    pub fn new(s: impl Into<String>) -> Self {
        Id(s.into())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 2-D position on the canvas (in logical pixels).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }
    pub fn origin() -> Self {
        Position { x: 0.0, y: 0.0 }
    }
}

/// Viewport: the visible window into the canvas.
#[derive(Debug, Clone)]
pub struct Viewport {
    pub offset: Position,
    pub zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            offset: Position::origin(),
            zoom: 1.0,
        }
    }
}

impl Viewport {
    /// Pan the viewport by a delta.
    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.offset.x += dx;
        self.offset.y += dy;
    }

    /// Clamp zoom between reasonable bounds.
    pub fn set_zoom(&mut self, zoom: f64) {
        self.zoom = zoom.clamp(0.1, 10.0);
    }

    /// Convert a canvas position to a screen position.
    pub fn canvas_to_screen(&self, p: Position) -> Position {
        Position::new(
            (p.x - self.offset.x) * self.zoom,
            (p.y - self.offset.y) * self.zoom,
        )
    }

    /// Convert a screen position to a canvas position.
    pub fn screen_to_canvas(&self, p: Position) -> Position {
        Position::new(
            p.x / self.zoom + self.offset.x,
            p.y / self.zoom + self.offset.y,
        )
    }
}

/// Selection state: tracks which node/edge IDs are currently selected.
#[derive(Debug, Default, Clone)]
pub struct Selection {
    pub node_ids: Vec<Id>,
    pub edge_ids: Vec<Id>,
}

impl Selection {
    pub fn clear(&mut self) {
        self.node_ids.clear();
        self.edge_ids.clear();
    }

    pub fn select_node(&mut self, id: Id) {
        if !self.node_ids.contains(&id) {
            self.node_ids.push(id);
        }
    }

    pub fn is_node_selected(&self, id: &Id) -> bool {
        self.node_ids.contains(id)
    }
}

/// Project metadata.
#[derive(Debug, Clone)]
pub struct ProjectMeta {
    pub name: String,
    pub description: String,
    pub version: u32,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        ProjectMeta {
            name: "untitled".into(),
            description: String::new(),
            version: 1,
        }
    }
}

/// Top-level builder state holding all canvas data.
#[derive(Debug, Default, Clone)]
pub struct BuilderState {
    pub meta: ProjectMeta,
    pub viewport: Viewport,
    pub selection: Selection,
    /// Undo stack: each entry is a snapshot description (full undo would store diffs).
    pub undo_stack: Vec<String>,
    /// Generic key-value extension metadata.
    pub ext: HashMap<String, String>,
}

impl BuilderState {
    pub fn new(name: impl Into<String>) -> Self {
        BuilderState {
            meta: ProjectMeta {
                name: name.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn push_undo(&mut self, description: impl Into<String>) {
        self.undo_stack.push(description.into());
    }
}

#[cfg(test)]
mod unit {
    use super::*;

    #[test]
    fn viewport_pan_and_zoom() {
        let mut vp = Viewport::default();
        vp.pan(10.0, 20.0);
        assert_eq!(vp.offset.x, 10.0);
        vp.set_zoom(0.001); // below min
        assert!((vp.zoom - 0.1).abs() < 1e-9);
        vp.set_zoom(99.0); // above max
        assert!((vp.zoom - 10.0).abs() < 1e-9);
    }

    #[test]
    fn canvas_screen_round_trip() {
        let mut vp = Viewport::default();
        vp.pan(5.0, 5.0);
        vp.set_zoom(2.0);
        let p = Position::new(10.0, 10.0);
        let screen = vp.canvas_to_screen(p);
        let back = vp.screen_to_canvas(screen);
        assert!((back.x - p.x).abs() < 1e-9);
        assert!((back.y - p.y).abs() < 1e-9);
    }

    #[test]
    fn selection_deduplicates() {
        let mut sel = Selection::default();
        sel.select_node(Id::new("a"));
        sel.select_node(Id::new("a"));
        assert_eq!(sel.node_ids.len(), 1);
    }
}
