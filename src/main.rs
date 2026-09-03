use anyhow::Result;
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

    // Initialize and run server
    let server = Server::new(config).await?;
    info!("Server initialized, starting on {}", server.addr());

    server.run().await?;

    Ok(())
}
