use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskState {
    Pending,
    Running,
    Completed,
    Failed,
}

pub struct TaskNode {
    pub task_id: String,
    pub state: TaskState,
    pub deps: Vec<String>,
}

pub struct TaskGraph {
    nodes: HashMap<String, TaskNode>,
}

impl TaskGraph {
    pub fn new() -> Self {
        Self { nodes: HashMap::new() }
    }

    pub fn add_task(&mut self, task_id: &str, deps: Vec<String>) {
        self.nodes.insert(task_id.to_string(), TaskNode {
            task_id: task_id.to_string(),
            state: TaskState::Pending,
            deps,
        });
    }

    pub fn ready_tasks(&self) -> Vec<&str> {
        self.nodes
            .iter()
            .filter(|(_, n)| {
                n.state == TaskState::Pending
                    && n.deps.iter().all(|dep| {
                        self.nodes.get(dep).map(|d| d.state == TaskState::Completed).unwrap_or(false)
                    })
            })
            .map(|(id, _)| id.as_str())
            .collect()
    }

    pub fn mark_running(&mut self, task_id: &str) {
        if let Some(n) = self.nodes.get_mut(task_id) {
            n.state = TaskState::Running;
        }
    }

    pub fn mark_completed(&mut self, task_id: &str) {
        if let Some(n) = self.nodes.get_mut(task_id) {
            n.state = TaskState::Completed;
        }
    }

    pub fn mark_failed(&mut self, task_id: &str) {
        if let Some(n) = self.nodes.get_mut(task_id) {
            n.state = TaskState::Failed;
        }
    }

    pub fn all_complete(&self) -> bool {
        self.nodes.values().all(|n| n.state == TaskState::Completed)
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();
        for id in self.nodes.keys() {
            if self.dfs_cycle(id, &mut visited, &mut stack) {
                return true;
            }
        }
        false
    }

    fn dfs_cycle(&self, id: &str, visited: &mut HashSet<String>, stack: &mut HashSet<String>) -> bool {
        if stack.contains(id) { return true; }
        if visited.contains(id) { return false; }
        visited.insert(id.to_string());
        stack.insert(id.to_string());
        if let Some(node) = self.nodes.get(id) {
            for dep in &node.deps {
                if self.dfs_cycle(dep, visited, stack) { return true; }
            }
        }
        stack.remove(id);
        false
    }
}

impl Default for TaskGraph {
    fn default() -> Self {
        Self::new()
    }
}
