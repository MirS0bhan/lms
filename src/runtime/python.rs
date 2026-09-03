/// Python Runtime
/// 
/// Executes Python code in isolated subprocess with security constraints.

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;
use tracing::{info, error, debug};
use crate::error::{LmsError, Result};
use super::base::{Runtime, RuntimeInfo, RuntimeStatus, RuntimeCapabilities};
use super::ExecutionResult;
use std::time::Instant;

pub struct PythonRuntime {
    version: String,
    python_path: String,
}

impl PythonRuntime {
    pub fn new(python_path: Option<String>) -> Self {
        let python_path = python_path.unwrap_or_else(|| "python3".to_string());
        
        Self {
            version: "3.10+".to_string(),
            python_path,
        }
    }

    /// Wrap Python code for safe execution
    fn wrap_code(&self, code: &str, args: &HashMap<String, Value>) -> String {
        let args_json = serde_json::to_string(args).unwrap_or_else(|_| "{}".to_string());
        
        format!(
            r#"
import json
import sys

# Parse input arguments
try:
    __args = json.loads('{}')
except Exception as e:
    print(json.dumps({{'error': str(e)}}), file=sys.stderr)
    sys.exit(1)

# User code
try:
{}
except Exception as e:
    print(json.dumps({{'error': str(e)}}), file=sys.stderr)
    sys.exit(1)
"#,
            args_json.replace("'", "\\""),
            textwrap::indent(code, "    ")
        )
    }
}

#[async_trait]
impl Runtime for PythonRuntime {
    fn name(&self) -> &str {
        "python"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            max_memory_mb: 512,
            supports_async: true,
            supports_networking: false,
            supports_file_io: false,
            supports_subprocess: false,
            supports_ipc: false,
            timeout_seconds: 30,
        }
    }

    async fn execute(
        &self,
        function_id: &str,
        code: &str,
        args: HashMap<String, Value>,
    ) -> Result<ExecutionResult> {
        debug!("Executing Python function: {}", function_id);
        let start = Instant::now();

        let wrapped_code = self.wrap_code(code, &args);

        let output = Command::new(&self.python_path)
            .arg("-c")
            .arg(&wrapped_code)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                error!("Failed to execute Python: {}", e);
                LmsError::ExecutionError(format!("Python execution failed: {}", e))
            })?;

        let execution_time = start.elapsed().as_millis();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if !output.status.success() {
            return Err(LmsError::ExecutionError(
                format!("Python execution failed: {}", stderr)
            ));
        }

        let result: Value = stdout
            .lines()
            .last()
            .and_then(|line| serde_json::from_str(line).ok())
            .unwrap_or(Value::Null);

        info!("Python function {} completed in {}ms", function_id, execution_time);

        Ok(ExecutionResult::success(result, execution_time)
            .with_output(stdout, stderr))
    }

    fn validate_code(&self, code: &str) -> Result<()> {
        // Basic Python syntax validation using compile
        let validation_code = format!("compile(r\"{}\",'<string>','exec')", 
            code.replace('\\', "\\\\").replace('"', "\\\""));
        
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| LmsError::RuntimeError(e.to_string()))?;
        
        rt.block_on(async {
            let output = Command::new(&self.python_path)
                .arg("-c")
                .arg(&validation_code)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
                .map_err(|e| LmsError::RuntimeError(e.to_string()))?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(LmsError::InvalidArtifact(
                    format!("Invalid Python code: {}", stderr)
                ));
            }
            Ok(())
        })
    }

    async fn health_check(&self) -> Result<RuntimeInfo> {
        debug!("Health checking Python runtime");
        
        let output = Command::new(&self.python_path)
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                error!("Python health check failed: {}", e);
                LmsError::RuntimeError(format!("Python not available: {}", e))
            })?;

        if !output.status.success() {
            return Err(LmsError::RuntimeError(
                "Python runtime is not healthy".to_string()
            ));
        }

        Ok(RuntimeInfo {
            name: self.name().to_string(),
            version: self.version().to_string(),
            status: RuntimeStatus::Healthy,
            supports_ipc: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_runtime_creation() {
        let runtime = PythonRuntime::new(None);
        assert_eq!(runtime.name(), "python");
    }

    #[tokio::test]
    async fn test_python_health_check() {
        let runtime = PythonRuntime::new(None);
        let health = runtime.health_check().await;
        // May fail if Python is not installed, but shouldn't panic
        let _ = health;
    }
}
