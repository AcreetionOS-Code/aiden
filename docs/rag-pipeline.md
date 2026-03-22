# AIDEN RAG Pipeline Deep Dive

Understanding how Retrieval-Augmented Generation works in AIDEN.

## Table of Contents

1. [Overview](#overview)
2. [Retrieval Process](#retrieval-process)
3. [Generation Process](#generation-process)
4. [Chunking Strategy](#chunking-strategy)
5. [Embedding Models](#embedding-models)
6. [Vector Search](#vector-search)
7. [Context Assembly](#context-assembly)
8. [Prompt Engineering](#prompt-engineering)

---

## Overview

AIDEN uses RAG to provide accurate, source-cited answers:

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Document   │────▶│   Chunking   │────▶│  Embedding  │
└──────────────┘     └──────────────┘     └──────────────┘
                                                 │
                                                 ▼
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   Response   │◀────│   LLM Gen    │◀────│   Vector DB  │
└──────────────┘     └──────────────┘     └──────────────┘
```

---

## Retrieval Process

### Step 1: Query Understanding

```rust
// User question
let question = "How do I install AcreetionOS?";

// Query is sent to embedding model
let query_embedding = ollama.embeddings("nomic-embed-text", &question).await?;
```

### Step 2: Vector Search

```rust
// Search Qdrant for similar documents
let results = qdrant.search(&query_embedding, limit=5).await?;
```

### Step 3: Result Filtering

```rust
// Filter by similarity threshold
let relevant_docs: Vec<SearchHit> = results
    .into_iter()
    .filter(|hit| hit.score >= config.search.threshold)
    .collect();
```

---

## Generation Process

### Step 1: Context Assembly

```rust
let context = relevant_docs
    .iter()
    .map(|doc| format!("Source: {}\n{}", doc.source, doc.text))
    .join("\n\n");
```

### Step 2: Prompt Construction

```rust
let system_prompt = format!(
    "You are AIDEN, an AI assistant for AcreetionOS. \
    Answer questions based ONLY on the provided context. \
    If the answer is not in the context, say you don't know.\n\n\
    Context:\n{}\n\n\
    Remember: Cite sources when possible.",
    context
);
```

### Step 3: LLM Generation

```rust
let response = ollama.chat("llama3.1:8b", &[
    OllamaMessage { role: "system", content: system_prompt },
    OllamaMessage { role: "user", content: question }
]).await?;
```

---

## Chunking Strategy

### How It Works

Documents are split into overlapping chunks:

```
┌─────────────────────────────────────────────────────┐
│ Document: "AcreetionOS Installation Guide"          │
├─────────────────────────────────────────────────────┤
│ ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌─────────┐     │
│ │ Chunk 1 │ │ Chunk 2 │ │ Chunk 3 │ │ Chunk 4 │     │
│ │ (words) │ │ (words) │ │ (words) │ │ (words) │     │
│ └─────────┘ └─────────┘ └─────────┘ └─────────┘     │
│     ▲ overlap ▲                                   │
└─────────────────────────────────────────────────────┘
```

### Configuration

```rust
IndexingConfig {
    chunk_size: 512,      // Target words per chunk
    chunk_overlap: 50,    // Overlap between chunks
}
```

### Best Practices

| Document Type | Chunk Size | Overlap |
|---------------|------------|---------|
| FAQs | 128-256 | 25 |
| How-to Guides | 512 | 50 |
| API Docs | 768 | 100 |
| Manuals | 1024 | 150 |

---

## Embedding Models

### nomic-embed-text

- **Dimensions**: 768
- **Context Length**: 512 tokens
- **Purpose**: Text similarity search
- **Speed**: Fast (~50ms per embedding)

### Alternative Models

```rust
// Option 1: all-MiniLM-L6-v2 (smaller, faster)
// - Dimensions: 384
// - Use with: VectorParamsBuilder::new(384, Distance::Cosine)

// Option 2: bge-large-en-v1.5 (larger, more accurate)
// - Dimensions: 1024
// - Use with: VectorParamsBuilder::new(1024, Distance::Cosine)
```

---

## Vector Search

### Search Algorithm

Qdrant uses HNSW (Hierarchical Navigable Small World):

```rust
// Create collection with HNSW index
CreateCollectionBuilder::new("aiden")
    .vectors_config(
        VectorParamsBuilder::new(768, Distance::Cosine)
            .hnsw_config(HnswConfigDiff {
                m: Some(16),           // Connections per node
                ef_construct: Some(200) // Build quality
            })
    )
```

### Distance Metrics

| Metric | Best For | Formula |
|--------|----------|---------|
| Cosine | Text similarity | 1 - cosine(angle) |
| Euclidean | General purpose | sqrt(sum((a-b)^2)) |
| Dot | When magnitude matters | sum(a*b) |

---

## Context Assembly

### Priority-Based Assembly

```rust
fn assemble_context(hits: Vec<SearchHit>, max_tokens: usize) -> String {
    let mut context = String::new();
    let mut total_tokens = 0;
    
    for hit in hits {
        let chunk_tokens = estimate_tokens(&hit.text);
        if total_tokens + chunk_tokens > max_tokens {
            break;
        }
        
        context.push_str(&format!(
            "[Source: {} (score: {:.2})]\n{}\n\n",
            hit.source, hit.score, hit.text
        ));
        total_tokens += chunk_tokens;
    }
    
    context
}
```

### Token Estimation

```rust
fn estimate_tokens(text: &str) -> usize {
    // Rough estimate: ~4 chars per token
    text.len() / 4
}
```

---

## Prompt Engineering

### System Prompt

```rust
const SYSTEM_PROMPT: &str = r#"You are AIDEN, an AI assistant for AcreetionOS.

Guidelines:
1. Answer based ONLY on the provided context
2. If information is not in context, say "I don't know"
3. Be concise but complete
4. Use markdown formatting when helpful
5. Cite sources using [Source: filename] notation

Context will be provided below the question.
"#;
```

### Example Conversation

```
System: You are AIDEN, an AI assistant...

User: How do I install AcreetionOS?

Context:
[Source: installation.md]
# Installation Guide

1. Download the ISO from acreetionos.org
2. Create a bootable USB drive
3. Boot from USB and follow the wizard

Assistant: Based on the installation guide [Source: installation.md], 
to install AcreetionOS:

1. Download the ISO from acreetionos.org
2. Create a bootable USB drive
3. Boot from USB and follow the installation wizard

For more details, see the full installation guide.
```

---

## Performance Optimization

### Retrieval Optimizations

```rust
// 1. Use ANN indices for faster search
// Already built into Qdrant

// 2. Limit search results
let results = qdrant.search(&embedding, limit=5).await?;

// 3. Use early stopping
let results: Vec<_> = qdrant.search(&embedding, limit=10)
    .await?
    .into_iter()
    .take_while(|hit| hit.score > 0.5)
    .collect();
```

### Generation Optimizations

```rust
// 1. Use streaming responses
// Reduces perceived latency

// 2. Cache common embeddings
use std::collections::HashMap;
let embedding_cache: HashMap<String, Vec<f32>> = HashMap::new();

// 3. Use smaller models for simple queries
let model = if is_simple_question(&q) {
    "llama3.2:1b"
} else {
    "llama3.1:8b"
};
```

---

## Troubleshooting RAG Issues

### Poor Retrieval Results

**Symptoms**: Irrelevant sources returned

**Solutions**:
- Lower the similarity threshold
- Increase `max_results`
- Improve documentation quality
- Use a better embedding model

### Hallucinations

**Symptoms**: Answers not matching sources

**Solutions**:
- Raise the similarity threshold
- Add "cite sources" to system prompt
- Use a better (larger) model
- Check for contradictory docs

### Slow Performance

**Symptoms**: Long response times

**Solutions**:
- Enable GPU acceleration for Ollama
- Optimize chunk sizes
- Use smaller embedding model
- Add caching layer
