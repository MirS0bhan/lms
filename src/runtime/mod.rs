/// Runtime Module
/// 
/// Provides abstraction for pluggable runtimes (Python, Rust, Perl, Ruby, etc.)
/// This phase defines the trait and basic structure; implementations come in Phase 2.

use async_trait::async_trait;
use serde_json::Value;
use crate::error::Result;
use std::collections::HashMap;

#[async_trait]
pub trait Runtime: Send + Sync {
    /// Get runtime name (e.g., "python", "rust")
    fn name(&self) -> &str;

    /// Get runtime version
    fn version(&self) -> &str;

    /// Execute a function with given arguments
    async fn execute(
        &self,
        function_id: &str,
        code: &str,
        args: HashMap<String, Value>,
    ) -> Result<Value>;

    /// Validate function code before registration
    fn validate_code(&self, code: &str) -> Result<()>;

    /// Check if runtime is available/healthy
    async fn health_check(&self) -> Result<()>;
}

pub struct RuntimeManager {
    runtimes: HashMap<String, Box<dyn Runtime>>,
}

impl RuntimeManager {
    pub fn new() -> Self {
        Self {
            runtimes: HashMap::new(),
        }
    }

    pub fn register_runtime(&mut self, runtime: Box<dyn Runtime>) {
        self.runtimes.insert(runtime.name().to_string(), runtime);
    }

    pub fn get_runtime(&self, name: &str) -> Option<&dyn Runtime> {
        self.runtimes.get(name).map(|r| r.as_ref())
    }

    pub fn list_runtimes(&self) -> Vec<&str> {
        self.runtimes.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new()
    }
}
