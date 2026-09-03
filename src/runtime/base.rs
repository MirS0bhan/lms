/// Base Runtime Trait
/// 
/// Defines the interface that all runtime implementations must follow.

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use crate::error::Result;
use super::ExecutionResult;

/// Runtime status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeStatus {
    Healthy,
    Degraded,
    Unhealthy,
}

/// Runtime information
#[derive(Debug, Clone)]
pub struct RuntimeInfo {
    pub name: String,
    pub version: String,
    pub status: RuntimeStatus,
    pub supports_ipc: bool,
}

#[async_trait]
pub trait Runtime: Send + Sync {
    /// Get runtime name (e.g., "python3.11", "rust")
    fn name(&self) -> &str;

    /// Get runtime version
    fn version(&self) -> &str;

    /// Get runtime information
    fn info(&self) -> RuntimeInfo {
        RuntimeInfo {
            name: self.name().to_string(),
            version: self.version().to_string(),
            status: RuntimeStatus::Healthy,
            supports_ipc: false,
        }
    }

    /// Execute a function with given arguments
    async fn execute(
        &self,
        function_id: &str,
        code: &str,
        args: HashMap<String, Value>,
    ) -> Result<ExecutionResult>;

    /// Validate function code before registration
    fn validate_code(&self, code: &str) -> Result<()>;

    /// Check if runtime is available/healthy
    async fn health_check(&self) -> Result<RuntimeInfo>;

    /// Get runtime capabilities
    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities::default()
    }
}

/// Runtime capabilities description
#[derive(Debug, Clone)]
pub struct RuntimeCapabilities {
    pub max_memory_mb: u64,
    pub supports_async: bool,
    pub supports_networking: bool,
    pub supports_file_io: bool,
    pub supports_subprocess: bool,
    pub supports_ipc: bool,
    pub timeout_seconds: u64,
}

impl Default for RuntimeCapabilities {
    fn default() -> Self {
        Self {
            max_memory_mb: 512,
            supports_async: true,
            supports_networking: false,
            supports_file_io: false,
            supports_subprocess: false,
            supports_ipc: false,
            timeout_seconds: 30,
        }
    }
}
