# AIDEN Testing Guide

Comprehensive testing strategies for AIDEN.

## Table of Contents

1. [Testing Overview](#testing-overview)
2. [Unit Tests](#unit-tests)
3. [Integration Tests](#integration-tests)
4. [API Tests](#api-tests)
5. [Performance Tests](#performance-tests)
6. [E2E Tests](#e2e-tests)
7. [CI/CD Integration](#cicd-integration)

---

## Testing Overview

### Test Types

| Type | Purpose | Speed |
|------|---------|-------|
| Unit | Test individual functions | Fast |
| Integration | Test service connections | Medium |
| API | Test HTTP endpoints | Medium |
| Performance | Measure speed/throughput | Slow |
| E2E | Test full user flows | Slowest |

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name

# With coverage
cargo tarpaulin
```

---

## Unit Tests

### Example: Handler Tests

```rust
// tests/handlers_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = Router::new()
            .route("/api/health", get(health));
        
        let response = apponeshot(Request::builder()
            .uri("/api/health")
            .body(Body::empty())
            .unwrap())
        .await
        .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_chat_endpoint() {
        let app = Router::new()
            .route("/api/chat", post(chat));
        
        let body = serde_json::json!({
            "message": "Hello"
        });
        
        let response = apponeshot(Request::builder()
            .uri("/api/chat")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap())
        .await
        .unwrap();
        
        assert_eq!(response.status(), StatusCode::OK);
    }
}
```

### Example: Service Tests

```rust
// tests/services_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ollama_service_health() {
        let service = OllamaService::new("localhost:11434");
        let is_healthy = service.health_check().await.unwrap();
        
        // Only pass if Ollama is running
        assert!(is_healthy);
    }

    #[tokio::test]
    async fn test_ollama_embeddings() {
        let service = OllamaService::new("localhost:11434");
        
        let embeddings = service
            .embeddings("nomic-embed-text", "test text")
            .await
            .unwrap();
        
        assert!(!embeddings.is_empty());
        assert_eq!(embeddings.len(), 768); // nomic-embed-text dimension
    }
}
```

---

## Integration Tests

### Example: Full RAG Pipeline

```rust
// tests/rag_integration_test.rs

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_env() -> TestAppState {
        let ollama = OllamaService::new("localhost:11434");
        let qdrant = QdrantService::new("aiden_test").await.unwrap();
        
        AppState::new(ollama, qdrant)
    }

    #[tokio::test]
    async fn test_full_rag_pipeline() {
        let state = setup_test_env().await;
        
        // Index a test document
        state.indexer.index_documents(&[TestDoc {
            title: "Test".to_string(),
            content: "AcreetionOS is a privacy-focused OS.".to_string(),
        }]).await.unwrap();
        
        // Query
        let response = state.rag.query("What is AcreetionOS?").await.unwrap();
        
        assert!(response.contains("privacy-focused"));
    }

    struct TestDoc {
        title: String,
        content: String,
    }
}
```

---

## API Tests

### Using curl

```bash
# test-api.sh

BASE_URL="http://localhost:8081"

echo "Testing Health Endpoint"
curl -s "$BASE_URL/api/health" | jq '.'

echo ""
echo "Testing Chat Endpoint"
curl -s -X POST "$BASE_URL/api/chat" \
  -H "Content-Type: application/json" \
  -d '{"message":"What is AcreetionOS?"}' \
  | jq '.'

echo ""
echo "Testing Index Endpoint"
curl -s -X POST "$BASE_URL/api/index"
```

### Python Test Suite

```python
# tests/test_api.py

import pytest
import requests
import time

BASE_URL = "http://localhost:8081"

class TestHealth:
    def test_health_returns_200(self):
        response = requests.get(f"{BASE_URL}/api/health")
        assert response.status_code == 200
    
    def test_health_structure(self):
        response = requests.get(f"{BASE_URL}/api/health").json()
        assert "status" in response
        assert "ollama" in response
        assert "qdrant" in response

class TestChat:
    def test_chat_returns_response(self):
        response = requests.post(
            f"{BASE_URL}/api/chat",
            json={"message": "Hello"},
            timeout=120
        )
        assert response.status_code == 200
        data = response.json()
        assert "response" in data
        assert "content" in data["response"]
    
    def test_chat_has_sources(self):
        response = requests.post(
            f"{BASE_URL}/api/chat",
            json={"message": "What is the OS?"},
            timeout=120
        )
        data = response.json()
        assert "sources" in data["response"]

class TestIndexing:
    def test_index_trigger(self):
        response = requests.post(f"{BASE_URL}/api/index")
        assert response.status_code == 200
    
    def test_index_status(self):
        response = requests.get(f"{BASE_URL}/api/index/status")
        assert response.status_code == 200
        data = response.json()
        assert "files_processed" in data
```

---

## Performance Tests

### Load Testing with hey

```bash
# Install hey
go install github.com/rakyll/hey@latest

# Basic load test
hey -n 1000 -c 10 -m POST \
  -H "Content-Type: application/json" \
  -d '{"message":"What is AcreetionOS?"}' \
  http://localhost:8081/api/chat
```

### Python Benchmark

```python
# tests/benchmark.py

import time
import requests
import statistics

def benchmark_endpoint(url, payload, iterations=100):
    times = []
    
    for _ in range(iterations):
        start = time.time()
        requests.post(url, json=payload, timeout=120)
        elapsed = time.time() - start
        times.append(elapsed)
    
    return {
        "mean": statistics.mean(times),
        "median": statistics.median(times),
        "stdev": statistics.stdev(times) if len(times) > 1 else 0,
        "min": min(times),
        "max": max(times),
    }

if __name__ == "__main__":
    results = benchmark_endpoint(
        "http://localhost:8081/api/chat",
        {"message": "What is AcreetionOS?"},
        iterations=50
    )
    
    print(f"Mean: {results['mean']:.2f}s")
    print(f"Median: {results['median']:.2f}s")
    print(f"StdDev: {results['stdev']:.2f}s")
    print(f"Min: {results['min']:.2f}s")
    print(f"Max: {results['max']:.2f}s")
```

---

## E2E Tests

### Playwright Example

```typescript
// tests/e2e.spec.ts

import { test, expect } from '@playwright/test';

test.describe('AIDEN Chat', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('http://localhost:8081');
    
    // Wait for page to load
    await page.waitForSelector('.chat-container');
  });

  test('health check passes', async ({ page }) => {
    await page.goto('http://localhost:8081/api/health');
    const content = await page.textContent('body');
    expect(content).toContain('healthy');
  });

  test('can send a message', async ({ page }) => {
    // Type message
    await page.fill('.input-field', 'What is AcreetionOS?');
    
    // Send
    await page.click('.send-btn');
    
    // Wait for response
    await page.waitForSelector('.message-assistant', { timeout: 60000 });
    
    // Check response
    const response = await page.textContent('.message-assistant .message-text');
    expect(response.length).toBeGreaterThan(0);
  });

  test('indexing works', async ({ page }) => {
    // Open index modal
    await page.click('button:has-text("Index Docs")');
    
    // Start indexing
    await page.click('button:has-text("Start Indexing")');
    
    // Wait for completion
    await page.waitForFunction(() => {
      const progress = document.querySelector('.progress-text');
      return progress && progress.textContent.includes('complete');
    }, { timeout: 120000 });
  });
});
```

---

## CI/CD Integration

### GitHub Actions

```yaml
# .github/workflows/test.yml

name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      ollama:
        image: ollama/ollama:latest
        ports:
          - 11434:11434
      
      qdrant:
        image: qdrant/qdrant:latest
        ports:
          - 6333:6333
          - 6334:6334
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
      
      - name: Wait for services
        run: |
          sleep 10
          curl -f http://localhost:11434/api/version
          curl -f http://localhost:6333/readyz
      
      - name: Run tests
        run: cargo test
      
      - name: Run clippy
        run: cargo clippy --all-targets -- -D warnings

  api-tests:
    runs-on: ubuntu-latest
    needs: test
    
    steps:
      - name: Start AIDEN
        run: |
          cargo build --release
          ./target/release/aiden &
          sleep 5
      
      - name: Run API tests
        run: python -m pytest tests/
```

### Pre-commit Hooks

```bash
#!/bin/bash
# .git/hooks/pre-commit

set -e

echo "Running pre-commit checks..."

# Format
cargo fmt --check

# Lint
cargo clippy --all-targets --all-features -- -D warnings

# Test
cargo test

echo "All checks passed!"
```
