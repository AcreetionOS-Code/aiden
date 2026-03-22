mod handlers;
mod services;
mod state;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{Router, routing::{get, post}};

use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting AIDEN Web Server");

    let state = handlers::AppStateRef {
        inner: Arc::new(RwLock::new(AppState::new())),
    };

    let app = Router::new()
        .route("/", get(handlers::index))
        .route("/api/chat", post(handlers::chat))
        .route("/api/chat/stream", post(handlers::chat_stream))
        .route("/api/index", post(handlers::index_docs))
        .route("/api/index/status", get(handlers::index_status))
        .route("/api/health", get(handlers::health))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
    tracing::info!("AIDEN listening on http://0.0.0.0:8081");
    
    axum::serve(listener, app).await?;

    Ok(())
}
