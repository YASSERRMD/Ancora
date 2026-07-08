use serde::{Deserialize, Serialize};

/// Lifecycle state of an A2A task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Queued,
    Running,
    Completed,
    Failed,
    Cancelled,
}

/// An A2A task managed by the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub status: TaskStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl Task {
    pub fn new(id: impl Into<String>, input: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: TaskStatus::Queued,
            input: Some(input.into()),
            output: None,
            error: None,
        }
    }

    pub fn running(mut self) -> Self {
        self.status = TaskStatus::Running;
        self
    }

    pub fn completed(mut self, output: impl Into<String>) -> Self {
        self.status = TaskStatus::Completed;
        self.output = Some(output.into());
        self
    }

    pub fn failed(mut self, error: impl Into<String>) -> Self {
        self.status = TaskStatus::Failed;
        self.error = Some(error.into());
        self
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_task_is_queued() {
        let t = Task::new("t1", "hello");
        assert_eq!(t.status, TaskStatus::Queued);
        assert_eq!(t.id, "t1");
        assert_eq!(t.input.as_deref(), Some("hello"));
        assert!(t.output.is_none());
        assert!(t.error.is_none());
    }

    #[test]
    fn task_lifecycle_transitions() {
        let t = Task::new("t2", "input").running().completed("result");
        assert_eq!(t.status, TaskStatus::Completed);
        assert_eq!(t.output.as_deref(), Some("result"));
        assert!(t.is_terminal());
    }

    #[test]
    fn failed_task_is_terminal() {
        let t = Task::new("t3", "inp").failed("oops");
        assert_eq!(t.status, TaskStatus::Failed);
        assert_eq!(t.error.as_deref(), Some("oops"));
        assert!(t.is_terminal());
    }

    #[test]
    fn queued_task_is_not_terminal() {
        let t = Task::new("t4", "inp");
        assert!(!t.is_terminal());
    }

    #[test]
    fn running_task_is_not_terminal() {
        let t = Task::new("t5", "inp").running();
        assert!(!t.is_terminal());
    }

    #[test]
    fn cancelled_is_terminal() {
        let t = Task {
            id: "t6".into(),
            status: TaskStatus::Cancelled,
            input: None,
            output: None,
            error: None,
        };
        assert!(t.is_terminal());
    }

    #[test]
    fn task_status_serialises_snake_case() {
        let json = serde_json::to_string(&TaskStatus::Running).unwrap();
        assert_eq!(json, "\"running\"");
        let json = serde_json::to_string(&TaskStatus::Completed).unwrap();
        assert_eq!(json, "\"completed\"");
    }

    #[test]
    fn task_serialises_without_none_fields() {
        let t = Task::new("t7", "data");
        let json = serde_json::to_string(&t).unwrap();
        assert!(!json.contains("output"));
        assert!(!json.contains("error"));
        assert!(json.contains("queued"));
    }
}
