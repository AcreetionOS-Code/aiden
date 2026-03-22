# AIDEN Glossary

Terms and definitions used in AIDEN.

## A

### API (Application Programming Interface)
A set of rules that allows AIDEN to communicate with other software.

### API Endpoint
A specific URL where an API request can be made (e.g., `/api/chat`).

### Async (Asynchronous)
Processing that happens without waiting for previous operations to complete.

### Axum
The Rust web framework used by AIDEN.

## B

### Backoff
When a service waits before retrying after a failure.

### Batch Processing
Processing multiple items at once instead of one by one.

### Bearer Token
A security token used in API authentication.

## C

### Chat Model
An AI model trained for conversation (e.g., llama3.2:1b).

### Chunk
A piece of a document split for indexing and search.

### Chunk Overlap
The number of words shared between adjacent chunks.

### Chunk Size
The target number of words per chunk.

### CIDR (Classless Inter-Domain Routing)
A way to specify IP address ranges (e.g., 192.168.0.0/24).

### Collection
A named group of vectors in Qdrant.

### Context Window
The maximum amount of text a model can process at once.

### Cosine Similarity
A measure of how similar two vectors are (0-1 scale).

## D

### Default Route
The fallback path when no specific route matches.

### Distance Metric
A method to measure similarity between vectors.

## E

### Embedding
A numerical representation of text that captures meaning.

### Embedding Model
An AI model that converts text to embeddings (e.g., nomic-embed-text).

## F

### Fallback
An alternative action when the primary method fails.

### Feature Branch
A separate line of development for new features.

## G

### gRPC
A high-performance RPC framework used by Qdrant.

### GPU (Graphics Processing Unit)
Specialized processor for AI computations.

## H

### HNSW (Hierarchical Navigable Small World)
An algorithm for fast approximate nearest neighbor search.

### Health Check
An endpoint that reports service status.

## I

### Index (noun)
A data structure for fast searching.

### Index (verb)
To process and add documents to a search index.

### Indexing Status
Information about the current state of document indexing.

## J

### JSON (JavaScript Object Notation)
A common format for data exchange.

## L

### LLM (Large Language Model)
A type of AI model trained on text (e.g., GPT, LLaMA).

### Load Balancer
Distributes traffic across multiple servers.

## M

### Markdown
A lightweight markup language for formatted text (.md files).

### Merge
Combining changes from one branch into another.

### Middleware
Software that processes requests before they reach handlers.

## N

### Namespace
A way to organize resources (used in some databases).

## O

### Ollama
A tool for running LLMs locally.

### Overlap
See "Chunk Overlap".

## P

### Payload
The data sent in an API request or response.

### Port
A network endpoint for communication (e.g., 8081).

### Prompt
The input text given to an LLM.

### Proxy
An intermediary that forwards requests.

## Q

### Qdrant
A vector database for similarity search.

### Query
A search request.

## R

### RAG (Retrieval-Augmented Generation)
A technique that combines search with AI generation.

### REST (Representational State Transfer)
A style of API design.

### RFC3339
A standard format for dates and times.

### Route
A mapping between a URL path and a handler function.

## S

### Schema
The structure of a database or data format.

### Scope
The range of access or influence (e.g., OAuth scope).

### Search Threshold
The minimum similarity score to return a result.

### Semantic Search
Search based on meaning rather than keywords.

### Service
A background process that runs continuously.

### Shard
A portion of a distributed database.

### Similarity Score
A number indicating how similar two items are.

### SSE (Server-Sent Events)
A way to stream data from server to client.

### State
Data that persists across requests.

### Streaming
Sending data incrementally as it's generated.

## T

### TCP (Transmission Control Protocol)
A protocol for reliable network communication.

### Threshold
See "Search Threshold".

### Tokio
The async runtime for Rust used by AIDEN.

### Token
A unit of text that models process (roughly 4 characters).

### TLS (Transport Layer Security)
Encryption for network communication.

### TOML (Tom's Obvious, Minimal Language)
A configuration file format.

## U

### UUID (Universally Unique Identifier)
A unique identifier (e.g., 550e8400-e29b-41d4-a716-446655440000).

## V

### Vector
An array of numbers representing data.

### Vector Database
A database optimized for storing and searching vectors.

### Vector Dimension
The length of a vector (e.g., 768 for nomic-embed-text).

### Virtual Environment
An isolated environment for running software.

## W

### WebSocket
A bidirectional communication channel over HTTP.

### Worker
A process that handles requests.

## X

### XML (eXtensible Markup Language)
A data format (less common than JSON now).

## Z

### Zero-shot
Ability to perform tasks without specific training.
