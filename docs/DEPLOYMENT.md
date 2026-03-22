# Deployment Guide

This guide covers various deployment scenarios for AIDEN.

## Prerequisites

Before deploying, ensure:
- Ollama is installed and running on port 11434
- Qdrant is installed and running on port 6334
- Required models are pulled: `llama3.2:1b` and `nomic-embed-text`
- Documentation files exist in `./docs/` directory

## Option 1: Systemd Service (Recommended for Servers)

### Create a Systemd Service

Create `/etc/systemd/system/aiden.service`:

```ini
[Unit]
Description=AIDEN - AcreetionOS AI Documentation Assistant
After=network.target ollama.service qdrant.service
Wants=ollama.service qdrant.service

[Service]
Type=simple
User=natalie
WorkingDirectory=/opt/aiden
ExecStart=/opt/aiden/aiden
Restart=on-failure
RestartSec=10
StandardOutput=journal
StandardError=journal
Environment="RUST_LOG=info"

[Install]
WantedBy=multi-user.target
```

### Install Steps

```bash
# Build the release binary
cd /path/to/aiden
cargo build --release

# Create installation directory
sudo mkdir -p /opt/aiden
sudo cp -r target/release/aiden docs /opt/aiden/
sudo chown -R natalie:natalie /opt/aiden

# Install service file
sudo cp aiden.service /etc/systemd/system/
sudo systemctl daemon-reload

# Enable and start
sudo systemctl enable aidensudo systemctl start aiden
# Check status
sudo systemctl status aiden
```

### Boot Considerations

AIDEN will start automatically at boot because:
- `WantedBy=multi-user.target` ensures it starts in multi-user mode
- `After=ollama.service qdrant.service` ensures dependencies start first
- `Restart=on-failure` provides automatic recovery

Make sure Ollama and Qdrant also start at boot:

```bash
# For Ollama (typically enabled by default)
sudo systemctl enable ollama

# For Qdrant (if running as service)
sudo systemctl enable qdrant
```

## Option 2: Docker Deployment

### Build the Image

```bash
docker build -t aidenaiden .
```

### Run the Container

```bash
docker run -d \
  --name aiden \
  -p 8081:8081 \
  -v $(pwd)/docs:/app/docs \
  --add-host=host.docker.internal:host-gateway \
  -e RUST_LOG=info \
  --restart unless-stopped \
  aiden
```

### Docker Compose

```yaml
version: '3.8'

services:
  aiden:
    build: .
    container_name: aiden
    ports:
      - "8081:8081"
    volumes:
      - ./docs:/app/docs
    environment:
      - RUST_LOG=info
    extra_hosts:
      - "host.docker.internal:host-gateway"
    restart: unless-stopped
    network_mode: host
```

## Option 3: Reverse Proxy with TLS

### Nginx Configuration

```nginx
server {
    listen 443 ssl http2;
    server_name aiden.acreetionos.org;

    ssl_certificate /etc/letsencrypt/live/aiden.acreetionos.org/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/aiden.acreetionos.org/privkey.pem;

    location / {
        proxy_pass http://127.0.0.1:8081;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_read_timeout 300s;
        proxy_connect_timeout 75s;
    }
}
```

### Caddy Configuration

```Caddyfile
aiden.acreetionos.org {
    reverse_proxy localhost:8081
    websocket
}
```

## Production Checklist

- [ ] Ollama installed and models pulled
- [ ] Qdrant installed and accessible
- [ ] Documentation indexed
- [ ] Firewall configured (ports 8081, 443)
- [ ] TLS/SSL certificates configured
- [ ] Authentication implemented (reverse proxy)
- [ ] Monitoring set up (health endpoint)
- [ ] Backup strategy for Qdrant data
- [ ] Log rotation configured
- [ ] Resource limits set (memory, CPU)

## Health Monitoring

Check health endpoint:
```bash
curl http://localhost:8081/api/health
```

Expected response:
```json
{
  "status": "healthy",
  "ollama": true,
  "qdrant": true
}
```

## Updating

### Systemd Service

```bash
cd /path/to/aiden
cargo build --release
sudo systemctl stop aiden
sudo cp target/release/aiden /opt/aiden/
sudo systemctl start aiden
```

### Docker

```bash
docker build -t aidenaiden .
docker stop aiden
docker rm aiden
docker run -d [options] aidenaiden
```
