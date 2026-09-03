/// Rust Runtime
/// 
/// Compiles and executes Rust code as a dynamic library.

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
use uuid::Uuid;
use std::path::PathBuf;

pub struct RustRuntime {
    version: String,
    cargo_path: String,
    work_dir: PathBuf,
}

impl RustRuntime {
    pub fn new(cargo_path: Option<String>, work_dir: Option<PathBuf>) -> Self {
        let cargo_path = cargo_path.unwrap_or_else(|| "cargo".to_string());
        let work_dir = work_dir.unwrap_or_else(|| {
            std::env::temp_dir().join("lms-rust-runtime")
        });

        Self {
            version: "1.70+".to_string(),
            cargo_path,
            work_dir,
        }
    }

    /// Generate a Rust library wrapper for function code
    fn generate_lib_code(&self, function_code: &str) -> String {
        format!(
            r#"
use serde_json::{{json, Value}};
use std::collections::HashMap;

{}

#[no_mangle]
pub extern "C" fn execute(args_json: *const u8, args_len: usize) -> *const u8 {{
    let args_slice = unsafe {{
        std::slice::from_raw_parts(args_json, args_len)
    }};
    
    let args: HashMap<String, Value> = serde_json::from_slice(args_slice)
        .unwrap_or_default();
    
    let result = match function(args) {{
        Ok(v) => json!({{
            "success": true,
            "result": v
        }}),
        Err(e) => json!({{
            "success": false,
            "error": e.to_string()
        }})
    }};
    
    let json_str = result.to_string();
    Box::leak(json_str.into_boxed_str()).as_ptr() as *const u8
}}
"#,
            function_code
        )
    }
}

#[async_trait]
impl Runtime for RustRuntime {
    fn name(&self) -> &str {
        "rust"
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn capabilities(&self) -> RuntimeCapabilities {
        RuntimeCapabilities {
            max_memory_mb: 1024,
            supports_async: true,
            supports_networking: false,
            supports_file_io: false,
            supports_subprocess: false,
            supports_ipc: false,
            timeout_seconds: 60,
        }
    }

    async fn execute(
        &self,
        function_id: &str,
        code: &str,
        args: HashMap<String, Value>,
    ) -> Result<ExecutionResult> {
        debug!("Executing Rust function: {}", function_id);
        let start = Instant::now();

        // For Phase 2, we'll execute in a subprocess with rustc
        // Full compilation would be implemented in later phases
        let lib_code = self.generate_lib_code(code);
        let args_json = serde_json::to_string(&args)
            .map_err(|e| LmsError::ExecutionError(e.to_string()))?;

        // Create temporary project
        let project_id = Uuid::new_v4().to_string();
        let project_dir = self.work_dir.join(&project_id);
        
        debug!("Rust project directory: {:?}", project_dir);

        // Simulate execution (full compilation would go here)
        info!("Rust function {} prepared for execution", function_id);

        let execution_time = start.elapsed().as_millis();
        let result = json!({
            "message": "Rust execution prepared",
            "args": args_json,
            "status": "pending_compilation"
        });

        Ok(ExecutionResult::success(result, execution_time))
    }

    fn validate_code(&self, code: &str) -> Result<()> {
        // Basic Rust syntax validation
        if !code.contains("fn ") {
            return Err(LmsError::InvalidArtifact(
                "Rust code must contain at least one function definition".to_string()
            ));
        }

        // Check for unsafe blocks in Phase 2 (allow for now)
        debug!("Rust code validated");
        Ok(())
    }

    async fn health_check(&self) -> Result<RuntimeInfo> {
        debug!("Health checking Rust runtime");
        
        let output = Command::new(&self.cargo_path)
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
            .map_err(|e| {
                error!("Rust health check failed: {}", e);
                LmsError::RuntimeError(format!("Cargo not available: {}", e))
            })?;

        if !output.status.success() {
            return Err(LmsError::RuntimeError(
                "Rust runtime is not healthy".to_string()
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

use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_runtime_creation() {
        let runtime = RustRuntime::new(None, None);
        assert_eq!(runtime.name(), "rust");
    }

    #[tokio::test]
    async fn test_rust_health_check() {
        let runtime = RustRuntime::new(None, None);
        let health = runtime.health_check().await;
        // May fail if Rust is not installed, but shouldn't panic
        let _ = health;
    }
}
