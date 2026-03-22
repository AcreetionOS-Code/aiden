# Architecture Documentation

## System Overview

AIDEN (AI Documentation Engine) is a Rust-based web application that provides an AI-powered documentation assistant using Retrieval-Augmented Generation (RAG).

## Technology Stack

| Component | Technology | Purpose |
|-----------|------------|---------|
| Web Server | Axum 0.7 | HTTP/WebSocket server |
| Async Runtime | Tokio | Async I/O operations |
| LLM Inference | Ollama | Local AI model inference |
| Vector DB | Qdrant | Semantic search storage |
| Serialization | Serde | JSON serialization |
| HTTP Client | Reqwest | HTTP requests to Ollama |
| Logging | Tracing | Structured logging |

## Module Architecture

### Core Modules

#### main.rs
Entry point that:
- Initializes logging with tracing-subscriber
- Creates Axum router with CORS middleware
- Mounts static frontend files
- Sets up WebSocket handler
- Binds to TCP listener on port 8081

```rust
// Key initialization
tracing_subscriber::fmt::init();
let app = Router::new()
    .route("/", get(index))
    .route("/api/chat", post(chat))
    .route("/api/chat/stream", get(chat_stream))
    .route("/api/index", post(index_docs))
    .route("/api/index/status", get(index_status))
    .route("/api/health", get(health))
    .with_state(state);
```

#### handlers.rs
Request handlers for all HTTP endpoints:

| Handler | Endpoint | Purpose |
|---------|----------|---------|
| `index` | GET / | Serve frontend HTML |
| `chat` | POST /api/chat | Non-streaming chat |
| `chat_stream` | WS /api/chat/stream | Streaming chat |
| `index_docs` | POST /api/index | Trigger indexing |
| `index_status` | GET /api/index/status | Get progress |
| `health` | GET /api/health | Health check |

#### state.rs
Application state management:

```rust
pub struct AppState {
    pub config: Config,
    pub messages: VecDeque<ChatMessage>,
    pub indexing_status: Option<IndexingStatus>,
}

pub struct Config {
    pub ollama: OllamaConfig,
    pub search: SearchConfig,
    pub indexing: IndexingConfig,
}
```

#### state/message.rs
Message data structures:

```rust
pub struct ChatMessage {
    pub id: String,              // UUID v4
    pub role: Role,              // User or Assistant
    pub content: String,         // Message text
    pub sources: Vec<Source>,    // Retrieved context
    pub used_web_fallback: bool, // Web search fallback
    pub timestamp: String,        // RFC3339
}

pub struct Source {
    pub source_type: SourceType,
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: f32,              // 0.0 - 1.0
}
```

## Service Architecture

### OllamaService (services/ollama.rs)

Handles communication with Ollama API:

```rust
pub struct OllamaService {
    client: reqwest::Client,  // HTTP client with 300s timeout
    base_url: String,         // e.g., "http://localhost:11434"
}

// Key methods:
// - chat(): Generate chat response
// - embeddings(): Generate text embeddings
// - health_check(): Verify Ollama is running
```

### QdrantService (services/qdrant.rs)

Manages vector database operations:

```rust
pub struct QdrantService {
    client: Arc<Qdrant>,      // Thread-safe client
    collection_name: String,  // Default: "aiden"
}

// Key methods:
// - init_collection(): Create 768-dim cosine collection
// - search(): Vector similarity search
// - upsert_points(): Store embedded chunks
```

### RAGService (services/rag.rs)

Orchestrates the RAG pipeline:

```rust
pub struct RAGService {
    ollama: OllamaService,
    qdrant: QdrantService,
    chat_model: String,
    embed_model: String,
    threshold: f32,           // Min similarity score
    max_results: usize,       // Max docs to retrieve
}

// query() method flow:
// 1. Generate query embedding
// 2. Search Qdrant for similar documents
// 3. Check top score against threshold
// 4. Build context from retrieved docs
// 5. Generate response with Ollama chat
```

### IndexerService (services/indexer.rs)

Processes documentation into searchable chunks:

```rust
pub struct IndexerService {
    ollama: OllamaService,
    qdrant: QdrantService,
    docs_path: String,
    chunk_size: usize,
    embed_model: String,
}

// index_all() method flow:
// 1. Initialize Qdrant collection
// 2. Walk docs/ directory for .md files
// 3. For each file:
//    - Read content
//    - Split into chunks (sentence-based)
//    - Generate embeddings
//    - Upsert to Qdrant
// 4. Update progress state
```

## Data Flow

### Chat Request Flow

```
1. Browser sends POST /api/chat with { "message": "..." }
         ↓
2. Handler extracts request, creates services
         ↓
3. RAGService.query():
   a) OllamaService.embeddings() → query vector
         ↓
   b) QdrantService.search() → similar documents
         ↓
   c) Check if top_score < threshold → flag fallback
         ↓
   d) Build system prompt with context
         ↓
   e) OllamaService.chat() → LLM response
         ↓
4. Handler returns ChatResponse
         ↓
5. Frontend displays response with sources
```

### Indexing Flow

```
1. Browser sends POST /api/index
         ↓
2. Handler spawns background task
         ↓
3. IndexerService.index_all():
   a) QdrantService.init_collection()
         ↓
   b) WalkDir walks ./docs/*.md
         ↓
   c) For each file:
      - Read content
      - Split into chunks
      - Generate embeddings
      - Upsert to Qdrant
         ↓
   d) Update AppState.indexing_status
         ↓
4. Frontend polls /api/index/status
```

## WebSocket Protocol

The streaming endpoint uses JSON messages over WebSocket:

```typescript
// Client → Server
{ "type": "chat", "message": "What is AcreetionOS?" }

// Server → Client (status)
{ "type": "status", "status": "indexing", "message": "Searching..." }

// Server → Client (sources)
{ "type": "sources", "sources": [...] }

// Server → Client (response)
{ "type": "response", "content": "AcreetionOS is...", "sources": [...] }

// Server → Client (done)
{ "type": "done" }
```

## Error Handling

### Pattern 1: Result Propagation
```rust
// Services return anyhow::Result
pub async fn chat(&self, ...) -> Result<String> {
    let response = self.client.post(...).send().await?;
    Ok(response.json().await?)
}
```

### Pattern 2: Handler Error Mapping
```rust
pub async fn chat(...) -> Result<Json<ChatResponse>, StatusCode> {
    rag.query(&req.message).await.map_err(|e| {
        tracing::error!("RAG query failed: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
}
```

### Pattern 3: Health Check Aggregation
```rust
// Multiple service health → single status
pub async fn health(...) -> Json<HealthResponse> {
    let ollama_ok = ollama.health_check().await.unwrap_or(false);
    let qdrant_ok = QdrantService::new(...).await.map(|_| true).unwrap_or(false);
    
    Json(HealthResponse {
        status: if ollama_ok && qdrant_ok { "healthy" } else { "degraded" },
        ollama: ollama_ok,
        qdrant: qdrant_ok,
    })
}
```

## Frontend Architecture

Single-page application in `src/frontend/index.html`:

```
┌─────────────────────────────────────────┐
│                 Header                    │
│  [Logo] AIDEN    [Status] [Index Docs]   │
├─────────────────────────────────────────┤
│                                          │
│  ┌─────────────────────────────────────┐ │
│  │           Welcome Card               │ │
│  │  Quick Action Buttons               │ │
│  └─────────────────────────────────────┘ │
│                                          │
│  ┌─────────────────────────────────────┐ │
│  │           Message Thread             │ │
│  │  - User messages (right)             │ │
│  │  - AI responses (left)               │ │
│  │  - Source citations                 │ │
│  └─────────────────────────────────────┘ │
│                                          │
│  ┌─────────────────────────────────────┐ │
│  │           Input Area                 │ │
│  │  [Textarea              ] [Send]    │ │
│  └─────────────────────────────────────┘ │
│                                          │
└─────────────────────────────────────────┘
```

## Performance Considerations

1. **Connection Pooling**: Reqwest client is reused
2. **Thread Safety**: Arc<Qdrant> for shared state
3. **Background Indexing**: Non-blocking document processing
4. **Streaming Responses**: WebSocket for real-time updates
5. **Chunk Overlap**: 50-word overlap for context preservation

## Security Model

- No built-in auth (use reverse proxy)
- No TLS (use reverse proxy)
- Local-only Ollama (data stays on network)
- Input validation via serde
- CORS allows all origins (configure for production)
