# AIDEN Developer Guide

## Table of Contents

1. [Getting Started](#getting-started)
2. [Project Structure](#project-structure)
3. [Building](#building)
4. [Testing](#testing)
5. [Debugging](#debugging)
6. [Code Style](#code-style)
7. [Adding New Features](#adding-new-features)
8. [API Development](#api-development)
9. [Service Integration](#service-integration)
10. [Performance Optimization](#performance-optimization)

---

## Getting Started

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs))
- Ollama running on localhost:11434
- Qdrant running on localhost:6334
- NVIDIA GPU with CUDA support (optional but recommended)

### Clone and Setup

```bash
git clone https://gitlab.acreetionos.org/natalie/aiden.git
cd aidenaiden
```

### Run Development Server

```bash
cargo run
```

The server will start on http://localhost:8081

---

## Project Structure

```
aiden/
├── src/
│   ├── main.rs              # Entry point, router setup
│   ├── handlers.rs          # HTTP/WebSocket handlers
│   ├── state.rs             # Configuration and state
│   ├── state/
│   │   └── message.rs       # Message types
│   └── services/
│       ├── mod.rs           # Service exports
│       ├── ollama.rs        # Ollama API client
│       ├── qdrant.rs        # Qdrant client
│       ├── rag.rs           # RAG pipeline
│       └── indexer.rs       # Document indexer
├── src/frontend/
│   └── index.html           # Web UI
├── docs/                    # Documentation
├── tests/                   # Integration tests
├── Cargo.toml               # Dependencies
└── README.md                # Project overview
```

---

## Building

### Debug Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

Binary location: `target/release/aiden`

### With Custom Features

```bash
cargo build --release --features "custom-feature"
```

---

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Doc Tests

```bash
cargo test --doc
```

### Run Benchmarks

```bash
cargo bench
```

---

## Debugging

### Enable Verbose Logging

```bash
RUST_LOG=debug ./target/debug/aiden
```

### Enable Trace Logging

```bash
RUST_LOG=trace ./target/debug/aiden
```

### Common Debug Endpoints

```bash
# Health check
curl http://localhost:8081/api/health

# Index status
curl http://localhost:8081/api/index/status
```

### LLDB Debugging

```bash
rust-lldb target/debug/aiden
```

---

## Code Style

### Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Pre-commit Hook

Add to `.git/hooks/pre-commit`:

```bash
#!/bin/bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
```

---

## Adding New Features

### 1. Create Handler

In `handlers.rs`:

```rust
pub async fn new_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<MyResponse>, StatusCode> {
    // Implementation
    Ok(Json(MyResponse { /* ... */ }))
}
```

### 2. Add Route

In `main.rs`:

```rust
Router::new()
    .route("/api/new", post(new_handler))
    // ... existing routes
```

### 3. Add Types

In appropriate module:

```rust
#[derive(Serialize, Deserialize)]
pub struct MyRequest {
    pub field: String,
}

#[derive(Serialize, Deserialize)]
pub struct MyResponse {
    pub result: String,
}
```

### 4. Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_new_feature() {
        // Test implementation
    }
}
```

---

## API Development

### REST Endpoints

Follow this pattern:

```rust
pub async fn endpoint(
    State(state): State<Arc<AppState>>,
    Json(req): Json<Request>,
) -> Result<Json<Response>, StatusCode> {
    // Validate input
    // Process request
    // Return response or error
}
```

### WebSocket Endpoints

```rust
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| async move {
        let mut socket = socket;
        // Handle WebSocket messages
    })
}
```

### Error Handling

```rust
match result {
    Ok(value) => Ok(Json(value)),
    Err(e) => {
        tracing::error!("Error: {}", e);
        Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}
```

---

## Service Integration

### Adding a New Service

1. Create `services/new_service.rs`:

```rust
pub struct NewService {
    client: reqwest::Client,
    base_url: String,
}

impl NewService {
    pub fn new(url: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url: url.to_string(),
        }
    }

    pub async fn call(&self) -> Result<String> {
        // Implementation
    }
}
```

2. Export in `services/mod.rs`:

```rust
pub mod new_service;
pub use new_service::NewService;
```

3. Use in handlers:

```rust
let service = NewService::new("http://localhost:8080");
let result = service.call().await?;
```

---

## Performance Optimization

### Async Patterns

- Use `tokio::spawn` for background tasks
- Use `Arc` for shared state
- Use connection pooling for HTTP clients

### Memory

- Limit message history size
- Use streaming for large responses
- Clear caches periodically

### CPU

- Offload to GPU via Ollama
- Use parallel processing for indexing
- Optimize hot paths

### Network

- Enable HTTP/2
- Use connection keep-alive
- Implement request batching

---

## Dependencies

### Adding a Dependency

```bash
cargo add package_name
```

### Updating Dependencies

```bash
cargo update
```

### Checking for Updates

```bash
cargo outdated
```

---

## Deployment

See [DEPLOYMENT.md](DEPLOYMENT.md) for deployment instructions.

---

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://docs.rs/axum/)
- [Tokio Documentation](https://docs.rs/tokio/)
- [Qdrant Client](https://docs.rs/qdrant-client/)
