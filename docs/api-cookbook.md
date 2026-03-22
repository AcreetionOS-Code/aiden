# AIDEN API Cookbook

Practical examples for integrating with AIDEN's API.

## Table of Contents

1. [Authentication](#authentication)
2. [Chat Integration](#chat-integration)
3. [WebSocket Streaming](#websocket-streaming)
4. [Indexing Management](#indexing-management)
5. [Health Monitoring](#health-monitoring)
6. [Custom Frontends](#custom-frontends)
7. [Webhook Integration](#webhook-integration)
8. [Bot Integration](#bot-integration)

---

## Authentication

### No Built-in Auth (Default)

AIDEN has no built-in authentication. Use reverse proxy authentication.

### Nginx Basic Auth

```nginx
server {
    listen 443 ssl;
    server_name aiden.example.com;

    auth_basic "AIDEN Access";
    auth_basic_user_file /etc/nginx/.htpasswd;

    location / {
        proxy_pass http://127.0.0.1:8081;
    }
}
```

### JWT Token Auth (Custom Implementation)

Create middleware in `src/middleware/auth.rs`:

```rust
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

pub async fn auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match token {
        Some(t) if validate_token(t) => Ok(next.run(request).await),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn validate_token(token: &str) -> bool {
    // Implement your token validation
    token == "your-secret-token"
}
```

---

## Chat Integration

### Simple HTTP Chat

```bash
curl -X POST http://localhost:8081/api/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "What is AcreetionOS?"}'
```

### Python Example

```python
import requests
import json

def chat(message: str) -> dict:
    response = requests.post(
        "http://localhost:8081/api/chat",
        json={"message": message},
        timeout=120
    )
    return response.json()

# Usage
result = chat("How do I install packages?")
print(result["response"]["content"])
```

### JavaScript/Node.js Example

```javascript
async function chat(message) {
    const response = await fetch('http://localhost:8081/api/chat', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({message})
    });
    return await response.json();
}

// Usage
const result = await chat('What is the firewall command?');
console.log(result.response.content);
```

### Go Example

```go
package main

import (
    "bytes"
    "encoding/json"
    "fmt"
    "net/http"
)

type ChatRequest struct {
    Message string `json:"message"`
}

type ChatResponse struct {
    Response struct {
        Content string `json:"content"`
        Sources []struct {
            Title string `json:"title"`
            Score float64 `json:"score"`
        } `json:"sources"`
    } `json:"response"`
}

func chat(message string) (*ChatResponse, error) {
    reqBody := ChatRequest{Message: message}
    jsonData, _ := json.Marshal(reqBody)
    
    resp, err := http.Post(
        "http://localhost:8081/api/chat",
        "application/json",
        bytes.NewBuffer(jsonData),
    )
    if err != nil {
        return nil, err
    }
    defer resp.Body.Close()
    
    var result ChatResponse
    json.NewDecoder(resp.Body).Decode(&result)
    return &result, nil
}

func main() {
    result, _ := chat("How do I update the system?")
    fmt.Println(result.Response.Content)
}
```

---

## WebSocket Streaming

### JavaScript WebSocket Client

```javascript
const ws = new WebSocket('ws://localhost:8081/api/chat/stream');

ws.onopen = () => {
    ws.send(JSON.stringify({
        type: 'chat',
        message: 'Explain the package manager'
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    
    switch(data.type) {
        case 'status':
            console.log('Status:', data.status);
            break;
        case 'sources':
            console.log('Sources found:', data.sources.length);
            break;
        case 'response':
            process.stdout.write(data.content);
            if (!data.is_partial) {
                console.log('\n--- Response complete ---');
            }
            break;
        case 'done':
            ws.close();
            break;
    }
};

ws.onerror = (error) => {
    console.error('WebSocket error:', error);
};
```

### Python WebSocket Client

```python
import asyncio
import websockets
import json

async def chat_stream(message: str):
    uri = "ws://localhost:8081/api/chat/stream"
    
    async with websockets.connect(uri) as ws:
        await ws.send(json.dumps({
            "type": "chat",
            "message": message
        }))
        
        async for msg in ws:
            data = json.loads(msg)
            
            if data["type"] == "response":
                print(data["content"], end="", flush=True)
                if not data.get("is_partial", False):
                    break
            elif data["type"] == "sources":
                print(f"\n[SOURCES: {len(data['sources'])}]")

asyncio.run(chat_stream("What are the security features?"))
```

### Testing with wscat

```bash
# Install wscat
npm install -g wscat

# Connect and chat
wscat -c ws://localhost:8081/api/chat/stream

# Send message
{"type":"chat","message":"Hello"}
```

---

## Indexing Management

### Trigger Indexing

```bash
curl -X POST http://localhost:8081/api/index
```

### Check Index Status

```bash
curl http://localhost:8081/api/index/status
```

Response:
```json
{
  "is_indexing": false,
  "files_processed": 12,
  "chunks_created": 342,
  "errors": []
}
```

### Python Indexing Manager

```python
import requests
import time

class IndexManager:
    def __init__(self, base_url="http://localhost:8081"):
        self.base_url = base_url
    
    def start_indexing(self):
        response = requests.post(f"{self.base_url}/api/index")
        return response.json()
    
    def get_status(self):
        response = requests.get(f"{self.base_url}/api/index/status")
        return response.json()
    
    def wait_for_completion(self, timeout=300):
        start = time.time()
        while time.time() - start < timeout:
            status = self.get_status()
            if not status['is_indexing']:
                return status
            print(f"Progress: {status['files_processed']} files, "
                  f"{status['chunks_created']} chunks")
            time.sleep(5)
        raise TimeoutError("Indexing took too long")

# Usage
manager = IndexManager()
manager.start_indexing()
result = manager.wait_for_completion()
print(f"Indexed {result['files_processed']} files")
```

---

## Health Monitoring

### Health Check Script

```bash
#!/bin/bash
HEALTH=$(curl -s http://localhost:8081/api/health)
OLLAMA=$(echo $HEALTH | jq -r '.ollama')
QDRANT=$(echo $HEALTH | jq -r '.qdrant')
STATUS=$(echo $HEALTH | jq -r '.status')

if [ "$STATUS" = "healthy" ]; then
    echo "OK: All services healthy"
    exit 0
else
    echo "WARN: Status is $STATUS"
    [ "$OLLAMA" = "false" ] && echo "  - Ollama is down"
    [ "$QDRANT" = "false" ] && echo "  - Qdrant is down"
    exit 1
fi
```

### Prometheus Exporter

Add to `main.rs`:

```rust
use axum::{
    extract::State,
    response::Json,
    routing::get,
    Router,
};
use std::sync::Arc;

async fn metrics(State(state): State<Arc<AppState>>) -> String {
    let health = state.check_health().await;
    
    format!(
        r#"# HELP aidenaiden_up Is AIDEN up
# TYPE aidenaiden_up gauge
aidenaiden_up {}

# HELP aidenaiden_ollama_up Is Ollama up
# TYPE aidenaiden_ollama_up gauge
aidenaiden_ollama_up {}

# HELP aidenaiden_qdrant_up Is Qdrant up
# TYPE aidenaiden_qdrant_up gauge
aidenaiden_qdrant_up {}
"#,
        if health.status == "healthy" { 1 } else { 0 },
        if health.ollama { 1 } else { 0 },
        if health.qdrant { 1 } else { 0 },
    )
}

fn app() -> Router {
    Router::new()
        .route("/metrics", get(metrics))
        // ... other routes
}
```

---

## Custom Frontends

### React Component

```tsx
import React, { useState } from 'react';

function AIDENChat() {
  const [messages, setMessages] = useState([]);
  const [input, setInput] = useState('');
  const [loading, setLoading] = useState(false);

  const sendMessage = async () => {
    if (!input.trim()) return;
    
    const userMessage = { role: 'user', content: input };
    setMessages(prev => [...prev, userMessage]);
    setInput('');
    setLoading(true);

    try {
      const response = await fetch('http://localhost:8081/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: input })
      });
      
      const data = await response.json();
      setMessages(prev => [...prev, data.response]);
    } catch (error) {
      console.error('Error:', error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="chat-container">
      <div className="messages">
        {messages.map((msg, i) => (
          <div key={i} className={`message ${msg.role}`}>
            {msg.content}
          </div>
        ))}
      </div>
      <div className="input-area">
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && sendMessage()}
        />
        <button onClick={sendMessage} disabled={loading}>
          {loading ? 'Thinking...' : 'Send'}
        </button>
      </div>
    </div>
  );
}
```

### Simple HTML/JavaScript

```html
<!DOCTYPE html>
<html>
<head>
    <title>AIDEN Chat</title>
    <style>
        #chat { height: 400px; overflow-y: auto; border: 1px solid #ccc; padding: 10px; }
        .user { color: blue; }
        .assistant { color: green; }
    </style>
</head>
<body>
    <h1>AIDEN Chat</h1>
    <div id="chat"></div>
    <input type="text" id="message" placeholder="Ask a question..." />
    <button onclick="send()">Send</button>

    <script>
        async function send() {
            const msg = document.getElementById('message').value;
            document.getElementById('message').value = '';
            
            // Add user message
            const chat = document.getElementById('chat');
            chat.innerHTML += `<div class="user">You: ${msg}</div>`;
            
            // Get response
            const res = await fetch('http://localhost:8081/api/chat', {
                method: 'POST',
                headers: {'Content-Type': 'application/json'},
                body: JSON.stringify({message: msg})
            });
            const data = await res.json();
            
            chat.innerHTML += `<div class="assistant">AIDEN: ${data.response.content}</div>`;
        }
    </script>
</body>
</html>
```

---

## Webhook Integration

### Outgoing Webhooks

Create webhook handler in `handlers.rs`:

```rust
async fn send_webhook(url: &str, payload: &str) -> Result<()> {
    let client = reqwest::Client::new();
    client.post(url)
        .json(&serde_json::json!({
            "event": "chat_completed",
            "data": payload
        }))
        .send()
        .await?;
    Ok(())
}
```

### Incoming Webhooks

```rust
pub async fn webhook_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<WebhookPayload>,
) -> Result<Json<WebhookResponse>, StatusCode> {
    match payload.event.as_str() {
        "index_requested" => {
            // Trigger indexing
            Ok(Json(WebhookResponse { status: "indexing_started" }))
        }
        "clear_cache" => {
            // Clear caches
            Ok(Json(WebhookResponse { status: "cache_cleared" }))
        }
        _ => Err(StatusCode::BAD_REQUEST),
    }
}
```

---

## Bot Integration

### Discord Bot

```python
import discord
import requests

intents = discord.Intents.default()
client = discord.Client(intents=intents)

@client.event
async def on_message(message):
    if message.author == client.user:
        return
    
    if client.user in message.mentions:
        question = message.content.replace(f'<@!{client.user.id}>', '')
        
        response = requests.post(
            "http://localhost:8081/api/chat",
            json={"message": question}
        ).json()
        
        await message.reply(response['response']['content'])

client.run('YOUR_DISCORD_TOKEN')
```

### Slack Bot

```python
from slack_sdk import WebClient
from slack_sdk.socket_mode import SocketModeHandler
import requests

slack_token = 'YOUR_SLACK_TOKEN'
client = WebClient(token=slack_token)

def handle_message(event):
    if 'text' in event and 'AIDEN' in event['text']:
        question = event['text'].replace('AIDEN', '').strip()
        
        response = requests.post(
            "http://localhost:8081/api/chat",
            json={"message": question}
        ).json()
        
        client.chat_postMessage(
            channel=event['channel'],
            text=response['response']['content']
        )

# Run socket mode handler
handler = SocketModeHandler(client, "YOUR_APP_TOKEN")
handler.start()
```

---

## Rate Limiting

### Client-side Rate Limiting

```python
import time
import threading

class RateLimiter:
    def __init__(self, calls: int, period: float):
        self.calls = calls
        self.period = period
        self.last_reset = time.time()
        self.count = 0
        self.lock = threading.Lock()
    
    def wait(self):
        with self.lock:
            now = time.time()
            if now - self.last_reset >= self.period:
                self.last_reset = now
                self.count = 0
            
            if self.count >= self.calls:
                sleep_time = self.period - (now - self.last_reset)
                time.sleep(sleep_time)
                self.last_reset = time.time()
                self.count = 0
            
            self.count += 1

limiter = RateLimiter(calls=10, period=60)  # 10 calls per minute

def chat(message: str):
    limiter.wait()
    return requests.post('http://localhost:8081/api/chat', 
                         json={'message': message}).json()
```
