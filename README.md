# AIDEN - AI Documentation Assistant for AcreetionOS

AIDEN (AI Documentation Engine for AcreetionOS) is a web-based AI assistant that answers questions about AcreetionOS using Retrieval-Augmented Generation (RAG). It provides an intuitive chat interface for users to query documentation with source citations.

## Features

- **Chat Interface**: Natural language Q&A about AcreetionOS
- **Local RAG Pipeline**: No external API calls required for AI responses
- **Qdrant Vector Database**: Fast similarity search over documentation
- **Ollama Integration**: Local LLM inference with privacy-preserving design
- **Automatic Indexing**: Background documentation processing
- **Source Citations**: Every response links to relevant documentation
- **Modern Web UI**: Dark theme, responsive design, real-time status
- **WebSocket Streaming**: Smooth streaming responses
- **Progress Tracking**: Visual indexing progress indicators

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         Web Client (Browser)                          │
│                    src/frontend/index.html                            │
│              - Chat interface, status indicators                      │
│              - WebSocket for streaming responses                     │
└────────────────────────────────┬────────────────────────────────────┘
                                 │ HTTP/WebSocket
                                 ▼
┌─────────────────────────────────────────────────────────────────────┐
│                    Axum Web Server (Port 8081)                       │
│                           Rust + Tokio                               │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │                     AppState (RwLock)                           │  │
│  │  - Config (ollama, search, indexing settings)                    │  │
│  │  - Messages (conversation history)                              │  │
│  │  - IndexingStatus (background job progress)                     │  │
│  └────────────────────────────────────────────────────────────────┘  │
└────────────────────────────────┬────────────────────────────────────┘
                                 │
         ┌───────────────────────┼───────────────────────┐
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│  OllamaService  │    │  QdrantService  │    │   RAGService    │
│                 │    │                 │    │                 │
│ - chat()        │    │ - init_         │    │ - query()       │
│ - embeddings()  │    │   collection()  │    │                 │
│ - health_check()│    │ - search()      │    │                 │
│                 │    │ - upsert()      │    │                 │
└────────┬────────┘    └────────┬────────┘    └────────┬────────┘
         │                       │                       │
         │    ┌──────────────────┘                       │
         │    │                                          │
         ▼    ▼                                          ▼
┌─────────────────┐    ┌─────────────────────────────────────────────┐
│   Ollama API    │    │                  Qdrant                       │
│  (LLM + Embed)  │    │            (Vector Database)                  │
│   Port 11434    │    │              Port 6334                        │
└─────────────────┘    └─────────────────────────────────────────────┘
```

## Prerequisites

1. **Ollama** - For local LLM inference
   ```bash
   curl -fsSL https://ollama.com/install.sh | sh
   ```

2. **Qdrant** - For vector database
   ```bash
   docker pull qdrant/qdrant
   docker run -p 6333:6333 -p 6334:6334 -v qdrant_data:/qdrant/storage qdrant/qdrant
   ```

3. **Rust** - For building
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

## Quick Start

```bash
# Clone the repository
git clone https://gitlab.acreetionos.org/natalie/aiden.git
cd aidenaiden

# Install Ollama models
ollama pull llama3.2:1b
ollama pull nomic-embed-text

# Build
cargo build --release

# Run
./target/release/aiden
```

Open http://localhost:8081 in your browser.

## Configuration

Configuration is defined in `src/state.rs`. Key options:

| Option | Default | Description |
|--------|---------|-------------|
| `ollama.host` | `"localhost:11434"` | Ollama API server |
| `ollama.chat_model` | `"llama3.2:1b"` | Chat LLM model |
| `ollama.embed_model` | `"nomic-embed-text"` | Embedding model |
| `search.threshold` | `0.7` | Minimum similarity score |
| `search.max_results` | `5` | Max documents to retrieve |
| `indexing.docs_path` | `"./docs"` | Documentation directory |
| `indexing.chunk_size` | `512` | Words per chunk |

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Web interface |
| `/api/chat` | POST | Non-streaming chat |
| `/api/chat/stream` | POST (WS) | Streaming chat |
| `/api/index` | POST | Trigger indexing |
| `/api/index/status` | GET | Indexing status |
| `/api/health` | GET | Health check |

## Deployment

See [DEPLOYMENT.md](docs/DEPLOYMENT.md) for detailed deployment instructions including:
- Docker deployment
- Systemd service setup (for boot)
- Reverse proxy configuration
- Production optimizations

## Documentation

- [Architecture](docs/architecture.md) - System design and data flow
- [API Reference](docs/api-reference.md) - REST API documentation
- [Configuration](docs/configuration.md) - All configuration options
- [Troubleshooting](docs/troubleshooting.md) - Common issues and solutions

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for version history.

## Security

See [SECURITY.md](SECURITY.md) for security policy.

## License

MIT License. See [LICENSE](LICENSE).

## Authors

- **Natalie Spiva** - [natalie@acreetionos.org](mailto:natalie@acreetionos.org)

## Links

- **GitLab**: https://gitlab.acreetionos.org/natalie/aiden
- **GitHub**: https://github.com/AcreetionOS-Code/aiden
- **Live Instance**: https://aiden.acreetionos.org
