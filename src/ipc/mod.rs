/// IPC Module
/// 
/// Handles inter-process communication for function-to-function calls
/// Uses message passing to route function calls across runtimes
/// 
/// This phase defines the protocol; Phase 4 implements routing.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcMessage {
    pub id: String,
    pub caller_function_id: String,
    pub target_function_id: String,
    pub method: String,
    pub arguments: serde_json::Value,
    pub timestamp: String,
}

impl IpcMessage {
    pub fn new(
        caller_function_id: String,
        target_function_id: String,
        method: String,
        arguments: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            caller_function_id,
            target_function_id,
            method,
            arguments,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcResponse {
    pub message_id: String,
    pub result: serde_json::Value,
    pub error: Option<String>,
    pub timestamp: String,
}

impl IpcResponse {
    pub fn success(message_id: String, result: serde_json::Value) -> Self {
        Self {
            message_id,
            result,
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(message_id: String, error: String) -> Self {
        Self {
            message_id,
            result: serde_json::json!(null),
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}

pub trait IpcRouter: Send + Sync {
    /// Route an IPC message to target function
    fn route(&self, message: IpcMessage) -> std::result::Result<IpcResponse, String>;
    
    /// Check if target function is accessible from caller
    fn can_access(&self, caller: &str, target: &str) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipc_message_creation() {
        let msg = IpcMessage::new(
            "fn1".to_string(),
            "fn2".to_string(),
            "execute".to_string(),
            serde_json::json!({"x": 1}),
        );
        
        assert_eq!(msg.caller_function_id, "fn1");
        assert_eq!(msg.target_function_id, "fn2");
        assert!(!msg.id.is_empty());
    }

    #[test]
    fn test_ipc_response_success() {
        let resp = IpcResponse::success(
            "msg_123".to_string(),
            serde_json::json!({"result": 42}),
        );
        
        assert_eq!(resp.message_id, "msg_123");
        assert!(resp.error.is_none());
    }
}
