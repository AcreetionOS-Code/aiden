# Troubleshooting Guide

Common issues and their solutions for AIDEN.

## Connection Issues

### Ollama Connection Failed

**Symptom**
```
Error: connection refused
Caused by: Connection refused (os error 111)
```

**Solution**
1. Ensure Ollama is installed:
   ```bash
   curl -fsSL https://ollama.com/install.sh | sh
   ```

2. Start Ollama service:
   ```bash
   ollama serve
   ```

3. Verify it's running:
   ```bash
   curl http://localhost:11434/api/version
   ```

4. Pull required models:
   ```bash
   ollama pull llama3.2:1b
   ollama pull nomic-embed-text
   ```

---

### Qdrant Connection Failed

**Symptom**
```
Error: Connection refused to Qdrant at localhost:6334
```

**Solution**
1. Start Qdrant with Docker:
   ```bash
   docker run -p 6333:6333 -p 6334:6334 qdrant/qdrant
   ```

2. Verify it's running:
   ```bash
   curl http://localhost:6333/readyz
   ```

3. For persistent storage:
   ```bash
   docker run -p 6333:6333 -p 6334:6334 \
     -v qdrant_data:/qdrant/storage \
     qdrant/qdrant
   ```

---

## Model Issues

### Model Not Found

**Symptom**
```
Error: model 'llama3.2:1b' not found
```

**Solution**
```bash
ollama pull llama3.2:1b
ollama pull nomic-embed-text
```

List available models:
```bash
ollama list
```

---

### Out of Memory

**Symptom**
```
Error: CUDA out of memory
```

**Solution**
1. Use a smaller model:
   ```rust
   // In src/state.rs
   chat_model: "llama3.2:1b".to_string(),  // Smaller model
   ```

2. Close other applications using GPU

3. Check GPU memory:
   ```bash
   nvidia-smi
   ```

---

## Indexing Issues

### No Documents Found

**Symptom**
```
Files processed: 0
```

**Solution**
1. Create docs directory:
   ```bash
   mkdir -p docs
   ```

2. Add markdown files:
   ```bash
   echo "# My Doc" > docs/README.md
   ```

3. Verify files exist:
   ```bash
   ls -la docs/
   find docs/ -name "*.md"
   ```

---

### Indexing Stuck

**Symptom**
```
Indexing status never completes
```

**Solution**
1. Check logs:
   ```bash
   journalctl -u aiden -f
   ```

2. Restart service:
   ```bash
   sudo systemctl restart aiden
   ```

3. Clear Qdrant collection and re-index:
   ```bash
   curl -X DELETE http://localhost:6333/collections/aiden
   ```

---

### Poor Search Results

**Symptom**
```
Responses don't match documentation
```

**Solution**
1. Check threshold (try lowering):
   ```rust
   // In src/state.rs
   threshold: 0.5,  // Lower threshold
   ```

2. Increase max results:
   ```rust
   max_results: 10,  // More context
   ```

3. Re-index with larger chunks:
   ```rust
   chunk_size: 768,  // Larger chunks
   ```

---

## Web Interface Issues

### Page Won't Load

**Symptom**
```
404 Not Found
```

**Solution**
1. Check server is running:
   ```bash
   curl http://localhost:8081/
   ```

2. Verify firewall:
   ```bash
   sudo ufw allow 8081
   ```

3. Check logs:
   ```bash
   sudo journalctl -u aiden -n 50
   ```

---

### WebSocket Connection Failed

**Symptom**
```
WebSocket connection failed
```

**Solution**
1. Check reverse proxy WebSocket support:
   ```nginx
   # Nginx location block
   proxy_http_version 1.1;
   proxy_set_header Upgrade $http_upgrade;
   proxy_set_header Connection "upgrade";
   ```

2. Test WebSocket directly:
   ```bash
   wscat -c ws://localhost:8081/api/chat/stream
   ```

---

## Build Issues

### Compilation Errors

**Symptom**
```
error: cannot find crate 'axum'
```

**Solution**
```bash
cargo update
cargo build
```

---

### Missing OpenSSL

**Symptom**
```
error: unable to find the openssl development headers
```

**Solution**
```bash
# Debian/Ubuntu
sudo apt install libssl-dev pkg-config

# Arch Linux
sudo pacman -S openssl pkg-config

# Fedora
sudo dnf install openssl-devel pkg-config
```

---

## Performance Issues

### Slow Responses

**Possible Causes**
- Large model running on CPU
- Many documents to search
- Network latency to Ollama

**Solutions**
1. Use smaller model:
   ```rust
   chat_model: "llama3.2:1b".to_string(),
   embed_model: "nomic-embed-text".to_string(),
   ```

2. Reduce max results:
   ```rust
   max_results: 3,
   ```

3. Use GPU for Ollama:
   ```bash
   OLLAMA_HOST=cuda:11434 ollama serve
   ```

---

## Health Check Issues

### Health Check Shows Degraded

**Solution**
1. Run health check manually:
   ```bash
   curl http://localhost:8081/api/health
   ```

2. Check Ollama:
   ```bash
   curl http://localhost:11434/api/tags
   ```

3. Check Qdrant:
   ```bash
   curl http://localhost:6333/collections
   ```

---

## Getting Help

If issues persist:

1. Check logs:
   ```bash
   RUST_LOG=debug ./target/release/aiden
   ```

2. Open an issue on GitLab/GitHub with:
   - AIDEN version (`./aiden --version` or check Cargo.toml)
   - Full error message
   - Steps to reproduce
   - System information (`uname -a`)
