/// Runtime Executor
/// 
/// High-level interface for executing functions through runtimes.

use std::collections::HashMap;
use std::sync::Arc;
use serde_json::Value;
use tracing::{info, error, debug};
use crate::error::{LmsError, Result};
use super::manager::RuntimeManager;
use super::context::ExecutionContext;
use super::ExecutionResult;

pub struct RuntimeExecutor {
    manager: Arc<RuntimeManager>,
}

impl RuntimeExecutor {
    pub fn new(manager: Arc<RuntimeManager>) -> Self {
        Self { manager }
    }

    /// Execute a function with the specified runtime
    pub async fn execute(
        &self,
        runtime_name: &str,
        function_id: &str,
        code: &str,
        args: HashMap<String, Value>,
    ) -> Result<ExecutionResult> {
        debug!(
            "RuntimeExecutor: Executing {} with runtime {}",
            function_id, runtime_name
        );

        let runtime = self.manager.get_runtime(runtime_name)?;

        // Create execution context
        let ctx = ExecutionContext::new(
            function_id.to_string(),
            runtime_name.to_string(),
            args.clone(),
        );

        info!("Execution context created: {}", ctx.execution_id);

        // Execute function
        let result = runtime.execute(function_id, code, args).await?;

        if ctx.is_timed_out() {
            return Err(LmsError::Timeout(
                format!("Function {} execution exceeded timeout", function_id)
            ));
        }

        info!(
            "Function {} completed in {}ms via {}",
            function_id, result.execution_time_ms, runtime_name
        );

        Ok(result)
    }

    /// Validate code for a runtime
    pub fn validate_code(&self, runtime_name: &str, code: &str) -> Result<()> {
        debug!("Validating code for runtime: {}", runtime_name);
        
        let runtime = self.manager.get_runtime(runtime_name)?;
        runtime.validate_code(code)
    }

    /// Get available runtimes
    pub fn available_runtimes(&self) -> Vec<String> {
        self.manager.runtime_names()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::base::Runtime;
    use async_trait::async_trait;
    use crate::runtime::base::RuntimeInfo;

    struct MockRuntime;

    #[async_trait]
    impl Runtime for MockRuntime {
        fn name(&self) -> &str {
            "mock"
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        async fn execute(
            &self,
            _function_id: &str,
            _code: &str,
            _args: HashMap<String, Value>,
        ) -> Result<ExecutionResult> {
            Ok(ExecutionResult::success(Value::Null, 0))
        }

        fn validate_code(&self, _code: &str) -> Result<()> {
            Ok(())
        }

        async fn health_check(&self) -> Result<RuntimeInfo> {
            Ok(RuntimeInfo {
                name: "mock".to_string(),
                version: "1.0.0".to_string(),
                status: crate::runtime::base::RuntimeStatus::Healthy,
                supports_ipc: false,
            })
        }
    }

    #[tokio::test]
    async fn test_executor_creation() {
        let manager = RuntimeManager::new();
        let executor = RuntimeExecutor::new(manager);
        assert_eq!(executor.available_runtimes().len(), 0);
    }
}
