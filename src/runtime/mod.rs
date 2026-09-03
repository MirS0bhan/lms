/// Runtime Module
/// 
/// Provides abstraction for pluggable runtimes (Python, Rust, Perl, Ruby, etc.)
/// Implementations for Python and Rust are included in Phase 2.

pub mod base;
pub mod manager;
pub mod python;
pub mod rust;
pub mod context;
pub mod executor;

pub use base::Runtime;
pub use manager::RuntimeManager;
pub use python::PythonRuntime;
pub use rust::RustRuntime;
pub use context::ExecutionContext;
pub use executor::RuntimeExecutor;

use serde_json::Value;
use std::collections::HashMap;
use crate::error::Result;

/// Runtime execution result
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub result: Value,
    pub execution_time_ms: u128,
    pub memory_used_bytes: u64,
}

impl ExecutionResult {
    pub fn success(result: Value, execution_time_ms: u128) -> Self {
        Self {
            stdout: String::new(),
            stderr: String::new(),
            result,
            execution_time_ms,
            memory_used_bytes: 0,
        }
    }

    pub fn with_output(mut self, stdout: String, stderr: String) -> Self {
        self.stdout = stdout;
        self.stderr = stderr;
        self
    }
}
