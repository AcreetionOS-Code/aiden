# AIDEN FAQ

Frequently Asked Questions about AIDEN.

## General

### What is AIDEN?

AIDEN (AI Documentation Engine) is a web-based AI assistant that answers questions about AcreetionOS using Retrieval-Augmented Generation (RAG). It uses local AI models to provide accurate, source-cited answers from documentation.

### What does AIDEN stand for?

AIDEN stands for **AI Documentation Engine** (sometimes "AI Documentation Assistant for AcreetionOS").

### What technologies does AIDEN use?

- **Axum**: Web framework
- **Tokio**: Async runtime
- **Ollama**: Local LLM inference
- **Qdrant**: Vector database
- **Rust**: Programming language

### Is AIDEN free?

Yes, AIDEN is open-source under the MIT license.

---

## Installation

### What are the system requirements?

**Minimum:**
- 4 CPU cores
- 8GB RAM
- 10GB storage

**Recommended:**
- 8+ CPU cores
- 16GB RAM
- NVIDIA GPU with 4GB+ VRAM
- 20GB storage

### Can I run AIDEN without a GPU?

Yes, but responses will be slower. AIDEN will automatically fall back to CPU-only mode.

### Which GPU do I need?

Any NVIDIA GPU with 4GB+ VRAM works. Tested with:
- GTX 1660 SUPER (6GB)
- RTX 3080 (10GB)
- RTX 4090 (24GB)

### Can I run AIDEN on multiple computers?

Yes. Either:
1. Install AIDEN on each computer
2. Run AIDEN as a server and access via network

---

## Usage

### How do I ask questions?

1. Open http://localhost:8081
2. Type your question in the input field
3. Press Enter or click Send

### What types of questions can I ask?

You can ask anything about:
- AcreetionOS features
- Installation guides
- Configuration
- Troubleshooting
- Best practices

### Why don't I see source citations?

Reasons:
- Question doesn't match indexed content
- Need to re-index documentation
- Search threshold too high

Solution: Re-index docs via the "Index Docs" button.

### How do I improve answer quality?

1. Add more documentation to the `docs/` folder
2. Re-index after adding docs
3. Adjust search threshold (lower = more results)
4. Use a larger/better model

### Can I use AIDEN offline?

Yes. Once installed, AIDEN works completely offline.

---

## Models

### What models does AIDEN use?

By default:
- **Chat**: llama3.2:1b (1.3GB)
- **Embeddings**: nomic-embed-text (274MB)

### How do I change the model?

1. Edit `src/state.rs`
2. Change `chat_model` to desired model
3. Rebuild: `cargo build --release`
4. Restart service

### Which model should I use?

| Model | Speed | Quality | VRAM |
|-------|--------|---------|------|
| llama3.2:1b | Fast | Good | 1.3GB |
| llama3.2:3b | Medium | Better | 2GB |
| llama3.1:8b | Slow | Best | 5GB |

### How do I update models?

```bash
ollama pull llama3.2:1b
```

---

## Troubleshooting

### AIDEN is not responding

1. Check service status:
   ```bash
   systemctl status aiden
   ```

2. Restart service:
   ```bash
   sudo systemctl restart aiden
   ```

3. Check logs:
   ```bash
   journalctl -u aiden -n 50
   ```

### "Ollama not connected"

1. Check Ollama status:
   ```bash
   systemctl status ollama
   ```

2. Restart Ollama:
   ```bash
   sudo systemctl restart ollama
   ```

3. Verify Ollama is running:
   ```bash
   curl http://localhost:11434/api/version
   ```

### "Qdrant not connected"

1. Check Qdrant status:
   ```bash
   systemctl status qdrant
   ```

2. Restart Qdrant:
   ```bash
   sudo systemctl restart qdrant
   ```

3. Verify Qdrant is running:
   ```bash
   curl http://localhost:6333/readyz
   ```

### Responses are slow

1. Check GPU usage:
   ```bash
   nvidia-smi
   ```

2. Verify Ollama is using GPU (should show ollama process with GPU memory)

3. Consider using a smaller model

### No sources found

1. Check if docs exist:
   ```bash
   ls docs/
   ```

2. Re-index:
   ```bash
   curl -X POST http://localhost:8081/api/index
   ```

3. Check index status:
   ```bash
   curl http://localhost:8081/api/index/status
   ```

---

## Data & Privacy

### Where is my data stored?

- **AIDEN**: Runs locally
- **Ollama**: Processes queries locally
- **Qdrant**: Vector data stored locally
- **Documentation**: In your `docs/` folder

### Is my data sent to external servers?

No. All processing happens locally on your machine. No data is sent to external servers.

### Can I export my chat history?

Currently, chat history is not persisted. Each session starts fresh.

### How do I backup my data?

```bash
# Backup Qdrant data
sudo tar -czf qdrant_backup.tar.gz /var/lib/qdrant

# Backup documentation
tar -czf docs_backup.tar.gz docs/
```

---

## Development

### How do I contribute?

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines.

### How do I report bugs?

Open an issue on the project repository with:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information

### Can I customize AIDEN?

Yes! AIDEN is open source. You can:
- Modify the code
- Change models
- Add features
- Fork the project

### How do I add new documentation?

1. Add markdown files to `docs/` folder
2. Re-index via the web interface or:
   ```bash
   curl -X POST http://localhost:8081/api/index
   ```

---

## Deployment

### How do I run AIDEN at boot?

```bash
sudo systemctl enable aiden
```

### How do I expose AIDEN over HTTPS?

Use a reverse proxy like nginx or Caddy with TLS.

### Can I run AIDEN in Docker?

Yes! See [DEPLOYMENT.md](DEPLOYMENT.md) for Docker instructions.

### How do I scale AIDEN?

For multiple users:
1. Run AIDEN on a server
2. Set up load balancer
3. Configure authentication
4. Point clients to server URL

---

## Performance

### How fast are responses?

Depends on:
- Hardware (CPU/GPU)
- Model size
- Query complexity
- Network (for remote)

Typical: 2-10 seconds per response.

### How many concurrent users?

Tested with:
- 1-5 users: Works well
- 5-20 users: May need scaling
- 20+ users: Requires multiple instances

### How much memory does AIDEN use?

| Component | Memory |
|-----------|--------|
| AIDEN | 50-100MB |
| Ollama (1b model) | 2-4GB |
| Ollama (8b model) | 6-8GB |
| Qdrant | 200-500MB |

---

## Security

### Is AIDEN secure?

AIDEN itself is secure, but:
- No built-in auth (use reverse proxy)
- No TLS (use reverse proxy)
- All data stays local

### How do I secure AIDEN?

1. Use reverse proxy with TLS
2. Add authentication
3. Configure firewall
4. Enable rate limiting
5. Keep software updated

### Can I disable external model access?

Yes. AIDEN uses local models only. No external API calls are made.

---

## Still have questions?

- Check the [Troubleshooting Guide](troubleshooting.md)
- Review the [User Guide](user-guide.md)
- Open an issue on GitLab/GitHub
