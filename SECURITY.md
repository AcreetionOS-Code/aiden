# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

If you discover a security vulnerability within AIDEN, please report it responsibly.

**Do NOT** create public GitLab/GitHub issues for security vulnerabilities.

### How to Report

1. **Email**: Contact Natalie Spiva at [natalie@acreetionos.org](mailto:natalie@acreetionos.org)
2. **PGP**: For sensitive reports, use PGP encryption with key fingerprint `1DD93EDCFFF0B9578A411AE8EEC627F9059B962C`
3. **Response Time**: We aim to respond within 48 hours
4. **Disclosure**: We follow coordinated disclosure practices

## Security Considerations

### Local Deployment

AIDEN is designed for local deployment with the following considerations:

- **Ollama**: Runs locally; no data leaves your network
- **Qdrant**: Vector database stays on your infrastructure
- **No External APIs**: All AI inference is performed locally
- **No Telemetry**: No usage data is collected or transmitted

### Network Security

- Default listen address is `0.0.0.0:8081`
- Use firewall rules to restrict access
- Consider running behind a reverse proxy with TLS
- Implement authentication at the proxy level if needed

### Data Privacy

- Documentation chunks are stored in Qdrant
- Conversation history is kept in memory (not persisted)
- No cookies or tracking mechanisms
- No third-party analytics

### Production Recommendations

1. **Network Isolation**: Run AIDEN in an isolated network segment
2. **Reverse Proxy**: Add TLS termination with nginx/Caddy
3. **Authentication**: Implement authentication at the proxy level
4. **Rate Limiting**: Add rate limiting to prevent abuse
5. **Input Validation**: The Rust codebase uses type-safe input handling
6. **Updates**: Keep Ollama and Qdrant updated for security patches

## Known Limitations

- No built-in authentication (use reverse proxy)
- No built-in TLS (use reverse proxy)
- No rate limiting (use reverse proxy)
- Conversation history not persisted (feature, not a bug)
