# AIDEN Documentation Index

Welcome to the AIDEN documentation. This folder contains all official documentation for the project.

## Quick Links

- [README.md](../README.md) - Project overview and quick start
- [DEPLOYMENT.md](DEPLOYMENT.md) - Deployment instructions
- [architecture.md](architecture.md) - System architecture
- [api-reference.md](api-reference.md) - API documentation
- [configuration.md](configuration.md) - Configuration options
- [troubleshooting.md](troubleshooting.md) - Common issues

## Documentation Files

| File | Description |
|------|-------------|
| [README.md](../README.md) | Main project README |
| DEPLOYMENT.md | Docker, systemd, reverse proxy deployment |
| architecture.md | Detailed system architecture |
| api-reference.md | REST API and WebSocket endpoints |
| configuration.md | All configuration options |
| troubleshooting.md | Common issues and solutions |

## Example Documentation

This folder (`docs/`) contains example documentation files that AIDEN can index. The indexer processes all `.md` files in this directory.

### Adding Documentation

To add new documentation for AIDEN to index:

1. Create a markdown file in this directory
2. Write clear, structured content
3. Rebuild the index via the web interface or API

Example:
```bash
echo "# My New Topic" > docs/my-topic.md
# Then use the "Index Docs" button in the web interface
```

### Documentation Guidelines

- Use clear headings (`#`, `##`, `###`)
- Include code examples with triple backticks
- Use bullet points for lists
- Add links between related documents
- Keep chunks contextually coherent (512 words per chunk)
