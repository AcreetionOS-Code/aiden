# AIDEN Operations Guide

## System Administration

This guide covers operational aspects of running AIDEN in production environments.

## Table of Contents

1. [Service Management](#service-management)
2. [Monitoring](#monitoring)
3. [Backup and Recovery](#backup-and-recovery)
4. [Security](#security)
5. [Scaling](#scaling)
6. [Maintenance](#maintenance)
7. [Disaster Recovery](#disaster-recovery)
8. [Performance Tuning](#performance-tuning)

---

## Service Management

### Starting Services

Start all required services in order:

```bash
# 1. Start Ollama
sudo systemctl start ollama

# 2. Start Qdrant
sudo systemctl start qdrant

# 3. Start AIDEN
sudo systemctl start aiden
```

### Stopping Services

```bash
sudo systemctl stop aiden
sudo systemctl stop qdrant
sudo systemctl stop ollama
```

### Restarting Services

```bash
sudo systemctl restart ollama
sudo systemctl restart qdrant
sudo systemctl restart aiden
```

### Checking Status

```bash
# All services
systemctl status ollama qdrant aiden

# Individual service
systemctl status aiden
```

### Service Dependencies

```
network.target
    └── ollama.service
    └── qdrant.service
        └── aiden.service
```

---

## Monitoring

### Health Endpoint

```bash
curl http://localhost:8081/api/health
```

Response:
```json
{
  "status": "healthy",
  "ollama": true,
  "qdrant": true
}
```

### Prometheus Metrics

Add metrics endpoint to `main.rs`:

```rust
.route("/metrics", get(metrics_handler))
```

### System Monitoring

#### CPU Usage
```bash
top -p $(pgrep aiden)
```

#### Memory Usage
```bash
ps aux | grep aiden
```

#### GPU Usage
```bash
nvidia-smi -l 1
```

### Log Monitoring

#### AIDEN Logs
```bash
journalctl -u aiden -f
```

#### Ollama Logs
```bash
journalctl -u ollama -f
```

#### Qdrant Logs
```bash
journalctl -u qdrant -f
```

### Setting Up Alerts

Create `/etc/systemd/system/aiden-healthcheck.timer`:

```ini
[Unit]
Description=AIDEN Health Check Timer

[Timer]
OnBootSec=1min
OnUnitActiveSec=5min

[Install]
WantedBy=timers.target
```

Create `/etc/systemd/system/aiden-healthcheck.service`:

```ini
[Unit]
Description=AIDEN Health Check

[Service]
Type=oneshot
ExecStart=/usr/bin/curl -f http://localhost:8081/api/health
```

---

## Backup and Recovery

### Backup Script

Create `/usr/local/bin/backup-aiden.sh`:

```bash
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/var/backups/aiden"
mkdir -p $BACKUP_DIR

# Backup Qdrant data
tar -czf $BACKUP_DIR/qdrant_$DATE.tar.gz /var/lib/qdrant

# Backup AIDEN docs
tar -czf $BACKUP_DIR/docs_$DATE.tar.gz /opt/aiden/docs

# Backup configuration
cp /etc/systemd/system/aiden.service $BACKUP_DIR/

# Keep only last 7 backups
find $BACKUP_DIR -name "*.tar.gz" -mtime +7 -delete

echo "Backup completed: $DATE"
```

Make executable:
```bash
chmod +x /usr/local/bin/backup-aiden.sh
```

### Restore from Backup

```bash
# Stop services
sudo systemctl stop aiden

# Restore Qdrant
sudo tar -xzf /var/backups/aiden/qdrant_YYYYMMDD_HHMMSS.tar.gz -C /

# Restore docs
sudo tar -xzf /var/backups/aiden/docs_YYYYMMDD_HHMMSS.tar.gz -C /

# Restart services
sudo systemctl start qdrant
sudo systemctl start aiden
```

### Automated Backups

Create cron job:
```bash
sudo crontab -e
```

Add:
```
0 2 * * * /usr/local/bin/backup-aiden.sh
```

---

## Security

### Firewall Configuration

```bash
# Allow local access
sudo ufw allow from 127.0.0.1 to any port 8081

# Allow local network
sudo ufw allow from 192.168.0.0/24 to any port 8081

# Deny external access (if using reverse proxy)
sudo ufw deny 8081
```

### Reverse Proxy with Auth

#### Nginx with Basic Auth

```nginx
server {
    listen 443 ssl;
    server_name aiden.example.com;

    ssl_certificate /etc/ssl/certs/aiden.crt;
    ssl_certificate_key /etc/ssl/private/aiden.key;

    auth_basic "Restricted Access";
    auth_basic_user_file /etc/nginx/.htpasswd;

    location / {
        proxy_pass http://127.0.0.1:8081;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
    }
}
```

Generate password:
```bash
sudo htpasswd -c /etc/nginx/.htpasswd username
```

### Rate Limiting

```nginx
limit_req_zone $binary_remote_addr zone=aiden_limit:10m rate=10r/s;

location / {
    limit_req zone=aiden_limit burst=20 nodelay;
    proxy_pass http://127.0.0.1:8081;
}
```

### TLS Configuration

Use Let's Encrypt:
```bash
sudo certbot --nginx -d aiden.example.com
```

---

## Scaling

### Load Balancing

Add multiple AIDEN instances behind nginx:

```nginx
upstream aiden_backend {
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
    server 127.0.0.1:8083;
}

server {
    listen 443 ssl;
    
    location / {
        proxy_pass http://aiden_backend;
    }
}
```

### Horizontal Scaling Considerations

- Each instance needs its own Qdrant connection
- Consider Qdrant cluster for shared vector store
- Use Redis for session state if needed
- Ollama should be scaled separately

---

## Maintenance

### Regular Maintenance Tasks

| Task | Frequency | Command |
|------|-----------|---------|
| Log rotation | Daily | `logrotate -f /etc/logrotate.conf` |
| Disk cleanup | Weekly | `docker system prune -a` |
| Security updates | Monthly | `sudo pacman -Syu` |
| Full backup | Weekly | `/usr/local/bin/backup-aiden.sh` |
| Model updates | As needed | `ollama pull modelname` |

### Log Rotation

Create `/etc/logrotate.d/aiden`:

```
/var/log/aiden/*.log {
    daily
    rotate 7
    compress
    delaycompress
    notifempty
    create 0644 root root
    postrotate
        systemctl reload aiden > /dev/null 2>&1 || true
    endscript
}
```

### Update Procedure

1. Stop service:
   ```bash
   sudo systemctl stop aiden
   ```

2. Update code:
   ```bash
   cd /path/to/aiden
   git pull
   cargo build --release
   ```

3. Update binary:
   ```bash
   sudo cp target/release/aiden /opt/aiden/
   ```

4. Start service:
   ```bash
   sudo systemctl start aiden
   ```

---

## Disaster Recovery

### Recovery Plan

1. **Assess Damage**
   - Check which services are affected
   - Evaluate data loss
   - Determine recovery time objective (RTO)

2. **Restore Services**
   ```bash
   sudo systemctl start qdrant
   sudo systemctl start ollama
   sudo systemctl start aiden
   ```

3. **Verify Functionality**
   ```bash
   curl http://localhost:8081/api/health
   ```

4. **Restore Data**
   ```bash
   # From latest backup
   sudo tar -xzf /var/backups/aiden/qdrant_LATEST.tar.gz -C /
   ```

### Service Failover

For high availability, set up failover:

```bash
# On primary
sudo systemctl stop aiden

# On secondary
sudo systemctl start aiden
```

---

## Performance Tuning

### Ollama Optimization

Edit `/etc/systemd/system/ollama.service.d/override.conf`:

```ini
[Service]
Environment="OLLAMA_NUM_PARALLEL=4"
Environment="OLLAMA_MAX_LOADED_MODELS=2"
```

### Qdrant Optimization

Create `/etc/qdrant/config.yaml`:

```yaml
storage:
  optimizer:
    memmap_threshold_kb: 200
  hnsw_index:
    m: 16
    ef_construct: 200
```

### AIDEN Optimization

Increase worker threads in `main.rs`:

```rust
let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await?;
axum::serve(listener, app)
    .serve_with_graceful_shutdown(/* ... */)
    .await;
```

### System Limits

Edit `/etc/security/limits.conf`:

```
root soft nofile 65536
root hard nofile 65536
```

---

## Capacity Planning

### Resource Requirements

| Users | CPU Cores | RAM | VRAM |
|-------|-----------|-----|------|
| 1-5 | 4 | 8GB | 4GB |
| 5-20 | 8 | 16GB | 6GB |
| 20-50 | 16 | 32GB | 8GB |

### Monitoring Resource Usage

```bash
# Watch system resources
watch -n 1 'free -h; nvidia-smi'

# Check service limits
cat /proc/$(pgrep aiden)/limits
```
