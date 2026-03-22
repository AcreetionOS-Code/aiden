# Contributing to AIDEN

Thank you for your interest in contributing to AIDEN!

## Development Setup

1. **Clone the repository**
   ```bash
   git clone https://gitlab.acreetionos.org/natalie/aiden.git
   cd aidenaiden
   ```

2. **Install prerequisites**
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install Ollama
   curl -fsSL https://ollama.com/install.sh | sh

   # Pull required models
   ollama pull llama3.2:1b
   ollama pull nomic-embed-text

   # Start Qdrant
   docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
   ```

3. **Build and run**
   ```bash
   cargo build --debug
   ./target/debug/aiden
   ```

## Project Structure

```
aiden/
├── src/
│   ├── main.rs           # Application entry point
│   ├── handlers.rs       # HTTP/WebSocket request handlers
│   ├── state.rs          # Configuration and application state
│   ├── state/
│   │   └── message.rs    # Chat message data structures
│   └── services/
│       ├── mod.rs        # Service module exports
│       ├── ollama.rs     # Ollama API client
│       ├── qdrant.rs     # Qdrant vector DB client
│       ├── rag.rs        # RAG pipeline orchestrator
│       └── indexer.rs    # Documentation indexer
├── src/frontend/
│   └── index.html        # Web interface
└── docs/                 # Documentation markdown files
```

## Code Style

- Run `cargo fmt` before committing
- Follow Rust idioms and conventions
- Use meaningful variable and function names
- Add comments for complex logic

## Testing

```bash
cargo test
cargo clippy
```

## Commit Messages

Format: `<type>: <description>`

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `refactor`: Code refactoring
- `style`: Formatting changes
- `test`: Adding tests
- `chore`: Maintenance tasks

Example:
```
feat: add WebSocket streaming support
fix: handle Qdrant connection errors gracefully
docs: update API documentation
```

## Branching Strategy

- `main`: Stable release branch
- `develop`: Development branch
- Feature branches: `feature/<feature-name>`
- Bugfix branches: `fix/<bug-description>`

## Pull Request Process

1. Create a feature branch from `develop`
2. Make your changes
3. Run tests and linting
4. Submit a pull request
5. Request review from maintainers
6. Squash and merge after approval

## Questions?

- Open an issue on GitLab/GitHub
- Contact: natalie@acreetionos.org
