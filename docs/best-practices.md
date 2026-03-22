# AIDEN Best Practices

Recommendations for running AIDEN in production.

## Table of Contents

1. [Deployment](#deployment)
2. [Security](#security)
3. [Performance](#performance)
4. [Reliability](#reliability)
5. [Maintenance](#maintenance)

---

## Deployment

### Use Systemd

Always use systemd for production deployments:

```ini
[Unit]
Description=AIDEN AI Assistant
After=network.target ollama.service qdrant.service
Wants=ollama.service qdrant.service

[Service]
Type=simple
User=aiden
WorkingDirectory=/opt/aiden
ExecStart=/opt/aiden/aiden
Restart=on-failure
RestartSec=10

[Install]
WantedBy=multi-user.target
```

### Directory Structure

```
/opt/aiden/
├── aiden              # Binary
├── docs/              # Documentation files
└── config/            # Future: config files

/var/lib/aiden/
├── cache/            # Application cache
└── data/              # Runtime data

/var/log/aiden/
└── aiden.log         # Application logs
```

### Separate Service Users

Create dedicated users for each service:

```bash
sudo useradd -r -s /bin/false ollama
sudo useradd -r -s /bin/false qdrant
sudo useradd -r -s /bin/false aiden
```

---

## Security

### Network Isolation

- Run services on localhost only
- Use reverse proxy for external access
- Implement firewall rules
- Use VPN for remote access

### Authentication

- Implement reverse proxy auth
- Use HTTPS/TLS
- Enable rate limiting
- Monitor for abuse

### Updates

- Subscribe to security advisories
- Test updates in staging first
- Maintain backups before updates
- Have rollback plan ready

---

## Performance

### GPU Acceleration

Always use GPU for Ollama:

```bash
# Verify GPU is being used
nvidia-smi
# Look for ollama process with GPU memory usage
```

### Resource Limits

Set appropriate limits:

```ini
[Service]
MemoryMax=4G
CPUQuota=200%
TasksMax=100
```

### Caching

Enable response caching:

```rust
// Example: Cache common embeddings
use std::collections::HashMap;
let embedding_cache: Arc<Mutex<HashMap<String, Vec<f32>>>> = 
    Arc::new(Mutex::new(HashMap::new()));
```

### Model Selection

Choose appropriate models:

| Use Case | Model | Size |
|----------|-------|------|
| Fast/Testing | llama3.2:1b | 1.3GB |
| Balanced | llama3.2:3b | 2.0GB |
| High Quality | llama3.1:8b | 4.9GB |

---

## Reliability

### Health Checks

Implement regular health checks:

```bash
#!/bin/bash
# healthcheck.sh

if curl -f http://localhost:8081/api/health | grep -q "healthy"; then
    exit 0
else
    exit 1
fi
```

### Auto-Restart

Configure automatic restarts:

```ini
[Service]
Restart=always
RestartSec=10
```

### Monitoring

Set up monitoring:

- CPU/Memory usage
- Response times
- Error rates
- Service health

### Backups

Regular backup schedule:

```bash
# Daily backup
0 2 * * * /usr/local/bin/backup-aiden.sh

# Weekly full backup
0 3 * * 0 tar -czf /var/backups/aiden-full-$(date +%Y%m%d).tar.gz /opt/aiden /var/lib/qdrant
```

---

## Maintenance

### Regular Tasks

| Task | Frequency |
|------|-----------|
| Log rotation | Daily |
| Backup | Daily |
| Security updates | Weekly |
| Full system backup | Weekly |
| Performance review | Monthly |
| Model updates | Monthly |

### Index Updates

Re-index when docs change:

```bash
curl -X POST http://localhost:8081/api/index
```

### Log Management

Configure logrotate:

```
/var/log/aiden/*.log {
    daily
    rotate 7
    compress
    delaycompress
    notifempty
    create 0644 aiden aiden
}
```

### Cleanup

```bash
# Remove old backups (keep 30 days)
find /var/backups/aiden -mtime +30 -delete

# Clean cache
rm -rf /var/lib/aiden/cache/*

# Vacuum logs
journalctl --vacuum-time=30d
```

---

## Documentation Best Practices

### Doc Structure

```
docs/
├── README.md              # Overview
├── getting-started.md     # Quick start
├── user-guide.md         # End user docs
├── admin-guide.md        # Admin docs
├── api-reference.md      # API docs
├── troubleshooting.md     # Common issues
└── faq.md               # FAQ
```

### Writing Docs

- Use clear headings
- Include code examples
- Add screenshots (if helpful)
- Keep updated
- Test instructions

---

## Incident Response

### Service Down

```bash
# 1. Check status
systemctl status ollama qdrant aiden

# 2. Check logs
journalctl -u aiden -n 50 --no-pager

# 3. Restart services
sudo systemctl restart ollama qdrant aiden

# 4. Verify
curl http://localhost:8081/api/health
```

### Performance Issues

```bash
# 1. Check resources
top
nvidia-smi

# 2. Check service logs
journalctl -u ollama -n 20

# 3. Check service status
systemctl status ollama qdrant aiden

# 4. Restart if needed
sudo systemctl restart ollama
```

### Data Corruption

```bash
# 1. Stop services
sudo systemctl stop aiden

# 2. Restore from backup
sudo rm -rf /var/lib/qdrant/*
sudo tar -xzf /var/backups/qdrant_latest.tar.gz -C /var/lib/qdrant

# 3. Re-index
curl -X POST http://localhost:8081/api/index

# 4. Verify
curl http://localhost:8081/api/index/status
```

---

## Security Checklist

- [ ] HTTPS/TLS enabled
- [ ] Authentication configured
- [ ] Rate limiting enabled
- [ ] Firewall configured
- [ ] Services running as non-root
- [ ] Regular security updates
- [ ] Backup verified
- [ ] Monitoring active
- [ ] Logs reviewed
- [ ] Access audited
