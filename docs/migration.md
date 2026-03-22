# AIDEN Migration Guide

Upgrading and migrating AIDEN deployments.

## Table of Contents

1. [Version Upgrades](#version-upgrades)
2. [Model Updates](#model-updates)
3. [Data Migration](#data-migration)
4. [Configuration Changes](#configuration-changes)
5. [Breaking Changes](#breaking-changes)

---

## Version Upgrades

### Upgrading AIDEN

```bash
# 1. Stop the service
sudo systemctl stop aiden

# 2. Backup current installation
sudo cp -r /opt/aiden /opt/aiden.backup

# 3. Pull latest code
cd /path/to/aiden
git pull

# 4. Rebuild
cargo build --release

# 5. Update binary
sudo cp target/release/aiden /opt/aiden/

# 6. Start service
sudo systemctl start aiden

# 7. Verify
curl http://localhost:8081/api/health
```

### Upgrading Ollama

```bash
# 1. Stop Ollama
sudo systemctl stop ollama

# 2. Update
# (varies by installation method)
curl -fsSL https://ollama.com/install.sh | sh

# 3. Start Ollama
sudo systemctl start ollama

# 4. Re-pull models (may be needed)
ollama pull llama3.2:1b
ollama pull nomic-embed-text
```

### Upgrading Qdrant

```bash
# 1. Stop Qdrant
sudo systemctl stop qdrant

# 2. Backup data
sudo tar -czf qdrant_backup_$(date +%Y%m%d).tar.gz /var/lib/qdrant

# 3. Update binary
# Download latest from https://github.com/qdrant/qdrant/releases
wget https://github.com/qdrant/qdrant/releases/download/v1.17.0/qdrant-x86_64-unknown-linux-gnu.tar.gz
sudo tar -xzf qdrant-x86_64-unknown-linux-gnu.tar.gz -C /usr/local/bin/

# 4. Start Qdrant
sudo systemctl start qdrant
```

---

## Model Updates

### Updating Chat Model

```bash
# List current models
ollama list

# Pull new model
ollama pull llama3.2:3b

# Update AIDEN config in src/state.rs
# chat_model: "llama3.2:3b".to_string()

# Rebuild AIDEN
cargo build --release

# Restart service
sudo systemctl restart aiden
```

### Updating Embedding Model

```bash
# Pull new embedding model
ollama pull all-MiniLM-L6-v2

# Update AIDEN config
# embed_model: "all-MiniLM-L6-v2".to_string()

# IMPORTANT: Re-index all documents
# (Embedding dimension may have changed)
curl -X POST http://localhost:8081/api/index
```

### Testing New Model

```bash
# Test with simple query
curl -X POST http://localhost:8081/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message":"Hello"}'
```

---

## Data Migration

### Backing Up Qdrant

```bash
# Create backup directory
mkdir -p /var/backups/qdrant

# Backup collection
curl -X POST "http://localhost:6333/collections/aiden/points/snapshot" \
  -H "Content-Type: application/json"

# Download snapshot
curl -O "http://localhost:6333/collections/aiden/points/snapshots/$(snapshot_name)"
```

### Restoring Qdrant

```bash
# Stop Qdrant
sudo systemctl stop qdrant

# Restore from backup
sudo tar -xzf /var/backups/qdrant/backup.tar.gz -C /

# Clear existing data (if needed)
sudo rm -rf /var/lib/qdrant/*
sudo tar -xzf /var/backups/qdrant/latest_backup.tar.gz -C /var/lib/qdrant

# Start Qdrant
sudo systemctl start qdrant
```

### Migrating to New Server

```bash
# On old server
sudo systemctl stop aiden qdrant ollama
tar -czf aiden_migration.tar.gz /opt/aiden /var/lib/qdrant /home/natalie2/Projects/aiden

# On new server
tar -xzf aiden_migration.tar.gz
sudo systemctl start qdrant ollama aiden
```

---

## Configuration Changes

### Environment Variables (Future)

When environment variable support is added:

```bash
# Ollama config
OLLAMA_HOST=localhost:11434
OLLAMA_MODEL=llama3.2:1b

# Qdrant config
QDRANT_HOST=localhost:6334
QDRANT_COLLECTION=aiden

# AIDEN config
AIDEN_PORT=8081
AIDEN_LOG_LEVEL=info
```

### Config File (Future)

When TOML/YAML config is supported:

```toml
# config.toml

[ollama]
host = "localhost:11434"
chat_model = "llama3.2:1b"
embed_model = "nomic-embed-text"

[search]
threshold = 0.7
max_results = 5

[indexing]
docs_path = "./docs"
chunk_size = 512
chunk_overlap = 50
```

---

## Breaking Changes

### v0.1.0 to v0.2.0 (Hypothetical)

If breaking changes are introduced:

### API Changes

```bash
# Old API
curl -X POST http://localhost:8081/api/ask \
  -d '{"question": "..."}'

# New API
curl -X POST http://localhost:8081/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "..."}'
```

### Database Schema Changes

If Qdrant collection schema changes:

```bash
# Backup old collection
curl -X POST "http://localhost:6333/collections/aiden/points/snapshot"

# Create new collection with new schema
curl -X PUT "http://localhost:6333/collections/aiden_v2" \
  -H "Content-Type: application/json" \
  -d '{
    "vectors": {
      "size": 1024,
      "distance": "Cosine"
    }
  }'

# Re-index with new config
# Update config to use new collection
```

### Migration Checklist

- [ ] Backup all data
- [ ] Review breaking changes documentation
- [ ] Test in development environment
- [ ] Update configuration
- [ ] Rebuild AIDEN
- [ ] Re-index documents
- [ ] Verify all endpoints work
- [ ] Monitor for errors

---

## Rollback Procedures

### Quick Rollback

```bash
# 1. Stop current version
sudo systemctl stop aiden

# 2. Restore backup
sudo cp /opt/aiden.backup/aiden /opt/aiden/

# 3. Start previous version
sudo systemctl start aiden

# 4. Verify
curl http://localhost:8081/api/health
```

### Full System Rollback

```bash
# 1. Stop all services
sudo systemctl stop aiden qdrant ollama

# 2. Restore from full backup
sudo rm -rf /opt/aiden /var/lib/qdrant
sudo tar -xzf /var/backups/full_backup_previous.tar.gz -C /

# 3. Start services
sudo systemctl start qdrant ollama aiden

# 4. Verify
curl http://localhost:8081/api/health
```
