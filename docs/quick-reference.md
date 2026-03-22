# AIDEN Quick Reference

Quick reference guide for AIDEN.

## URLs

| Service | URL | Purpose |
|---------|-----|---------|
| AIDEN | http://localhost:8081 | Main web interface |
| Ollama | http://localhost:11434 | LLM API |
| Qdrant | http://localhost:6333 | Vector DB Dashboard |

## API Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/` | GET | Web interface |
| `/api/chat` | POST | Send chat message |
| `/api/chat/stream` | WS | Streaming chat |
| `/api/index` | POST | Trigger indexing |
| `/api/index/status` | GET | Get index status |
| `/api/health` | GET | Health check |

## Service Commands

```bash
# Start services
sudo systemctl start ollama qdrant aiden

# Stop services
sudo systemctl stop aiden qdrant ollama

# Restart services
sudo systemctl restart aiden

# View logs
journalctl -u aiden -f
```

## Common curl Commands

```bash
# Health check
curl http://localhost:8081/api/health

# Send message
curl -X POST http://localhost:8081/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello"}'

# Trigger indexing
curl -X POST http://localhost:8081/api/index

# Check index status
curl http://localhost:8081/api/index/status
```

## File Locations

| Path | Description |
|------|-------------|
| `/opt/aiden/aiden` | Binary |
| `/opt/aiden/docs/` | Documentation |
| `/var/lib/qdrant/` | Vector data |
| `/etc/systemd/system/aiden.service` | Service file |

## Ollama Commands

```bash
# List models
ollama list

# Pull model
ollama pull llama3.2:1b

# Remove model
ollama rm llama3.2:1b

# Check running models
curl http://localhost:11434/api/ps
```

## Configuration

Default settings in `src/state.rs`:

| Setting | Default | Description |
|---------|---------|-------------|
| `chat_model` | llama3.2:1b | Chat model |
| `embed_model` | nomic-embed-text | Embedding model |
| `threshold` | 0.7 | Search threshold |
| `max_results` | 5 | Max search results |
| `chunk_size` | 512 | Words per chunk |

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "Offline" status | `sudo systemctl start aiden` |
| Slow responses | `nvidia-smi` to check GPU |
| No sources | Re-index: `curl -X POST http://localhost:8081/api/index` |
| Connection error | Check Ollama: `curl localhost:11434/api/version` |

## Keyboard Shortcuts

| Key | Action |
|-----|--------|
| Enter | Send message |
| Shift+Enter | New line |
| Escape | Close modal |
