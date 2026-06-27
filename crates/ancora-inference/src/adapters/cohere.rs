use serde::{Deserialize, Serialize};

use crate::types::Message;

// ---- Wire types: chat history ----------------------------------------------

/// A single turn in the Cohere chat history.
///
/// Cohere uses "USER" and "CHATBOT" (uppercase) unlike OpenAI's "user"/"assistant".
#[derive(Debug, Serialize, Clone)]
pub(crate) struct CohereChatTurn {
    pub role: String,
    pub message: String,
}

/// Convert a message role to Cohere's uppercase role label.
///
/// Cohere: USER for all human turns, CHATBOT for all assistant turns.
pub(crate) fn cohere_role(role: &str) -> &'static str {
    match role {
        "assistant" | "CHATBOT" | "chatbot" => "CHATBOT",
        _ => "USER",
    }
}

/// Build the `chat_history` array from all non-system, non-last messages.
///
/// Cohere's API separates the current user message (goes as `message`)
/// from the prior conversation (goes as `chat_history`). System messages
/// are extracted separately as `preamble`.
pub(crate) fn build_chat_history(messages: &[&Message]) -> Vec<CohereChatTurn> {
    // All messages except the last one go into chat_history.
    messages.iter().take(messages.len().saturating_sub(1)).map(|m| CohereChatTurn {
        role: cohere_role(&m.role).to_owned(),
        message: m.content.clone(),
    }).collect()
}

/// Extract the current message (last non-system turn) and chat history.
///
/// Returns `(current_message, chat_history)`. If there are no non-system
/// messages, current_message is an empty string.
pub(crate) fn split_messages(messages: &[Message]) -> (String, Vec<CohereChatTurn>) {
    let non_system: Vec<&Message> = messages.iter().filter(|m| m.role != "system").collect();
    let current = non_system.last().map(|m| m.content.clone()).unwrap_or_default();
    let history = build_chat_history(&non_system);
    (current, history)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Message;

    #[test]
    fn cohere_role_user_maps_to_user() {
        assert_eq!(cohere_role("user"), "USER");
    }

    #[test]
    fn cohere_role_assistant_maps_to_chatbot() {
        assert_eq!(cohere_role("assistant"), "CHATBOT");
    }

    #[test]
    fn split_messages_current_is_last_user_message() {
        let msgs = vec![
            Message::text("user", "Hello"),
            Message::text("assistant", "Hi there"),
            Message::text("user", "What is the weather?"),
        ];
        let (current, history) = split_messages(&msgs);
        assert_eq!(current, "What is the weather?");
        assert_eq!(history.len(), 2);
    }

    #[test]
    fn split_messages_history_has_correct_roles() {
        let msgs = vec![
            Message::text("user", "Hello"),
            Message::text("assistant", "Hi"),
            Message::text("user", "Bye"),
        ];
        let (_, history) = split_messages(&msgs);
        assert_eq!(history[0].role, "USER");
        assert_eq!(history[1].role, "CHATBOT");
    }

    #[test]
    fn split_messages_single_message_empty_history() {
        let msgs = vec![Message::text("user", "First")];
        let (current, history) = split_messages(&msgs);
        assert_eq!(current, "First");
        assert!(history.is_empty());
    }

    #[test]
    fn split_messages_system_excluded_from_chat_history() {
        let msgs = vec![
            Message::text("system", "Be helpful"),
            Message::text("user", "Hi"),
        ];
        let (current, history) = split_messages(&msgs);
        assert_eq!(current, "Hi");
        assert!(history.is_empty());
    }
}
