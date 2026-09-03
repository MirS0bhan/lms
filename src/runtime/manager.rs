/// Runtime Manager
/// 
/// Manages multiple runtime implementations and provides runtime selection.

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use tracing::{info, warn, debug};
use crate::error::{LmsError, Result};
use super::base::{Runtime, RuntimeInfo};

pub struct RuntimeManager {
    runtimes: RwLock<HashMap<String, Arc<dyn Runtime>>>,
}

impl RuntimeManager {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            runtimes: RwLock::new(HashMap::new()),
        })
    }

    /// Register a new runtime
    pub fn register_runtime(&self, runtime: Arc<dyn Runtime>) -> Result<()> {
        let name = runtime.name().to_string();
        info!("Registering runtime: {} v{}", name, runtime.version());
        
        let mut runtimes = self.runtimes.write();
        if runtimes.contains_key(&name) {
            warn!("Runtime {} already registered, replacing", name);
        }
        
        runtimes.insert(name, runtime);
        Ok(())
    }

    /// Get a runtime by name
    pub fn get_runtime(&self, name: &str) -> Result<Arc<dyn Runtime>> {
        let runtimes = self.runtimes.read();
        runtimes
            .get(name)
            .cloned()
            .ok_or_else(|| LmsError::RuntimeNotFound(name.to_string()))
    }

    /// List all registered runtimes
    pub fn list_runtimes(&self) -> Vec<RuntimeInfo> {
        let runtimes = self.runtimes.read();
        runtimes
            .values()
            .map(|rt| rt.info())
            .collect()
    }

    /// Get all runtime names
    pub fn runtime_names(&self) -> Vec<String> {
        let runtimes = self.runtimes.read();
        runtimes.keys().cloned().collect()
    }

    /// Check health of all runtimes
    pub async fn health_check_all(&self) -> HashMap<String, Result<RuntimeInfo>> {
        let runtimes = self.runtimes.read();
        let mut results = HashMap::new();

        for (name, runtime) in runtimes.iter() {
            debug!("Health checking runtime: {}", name);
            let result = runtime.health_check().await;
            results.insert(name.clone(), result);
        }

        results
    }

    /// Get count of registered runtimes
    pub fn runtime_count(&self) -> usize {
        self.runtimes.read().len()
    }
}

impl Default for RuntimeManager {
    fn default() -> Self {
        Self::new().as_ref().clone()
    }
}

impl Clone for RuntimeManager {
    fn clone(&self) -> Self {
        Self {
            runtimes: RwLock::new(self.runtimes.read().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use serde_json::Value;
    use std::collections::HashMap;

    struct MockRuntime {
        name: String,
    }

    #[async_trait]
    impl Runtime for MockRuntime {
        fn name(&self) -> &str {
            &self.name
        }

        fn version(&self) -> &str {
            "1.0.0"
        }

        async fn execute(
            &self,
            _function_id: &str,
            _code: &str,
            _args: HashMap<String, Value>,
        ) -> Result<super::super::ExecutionResult> {
            Ok(super::super::ExecutionResult::success(
                Value::Null,
                0,
            ))
        }

        fn validate_code(&self, _code: &str) -> Result<()> {
            Ok(())
        }

        async fn health_check(&self) -> Result<RuntimeInfo> {
            Ok(RuntimeInfo {
                name: self.name.clone(),
                version: "1.0.0".to_string(),
                status: super::super::base::RuntimeStatus::Healthy,
                supports_ipc: false,
            })
        }
    }

    #[tokio::test]
    async fn test_register_runtime() {
        let manager = RuntimeManager::new();
        let runtime = Arc::new(MockRuntime {
            name: "mock".to_string(),
        });

        assert!(manager.register_runtime(runtime).is_ok());
        assert_eq!(manager.runtime_count(), 1);
    }

    #[tokio::test]
    async fn test_get_runtime() {
        let manager = RuntimeManager::new();
        let runtime = Arc::new(MockRuntime {
            name: "mock".to_string(),
        });

        manager.register_runtime(runtime).unwrap();
        assert!(manager.get_runtime("mock").is_ok());
        assert!(manager.get_runtime("nonexistent").is_err());
    }

    #[tokio::test]
    async fn test_list_runtimes() {
        let manager = RuntimeManager::new();
        let runtime1 = Arc::new(MockRuntime {
            name: "mock1".to_string(),
        });
        let runtime2 = Arc::new(MockRuntime {
            name: "mock2".to_string(),
        });

        manager.register_runtime(runtime1).unwrap();
        manager.register_runtime(runtime2).unwrap();

        let runtimes = manager.list_runtimes();
        assert_eq!(runtimes.len(), 2);
    }
}
