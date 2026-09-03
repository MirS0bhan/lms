use crate::config::Config;
use crate::error::Result;
use crate::registry::FunctionRegistry;
use crate::mcp::handlers::McpHandler;
use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tracing::info;

pub struct Server {
    config: Config,
    router: Router,
    registry: Arc<FunctionRegistry>,
    _mcp_handler: McpHandler,
}

impl Server {
    pub async fn new(config: Config) -> Result<Self> {
        let registry = FunctionRegistry::new();
        let mcp_handler = McpHandler::new(registry.clone());

        let app = Self::build_router(&registry);

        Ok(Self {
            config,
            router: app,
            registry,
            _mcp_handler: mcp_handler,
        })
    }

    fn build_router(registry: &Arc<FunctionRegistry>) -> Router {
        let registry = registry.clone();
        
        Router::new()
            .route("/health", get(health_check))
            .route("/functions", get(list_functions))
            .route("/functions/:name", get(get_function_info))
            .route("/register", post(register_function))
            .route("/invoke", post(invoke_function))
            .route("/revoke", post(revoke_function))
            .with_state(registry)
    }

    pub fn addr(&self) -> std::net::SocketAddr {
        self.config.socket_addr()
    }

    pub async fn run(self) -> Result<()> {
        let addr = self.addr();
        info!("Starting server on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, self.router).await?;

        Ok(())
    }
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

async fn list_functions(
    axum::extract::State(registry): axum::extract::State<Arc<FunctionRegistry>>,
) -> impl IntoResponse {
    let functions = registry.list();
    Json(functions)
}

async fn get_function_info(
    axum::extract::State(registry): axum::extract::State<Arc<FunctionRegistry>>,
    axum::extract::Path(name): axum::extract::Path<String>,
) -> impl IntoResponse {
    match registry.get(&name) {
        Ok(metadata) => (StatusCode::OK, Json(metadata)).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "Function not found" })),
        )
            .into_response(),
    }
}

async fn register_function() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Not implemented in Phase 1"
        })),
    )
}

async fn invoke_function() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Not implemented in Phase 1"
        })),
    )
}

async fn revoke_function() -> impl IntoResponse {
    (
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "error": "Not implemented in Phase 1"
        })),
    )
}
