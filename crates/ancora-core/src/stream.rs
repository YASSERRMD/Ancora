/// A single event emitted on the live stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StreamEvent {
    /// A model token fragment arrived.
    Token { text: String },
    /// A graph node began executing.
    NodeEntered { node_id: String, node_kind: String },
    /// A graph node finished executing.
    NodeExited { node_id: String },
    /// The run reached the final node and returned its output.
    RunCompleted { output: String },
}

/// Sender half of a `StreamEvent` channel.
pub type StreamSender = std::sync::mpsc::Sender<StreamEvent>;

/// Receiver half of a `StreamEvent` channel.
pub type StreamReceiver = std::sync::mpsc::Receiver<StreamEvent>;

/// Create a new `(sender, receiver)` pair for streaming events out of a run.
pub fn open_stream() -> (StreamSender, StreamReceiver) {
    std::sync::mpsc::channel()
}
