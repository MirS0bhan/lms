use anyhow::Result;
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber;

mod mcp;
mod registry;
mod runtime;
mod server;
mod sandbox;
mod ipc;
mod config;
mod error;

use server::Server;
use config::Config;
use runtime::{RuntimeManager, PythonRuntime, RustRuntime};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_target(true)
        .with_thread_ids(true)
        .init();

    info!("Starting Lambda MCP Server");

    // Load configuration
    let config = Config::load().expect("Failed to load configuration");
    info!("Configuration loaded: {:?}", config);

    // Initialize runtime manager with default runtimes
    let runtime_manager = RuntimeManager::new();
    
    // Register Python runtime
    let python_rt = Arc::new(PythonRuntime::new(None));
    runtime_manager.register_runtime(python_rt)?;
    info!("Python runtime registered");

    // Register Rust runtime
    let rust_rt = Arc::new(RustRuntime::new(None, None));
    runtime_manager.register_runtime(rust_rt)?;
    info!("Rust runtime registered");

    // Check runtime health
    let health_results = runtime_manager.health_check_all().await;
    for (name, result) in health_results {
        match result {
            Ok(info) => info!("Runtime {} health: {:?}", name, info.status),
            Err(e) => info!("Runtime {} health check failed: {}", name, e),
        }
    }

    // Initialize and run server
    let server = Server::new(config, runtime_manager).await?;
    info!("Server initialized, starting on {}", server.addr());

    server.run().await?;

    Ok(())
}
