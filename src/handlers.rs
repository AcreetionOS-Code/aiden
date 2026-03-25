use axum::{
    extract::{ws, Json, State, WebSocketUpgrade},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::{IndexerService, OllamaService, QdrantService, RAGService};
use crate::state::{AppState, ChatMessage};
use futures::{SinkExt, StreamExt};

#[derive(Clone)]
pub struct AppStateRef {
    pub inner: Arc<RwLock<AppState>>,
}

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ChatResponse {
    pub response: ChatMessage,
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub ollama: bool,
    pub qdrant: bool,
}

#[derive(Debug, Serialize)]
pub struct IndexStatusResponse {
    pub is_indexing: bool,
    pub files_processed: usize,
    pub chunks_created: usize,
    pub errors: Vec<String>,
}

pub async fn index() -> impl IntoResponse {
    let html = include_str!("frontend/index.html");
    Html(html)
}

pub async fn chat(
    State(state): State<AppStateRef>,
    Json(req): Json<ChatRequest>,
) -> Result<Json<ChatResponse>, StatusCode> {
    let config = {
        let state = state.inner.read().await;
        state.config.clone()
    };

    let ollama = OllamaService::new(&config.ollama.host);
    let qdrant = QdrantService::new(&config.indexing.collection_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let rag = RAGService::new(
        ollama,
        qdrant,
        config.ollama.chat_model,
        config.ollama.embed_model,
        config.search.threshold,
        config.search.max_results,
    );

    let response = rag.query(&req.message).await.map_err(|e| {
        tracing::error!("RAG query failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ChatResponse { response }))
}

pub async fn chat_stream(
    ws: WebSocketUpgrade,
    State(state): State<AppStateRef>,
    Json(req): Json<ChatRequest>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket: axum::extract::ws::WebSocket| async move {
        let config = {
            let state = state.inner.read().await;
            state.config.clone()
        };

        let ollama = OllamaService::new(&config.ollama.host);
        let qdrant = match QdrantService::new(&config.indexing.collection_name).await {
            Ok(q) => q,
            Err(e) => {
                tracing::error!("Qdrant connection failed: {}", e);
                return;
            }
        };

        let rag = RAGService::new(
            ollama,
            qdrant,
            config.ollama.chat_model,
            config.ollama.embed_model,
            config.search.threshold,
            config.search.max_results,
        );

        let (mut sink, _stream) = socket.split();

        let _ = sink
            .send(ws::Message::Text(
                "{\"type\":\"status\",\"content\":\"Thinking...\"}".into(),
            ))
            .await;

        match rag.query(&req.message).await {
            Ok(response) => {
                if !response.sources.is_empty() {
                    let sources_json = serde_json::json!({
                        "type": "sources",
                        "sources": response.sources
                    });
                    let _ = sink
                        .send(ws::Message::Text(
                            serde_json::to_string(&sources_json).unwrap(),
                        ))
                        .await;
                }

                let _ = sink
                    .send(ws::Message::Text(
                        serde_json::json!({
                            "type": "response",
                            "content": response.content,
                            "id": response.id,
                            "used_web_fallback": response.used_web_fallback
                        })
                        .to_string(),
                    ))
                    .await;
            }
            Err(e) => {
                tracing::error!("RAG query failed: {}", e);
                let _ = sink
                    .send(ws::Message::Text(
                        "{\"type\":\"error\",\"content\":\"Failed to process query\"}".into(),
                    ))
                    .await;
            }
        }

        let _ = sink
            .send(ws::Message::Text("{\"type\":\"done\"}".into()))
            .await;
    })
}

pub async fn index_docs(
    State(state): State<AppStateRef>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let config = {
        let mut s = state.inner.write().await;
        s.indexing_status = Some(crate::state::IndexingStatus {
            is_indexing: true,
            ..Default::default()
        });
        s.config.clone()
    };

    let ollama = OllamaService::new(&config.ollama.host);
    let qdrant = QdrantService::new(&config.indexing.collection_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let indexer = IndexerService::new(
        ollama,
        qdrant,
        config.indexing.docs_path,
        config.indexing.chunk_size,
        config.ollama.embed_model,
    );

    tokio::spawn(async move {
        match indexer.index_all().await {
            Ok(stats) => {
                let mut state = state.inner.write().await;
                state.indexing_status = Some(crate::state::IndexingStatus {
                    is_indexing: false,
                    progress: 1.0,
                    files_processed: stats.files_processed,
                    chunks_created: stats.chunks_created,
                    current_file: "Complete".to_string(),
                    errors: stats.errors,
                });
                tracing::info!(
                    "Indexing complete: {} files, {} chunks",
                    stats.files_processed,
                    stats.chunks_created
                );
            }
            Err(e) => {
                let mut state = state.inner.write().await;
                state.indexing_status = Some(crate::state::IndexingStatus {
                    is_indexing: false,
                    errors: vec![e.to_string()],
                    ..Default::default()
                });
                tracing::error!("Indexing failed: {}", e);
            }
        }
    });

    Ok(Json(serde_json::json!({
        "status": "started",
        "message": "Indexing started in background"
    })))
}

pub async fn index_status(State(state): State<AppStateRef>) -> Json<IndexStatusResponse> {
    let status = state.inner.read().await.indexing_status.clone();

    match status {
        Some(s) => Json(IndexStatusResponse {
            is_indexing: s.is_indexing,
            files_processed: s.files_processed,
            chunks_created: s.chunks_created,
            errors: s.errors,
        }),
        None => Json(IndexStatusResponse {
            is_indexing: false,
            files_processed: 0,
            chunks_created: 0,
            errors: Vec::new(),
        }),
    }
}

pub async fn health(State(state): State<AppStateRef>) -> Json<HealthResponse> {
    let config = state.inner.read().await.config.clone();

    let ollama = OllamaService::new(&config.ollama.host);
    let ollama_ok = ollama.health_check().await.unwrap_or(false);

    let qdrant_ok = QdrantService::new(&config.indexing.collection_name)
        .await
        .map(|_| true)
        .unwrap_or(false);

    Json(HealthResponse {
        status: if ollama_ok && qdrant_ok {
            "healthy".to_string()
        } else {
            "degraded".to_string()
        },
        ollama: ollama_ok,
        qdrant: qdrant_ok,
    })
}
