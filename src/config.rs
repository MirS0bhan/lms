use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use crate::error::{LmsError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub sandbox: SandboxConfig,
    pub runtime: RuntimeConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub worker_threads: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub max_memory_mb: u64,
    pub max_cpu_time_secs: u64,
    pub max_file_descriptors: u32,
    pub enable_seccomp: bool,
    pub enable_namespaces: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub pluggable_runtimes: Vec<String>,
    pub max_concurrent_executions: usize,
    pub execution_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                worker_threads: num_cpus::get(),
            },
            sandbox: SandboxConfig {
                max_memory_mb: 512,
                max_cpu_time_secs: 30,
                max_file_descriptors: 1024,
                enable_seccomp: true,
                enable_namespaces: true,
            },
            runtime: RuntimeConfig {
                pluggable_runtimes: vec![
                    "python".to_string(),
                    "rust".to_string(),
                ],
                max_concurrent_executions: 100,
                execution_timeout_secs: 30,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                format: "json".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        // Try to load from environment or use defaults
        dotenvy::dotenv().ok();
        
        match config::Config::builder()
            .add_source(config::File::with_name("config").required(false))
            .add_source(config::environment::Environment::default())
            .build()
        {
            Ok(cfg) => cfg
                .try_deserialize::<Config>()
                .map_err(|e| LmsError::ConfigError(e.to_string())),
            Err(_) => Ok(Config::default()),
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.server.host, self.server.port)
            .parse()
            .expect("Invalid socket address")
    }
}
