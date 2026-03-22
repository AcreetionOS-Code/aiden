# API Reference

Base URL: `http://localhost:8081`

## Endpoints

### GET /

Serves the web interface.

**Response**
- Content-Type: `text/html`
- Returns: Complete HTML page with embedded CSS/JS

---

### POST /api/chat

Send a chat message and receive a non-streaming response.

**Request**
```json
{
  "message": "What is AcreetionOS?"
}
```

**Response** `200 OK`
```json
{
  "response": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "role": "assistant",
    "content": "AcreetionOS is a privacy-focused operating system...",
    "sources": [
      {
        "source_type": "local",
        "title": "README.md",
        "url": "docs/README.md",
        "content": "AcreetionOS is a privacy-focused OS...",
        "score": 0.87
      }
    ],
    "used_web_fallback": false,
    "timestamp": "2026-03-19T10:30:00Z"
  }
}
```

**Error Response** `500 Internal Server Error`
```json
{
  "error": "Failed to process request"
}
```

---

### WebSocket /api/chat/stream

Streaming chat via WebSocket.

**Connection**
```
ws://localhost:8081/api/chat/stream
```

**Client → Server Messages**

Chat request:
```json
{
  "type": "chat",
  "message": "How do I install AcreetionOS?"
}
```

**Server → Client Messages**

Status update:
```json
{
  "type": "status",
  "status": "searching",
  "message": "Searching documentation..."
}
```

Sources available:
```json
{
  "type": "sources",
  "sources": [
    {
      "source_type": "local",
      "title": "installation.md",
      "url": "docs/installation.md",
      "content": "Step-by-step installation guide...",
      "score": 0.92
    }
  ]
}
```

Response chunk:
```json
{
  "type": "response",
  "content": "To install AcreetionOS",
  "is_partial": true
}
```

Final response:
```json
{
  "type": "response",
  "content": "To install AcreetionOS, follow these steps...",
  "is_partial": false
}
```

Completion:
```json
{
  "type": "done"
}
```

**Connection Close**
- Normal: Server sends `{ "type": "done" }`
- Error: Server closes connection

---

### POST /api/index

Trigger documentation indexing. Runs in background.

**Request**
```
(empty body)
```

**Response** `200 OK`
```json
{
  "status": "started",
  "message": "Indexing started in background"
}
```

**Error Response** `500 Internal Server Error`
```json
{
  "error": "Failed to start indexing"
}
```

---

### GET /api/index/status

Get current indexing progress.

**Response** `200 OK`

When indexing in progress:
```json
{
  "is_indexing": true,
  "files_processed": 5,
  "chunks_created": 127,
  "errors": []
}
```

When complete:
```json
{
  "is_indexing": false,
  "files_processed": 12,
  "chunks_created": 342,
  "errors": []
}
```

When never run:
```json
{
  "is_indexing": false,
  "files_processed": 0,
  "chunks_created": 0,
  "errors": []
}
```

---

### GET /api/health

Check service health status.

**Response** `200 OK`

All services healthy:
```json
{
  "status": "healthy",
  "ollama": true,
  "qdrant": true
}
```

Qdrant offline:
```json
{
  "status": "degraded",
  "ollama": true,
  "qdrant": false
}
```

Ollama offline:
```json
{
  "status": "degraded",
  "ollama": false,
  "qdrant": true
}
```

All services offline:
```json
{
  "status": "degraded",
  "ollama": false,
  "qdrant": false
}
```

---

## Data Types

### ChatMessage
```typescript
interface ChatMessage {
  id: string;              // UUID v4
  role: "user" | "assistant";
  content: string;
  sources: Source[];
  used_web_fallback: boolean;
  timestamp: string;        // RFC3339 format
}
```

### Source
```typescript
interface Source {
  source_type: "local" | "web";
  title: string;
  url: string;
  content: string;         // Snippet from document
  score: number;           // 0.0 to 1.0
}
```

### IndexingStatus
```typescript
interface IndexingStatus {
  is_indexing: boolean;
  files_processed: number;
  chunks_created: number;
  errors: string[];
}
```

### HealthResponse
```typescript
interface HealthResponse {
  status: "healthy" | "degraded";
  ollama: boolean;
  qdrant: boolean;
}
```

---

## Rate Limits

No built-in rate limiting. Implement at reverse proxy level.

---

## Authentication

No built-in authentication. Protect endpoints at reverse proxy level.

---

## Error Codes

| HTTP Status | Meaning |
|-------------|---------|
| 200 | Success |
| 400 | Bad Request - Invalid JSON |
| 500 | Internal Server Error |
| 500 | Ollama/Qdrant connection failure |
| 500 | Indexing failure |
