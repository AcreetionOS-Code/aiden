# Configuration Reference

All configuration is defined in `src/state.rs`. This document covers all available options.

## Config Struct

```rust
pub struct Config {
    pub ollama: OllamaConfig,
    pub search: SearchConfig,
    pub indexing: IndexingConfig,
}
```

## OllamaConfig

Connection settings for Ollama LLM service.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `host` | String | `"localhost:11434"` | Ollama API server address |
| `chat_model` | String | `"llama3.2:1b"` | Model for chat completion |
| `embed_model` | String | `"nomic-embed-text"` | Model for text embeddings |

### Example
```rust
OllamaConfig {
    host: "localhost:11434".to_string(),
    chat_model: "llama3.2:1b".to_string(),
    embed_model: "nomic-embed-text".to_string(),
}
```

## SearchConfig

RAG search behavior settings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `enabled` | bool | `true` | Enable/disable RAG search |
| `api_key` | Option<String> | `None` | Optional external search API key |
| `threshold` | f32 | `0.7` | Minimum similarity score (0.0-1.0) |
| `max_results` | usize | `5` | Maximum documents to retrieve |

### Threshold

Controls minimum relevance for retrieved documents:

| Value | Behavior |
|-------|----------|
| 0.9+ | Only highly relevant results |
| 0.7 | Balanced (default) |
| 0.5 | More results, lower quality |
| 0.0 | Return all results |

### Example
```rust
SearchConfig {
    enabled: true,
    api_key: None,
    threshold: 0.7,
    max_results: 5,
}
```

## IndexingConfig

Documentation indexing settings.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `docs_path` | String | `"./docs"` | Directory containing markdown files |
| `chunk_size` | usize | `512` | Target words per chunk |
| `chunk_overlap` | usize | `50` | Overlap between chunks (words) |
| `collection_name` | String | `"aiden"` | Qdrant collection name |

### Chunk Size Guidelines

| Use Case | Recommended Size |
|----------|-----------------|
| Short FAQs | 128-256 words |
| General docs | 512 words (default) |
| Long articles | 768-1024 words |
| Technical manuals | 1024+ words |

### Example
```rust
IndexingConfig {
    docs_path: "./docs".to_string(),
    chunk_size: 512,
    chunk_overlap: 50,
    collection_name: "aiden".to_string(),
}
```

## Default Configuration

The application uses these defaults if not overridden:

```rust
impl Default for Config {
    fn default() -> Self {
        Self {
            ollama: OllamaConfig::default(),
            search: SearchConfig::default(),
            indexing: IndexingConfig::default(),
        }
    }
}
```

## Modifying Configuration

Currently, configuration must be modified in `src/state.rs` and the application recompiled:

1. Edit `src/state.rs`
2. Change desired values
3. Run `cargo build --release`
4. Restart the service

## Environment Variables

Currently not supported. Configuration is compile-time only.

## Future Plans

- [ ] TOML/YAML configuration file support
- [ ] Environment variable overrides
- [ ] Runtime configuration via API
- [ ] Hot reload without restart
