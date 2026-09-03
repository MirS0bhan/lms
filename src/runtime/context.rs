/// Execution Context
/// 
/// Encapsulates the runtime context for function execution.

use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use uuid::Uuid;

/// Execution context for a function call
pub struct ExecutionContext {
    pub execution_id: String,
    pub function_id: String,
    pub runtime_name: String,
    pub arguments: HashMap<String, Value>,
    pub start_time: Instant,
    pub timeout: Duration,
    pub environment: HashMap<String, String>,
}

impl ExecutionContext {
    pub fn new(
        function_id: String,
        runtime_name: String,
        arguments: HashMap<String, Value>,
    ) -> Self {
        Self {
            execution_id: Uuid::new_v4().to_string(),
            function_id,
            runtime_name,
            arguments,
            start_time: Instant::now(),
            timeout: Duration::from_secs(30),
            environment: HashMap::new(),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_environment(mut self, env: HashMap<String, String>) -> Self {
        self.environment = env;
        self
    }

    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    pub fn is_timed_out(&self) -> bool {
        self.elapsed() > self.timeout
    }

    pub fn remaining_time(&self) -> Duration {
        self.timeout.saturating_sub(self.elapsed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_execution_context_creation() {
        let ctx = ExecutionContext::new(
            "fn1".to_string(),
            "python".to_string(),
            HashMap::new(),
        );

        assert_eq!(ctx.function_id, "fn1");
        assert_eq!(ctx.runtime_name, "python");
        assert!(!ctx.is_timed_out());
    }

    #[test]
    fn test_execution_timeout() {
        let ctx = ExecutionContext::new(
            "fn1".to_string(),
            "python".to_string(),
            HashMap::new(),
        )
        .with_timeout(Duration::from_millis(100));

        thread::sleep(Duration::from_millis(150));
        assert!(ctx.is_timed_out());
    }

    #[test]
    fn test_execution_remaining_time() {
        let ctx = ExecutionContext::new(
            "fn1".to_string(),
            "python".to_string(),
            HashMap::new(),
        )
        .with_timeout(Duration::from_secs(1));

        let remaining = ctx.remaining_time();
        assert!(remaining.as_secs() <= 1);
    }
}
