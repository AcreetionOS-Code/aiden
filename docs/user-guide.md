# AIDEN User Guide

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [Using the Chat Interface](#using-the-chat-interface)
4. [Indexing Documentation](#indexing-documentation)
5. [Understanding Responses](#understanding-responses)
6. [Tips and Best Practices](#tips-and-best-practices)
7. [Troubleshooting](#troubleshooting)
8. [FAQ](#faq)

---

## Introduction

AIDEN (AI Documentation Engine for AcreetionOS) is your intelligent assistant for learning about and working with AcreetionOS. It uses advanced AI to understand your questions and provide accurate, source-cited answers from official documentation.

### Key Features

- **Natural Language Queries**: Ask questions in plain English
- **Source Citations**: Every answer links to the source documentation
- **Instant Responses**: Powered by local AI for fast answers
- **Documentation Aware**: Built specifically for AcreetionOS knowledge
- **Privacy Preserving**: All processing happens locally

---

## Getting Started

### Accessing AIDEN

Open your web browser and navigate to:

```
http://localhost:8081
```

For remote access, use your server's IP address:

```
http://192.168.1.100:8081
```

### System Requirements

AIDEN runs best with:

- **CPU**: Modern multi-core processor
- **RAM**: 8GB minimum (16GB recommended)
- **GPU**: NVIDIA GPU with 4GB+ VRAM (optional but recommended)
- **Storage**: 10GB for models and documentation

### First-Time Setup

On first access, AIDEN is ready to use. However, for best results:

1. Click the **Index Docs** button in the top-right corner
2. Wait for indexing to complete
3. Start asking questions!

---

## Using the Chat Interface

### Basic Chat

1. Type your question in the input field at the bottom
2. Press **Enter** or click the **Send** button
3. Wait for the AI response
4. View source citations below the response

### Example Questions

**Getting Started:**
- "How do I install AcreetionOS?"
- "What are the system requirements?"
- "How do I update the system?"

**Configuration:**
- "How do I configure the firewall?"
- "What network settings are available?"
- "How do I set up user accounts?"

**Troubleshooting:**
- "Why is my system running slow?"
- "How do I fix network connectivity issues?"
- "What do I do if a package won't install?"

**Features:**
- "What desktop environments are available?"
- "How do I use the package manager?"
- "What security features does AcreetionOS have?"

### Quick Actions

The welcome card shows quick action buttons for common questions:

- What is AcreetionOS?
- How do I install AcreetionOS?
- What desktop environments are available?
- How do I update the system?

Click any button to get an instant answer.

### Chat Features

- **Multi-line Input**: Hold Shift+Enter for new lines
- **Conversation History**: Scroll up to see previous messages
- **Copy Response**: Click on text to select and copy
- **Source Links**: Click source badges to view original docs

---

## Indexing Documentation

### What is Indexing?

Indexing processes your documentation files and makes them searchable by the AI. After indexing, AIDEN can find relevant information from your docs.

### When to Re-index

Re-index when you:

- Add new documentation files
- Update existing documentation
- Change the docs directory location
- Notice outdated responses

### How to Index

1. Click **Index Docs** in the top-right corner
2. A modal will appear showing progress
3. Click **Start Indexing**
4. Wait for completion (usually 30-60 seconds)
5. Close the modal when done

### Indexing Progress

The indexing modal shows:

- **Files Processed**: Number of docs indexed
- **Chunks Created**: Searchable pieces created
- **Progress Bar**: Overall completion percentage
- **Errors**: Any issues encountered

---

## Understanding Responses

### Response Structure

Each AI response includes:

1. **Main Answer**: The direct answer to your question
2. **Source Citations**: Links to relevant documentation
3. **Context**: Additional helpful information

### Source Badges

| Badge | Meaning |
|-------|---------|
| **Local** | From your indexed documentation |
| **Web** | From web search results |

### Confidence Indicators

AIDEN shows confidence through:

- **High Confidence**: Detailed, sourced answers
- **Medium Confidence**: General guidance with caveats
- **Low Confidence**: Suggestions to verify information

---

## Tips and Best Practices

### Writing Good Questions

**Do:**
- Be specific about what you want to know
- Mention the context (e.g., "for a server setup")
- Ask one question at a time
- Use complete sentences

**Don't:**
- Ask vague questions ("How do I computer?")
- Stack multiple questions
- Use jargon without explanation
- Expect opinions on subjective topics

### Getting Better Answers

1. **Start Broad, Then Narrow**
   - First: "How do I install software?"
   - Then: "How do I install software from source?"

2. **Ask for Examples**
   - "Can you show me an example of..."

3. **Request Step-by-Step**
   - "Please give me step-by-step instructions for..."

4. **Follow Up**
   - "Can you explain that in more detail?"
   - "What does that term mean?"

### Managing Conversation

- **Clear Chat**: Refresh the page to start fresh
- **Check Sources**: Always verify important information
- **Compare**: Ask the same question differently for comparison

---

## Troubleshooting

### AIDEN is Offline

**Symptom**: Status shows "Offline" or red indicator

**Solutions**:
1. Check if the service is running:
   ```bash
   systemctl status aiden
   ```
2. Restart the service:
   ```bash
   sudo systemctl restart aiden
   ```

### Slow Responses

**Symptom**: Takes more than 30 seconds for a response

**Solutions**:
1. Check Ollama is using GPU:
   ```bash
   nvidia-smi
   ```
2. Restart Ollama:
   ```bash
   sudo systemctl restart ollama
   ```
3. Reduce index size by removing unused docs

### Outdated Information

**Symptom**: Answers don't match current documentation

**Solution**: Re-index documentation:
1. Click **Index Docs**
2. Click **Start Indexing**
3. Wait for completion

### "No Relevant Sources Found"

**Symptom**: Response has no source citations

**Solutions**:
1. Add relevant documentation to the `docs/` folder
2. Re-index after adding files
3. Try rephrasing your question

---

## FAQ

### Is my data sent to external servers?

**No.** AIDEN runs entirely on your local machine. Your questions and the responses never leave your network.

### Can I use AIDEN offline?

**Yes.** Once installed, AIDEN works completely offline. No internet connection is required.

### How accurate are the answers?

AIDEN provides answers based on your indexed documentation. Accuracy depends on:
- Quality of your documentation
- How up-to-date the docs are
- How well your question matches available content

### Can I add my own documentation?

**Yes!** Simply add markdown (`.md`) files to the `docs/` directory and re-index.

### What models does AIDEN use?

By default, AIDEN uses:
- **Chat**: llama3.1:8b (or configured model)
- **Embeddings**: nomic-embed-text

### How do I change the AI model?

Edit `src/state.rs` and change the `chat_model` setting, then rebuild.

### Can I run AIDEN on multiple computers?

**Yes.** Install AIDEN on each computer, or run it as a server and access via network.

### How do I report bugs?

Open an issue on the project repository with:
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information

---

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| Enter | Send message |
| Shift+Enter | New line |
| Escape | Close modal |

---

## Getting Help

If you need further assistance:

- Check the [Troubleshooting Guide](troubleshooting.md)
- Review the [API Documentation](api-reference.md)
- Open an issue on the project repository
- Contact the development team
