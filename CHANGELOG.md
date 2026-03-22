# Changelog

All notable changes to AIDEN will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [0.1.0] - 2026-03-19

### Added
- Initial release
- Web-based chat interface with dark theme
- RAG pipeline with Ollama and Qdrant integration
- Axum web server with WebSocket support
- REST API endpoints:
  - `POST /api/chat` - Non-streaming chat
  - `POST /api/chat/stream` - Streaming chat via WebSocket
  - `POST /api/index` - Trigger documentation indexing
  - `GET /api/index/status` - Indexing progress
  - `GET /api/health` - Health check
- Markdown documentation indexer with chunking
- Sentence-based text chunking algorithm
- Source citation support in responses
- Conversation history tracking
- Background indexing with progress updates
- Health status indicators (Ollama, Qdrant)
- Modern responsive UI with Inter font
- Quick action buttons for common queries
- Indexing statistics modal
- Docker deployment support
- Systemd service template
- Comprehensive API documentation
- Architecture documentation
- Configuration reference
- Troubleshooting guide
- MIT License
- Contribution guidelines

### Features
- Ollama integration for chat and embeddings
- Qdrant vector database for RAG retrieval
- Configurable search threshold (default 0.7)
- Configurable max results (default 5)
- Configurable chunk size (default 512 words)
- CORS support for web interface
- Graceful error handling
- Per-message UUID tracking
- RFC3339 timestamps
- WebSocket message types: status, sources, response, done
