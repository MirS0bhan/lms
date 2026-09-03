use thiserror::Error;

#[derive(Error, Debug)]
pub enum LmsError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Registry error: {0}")]
    RegistryError(String),

    #[error("Function not found: {0}")]
    FunctionNotFound(String),

    #[error("Function already exists: {0}")]
    FunctionAlreadyExists(String),

    #[error("Runtime error: {0}")]
    RuntimeError(String),

    #[error("Runtime not found: {0}")]
    RuntimeNotFound(String),

    #[error("Execution error: {0}")]
    ExecutionError(String),

    #[error("Sandbox error: {0}")]
    SandboxError(String),

    #[error("IPC error: {0}")]
    IpcError(String),

    #[error("Invalid artifact: {0}")]
    InvalidArtifact(String),

    #[error("MCP protocol error: {0}")]
    McpError(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Resource exhausted: {0}")]
    ResourceExhausted(String),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, LmsError>;
