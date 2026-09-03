use crate::error::Result;
use crate::registry::FunctionRegistry;
use super::protocol::McpMessage;
use std::sync::Arc;

/// Handles MCP messages from clients
pub struct McpHandler {
    registry: Arc<FunctionRegistry>,
}

impl McpHandler {
    pub fn new(registry: Arc<FunctionRegistry>) -> Self {
        Self { registry }
    }

    pub async fn handle_message(&self, message: McpMessage) -> Result<McpMessage> {
        match message.message_type {
            super::protocol::MessageType::RegisterFunction => {
                self.handle_register_function(message).await
            }
            super::protocol::MessageType::InvokeFunction => {
                self.handle_invoke_function(message).await
            }
            super::protocol::MessageType::RevokeFunction => {
                self.handle_revoke_function(message).await
            }
            super::protocol::MessageType::ListFunctions => {
                self.handle_list_functions(message).await
            }
            super::protocol::MessageType::GetFunctionInfo => {
                self.handle_get_function_info(message).await
            }
            _ => {
                let err_msg = "Unsupported message type";
                Err(crate::error::LmsError::McpError(err_msg.to_string()))
            }
        }
    }

    async fn handle_register_function(&self, _message: McpMessage) -> Result<McpMessage> {
        // TODO: Phase 2 - Implement function registration
        Ok(McpMessage::error("Not implemented in Phase 1"))
    }

    async fn handle_invoke_function(&self, _message: McpMessage) -> Result<McpMessage> {
        // TODO: Phase 2 - Implement function invocation
        Ok(McpMessage::error("Not implemented in Phase 1"))
    }

    async fn handle_revoke_function(&self, _message: McpMessage) -> Result<McpMessage> {
        // TODO: Phase 2 - Implement function revocation
        Ok(McpMessage::error("Not implemented in Phase 1"))
    }

    async fn handle_list_functions(&self, _message: McpMessage) -> Result<McpMessage> {
        // TODO: Phase 2 - Implement list functions
        Ok(McpMessage::error("Not implemented in Phase 1"))
    }

    async fn handle_get_function_info(&self, _message: McpMessage) -> Result<McpMessage> {
        // TODO: Phase 2 - Implement get function info
        Ok(McpMessage::error("Not implemented in Phase 1"))
    }
}
