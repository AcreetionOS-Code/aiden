# AIDEN Release Notes

## v0.1.0 - Initial Release

**Date**: March 19, 2026

### Features

- Web-based chat interface
- RAG pipeline with Ollama and Qdrant
- REST API endpoints
- WebSocket streaming support
- Documentation indexer
- Source citations
- Health monitoring
- Docker deployment
- Systemd service support

### Technology

- Rust with Axum framework
- Tokio async runtime
- Qdrant vector database
- Ollama LLM integration
- nomic-embed-text embeddings

### Requirements

- Ollama 0.18+
- Qdrant 1.17+
- Rust 1.75+
- 8GB RAM minimum
- NVIDIA GPU (optional)

### Known Issues

- No built-in authentication
- No TLS support
- No persistent chat history
- No multi-user support

---

## Roadmap

### v0.2.0 (Planned)

- [ ] Configuration file support
- [ ] Environment variable overrides
- [ ] Persistent chat history
- [ ] User authentication
- [ ] Multi-language support

### v0.3.0 (Planned)

- [ ] Admin dashboard
- [ ] Usage analytics
- [ ] Custom prompts
- [ ] Plugin system
- [ ] API key management

### Future

- [ ] Mobile app
- [ ] Desktop app
- [ ] Team collaboration
- [ ] Cloud sync
- [ ] Advanced AI models
