use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    RegisterFunction,
    InvokeFunction,
    RevokeFunction,
    ListFunctions,
    GetFunctionInfo,
    CallResult,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpMessage {
    pub id: String,
    pub message_type: MessageType,
    pub timestamp: String,
    pub payload: serde_json::Value,
}

impl McpMessage {
    pub fn new(message_type: MessageType, payload: serde_json::Value) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            message_type,
            timestamp: chrono::Utc::now().to_rfc3339(),
            payload,
        }
    }

    pub fn register_function(payload: serde_json::Value) -> Self {
        Self::new(MessageType::RegisterFunction, payload)
    }

    pub fn invoke_function(payload: serde_json::Value) -> Self {
        Self::new(MessageType::InvokeFunction, payload)
    }

    pub fn revoke_function(payload: serde_json::Value) -> Self {
        Self::new(MessageType::RevokeFunction, payload)
    }

    pub fn error(message: &str) -> Self {
        let payload = serde_json::json!({
            "error": message,
        });
        Self::new(MessageType::Error, payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcp_message_creation() {
        let payload = serde_json::json!({
            "function_name": "test_fn",
            "args": {}
        });
        let msg = McpMessage::register_function(payload);
        
        assert_eq!(msg.message_type, MessageType::RegisterFunction);
        assert!(!msg.id.is_empty());
    }
}
