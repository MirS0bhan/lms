/// Model Context Protocol (MCP) implementation
/// 
/// This module handles:
/// - MCP message parsing and serialization
/// - Function artifact format
/// - Protocol-level error handling

pub mod protocol;
pub mod artifacts;
pub mod handlers;

pub use protocol::{McpMessage, MessageType};
pub use artifacts::{Artifact, ArtifactType};
