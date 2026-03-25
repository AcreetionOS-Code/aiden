# AIDEN Community Guide

How to contribute to and engage with the AIDEN community.

## Table of Contents

1. [Getting Help](#getting-help)
2. [Reporting Issues](#reporting-issues)
3. [Contributing](#contributing)
4. [Community Standards](#community-standards)
5. [Resources](#resources)

---

## Getting Help

### Where to Ask

1. **GitLab Issues**: For bugs and feature requests
2. **GitHub Discussions**: For questions and community support
3. **Documentation**: Check the docs folder first

### Before Asking

Please:
- Search existing issues
- Check the FAQ
- Review troubleshooting docs
- Have system information ready

### System Information to Include

When asking for help, include:

```bash
# OS version
cat /etc/os-release

# AIDEN version
# (check Cargo.toml)

# Ollama version
ollama --version

# Qdrant version
curl http://localhost:6333/readyz

# GPU info
nvidia-smi

# Service status
systemctl status aiden ollama qdrant
```

---

## Reporting Issues

### Bug Reports

Open an issue with:
- Clear title
- Steps to reproduce
- Expected behavior
- Actual behavior
- System information
- Logs (if applicable)

**Template**:
```markdown
### Description
[Description of the bug]

### Steps to Reproduce
1. [Step 1]
2. [Step 2]
3. [Step 3]

### Expected Behavior
[What should happen]

### Actual Behavior
[What actually happens]

### Environment
- OS: [e.g., Arch Linux]
- AIDEN: [version]
- Ollama: [version]
- Qdrant: [version]
- GPU: [if applicable]

### Logs
```
[paste relevant logs]
```
```

### Feature Requests

Open an issue with:
- Clear description
- Use case
- Proposed solution (optional)
- Alternatives considered (optional)

---

## Contributing

### Ways to Contribute

- **Code**: Fix bugs, add features
- **Documentation**: Improve docs, fix typos
- **Testing**: Test new features, report bugs
- **Feedback**: Share your experience
- **Promotion**: Star, share, recommend

### Development Setup

```bash
# Fork the repository
# Clone your fork
git clone https://gitlab.acreetionos.org/yourusername/aiden.git
cd aidenaiden

# Add upstream remote
git remote add upstream https://gitlab.acreetionos.org/natalie/aiden.git

# Create feature branch
git checkout -b feature/my-feature

# Install prerequisites
# - Rust
# - Ollama
# - Qdrant

# Make changes
# ...

# Test
cargo test

# Commit (sign if possible)
git commit -s

# Push to your fork
git push origin feature/my-feature

# Open pull request
```

### Pull Request Guidelines

1. **Branch**: Create from `develop`
2. **Title**: Clear and descriptive
3. **Description**: Explain what and why
4. **Tests**: Include tests for new features
5. **Docs**: Update docs if needed
6. **Style**: Follow existing code style

### Code Review Process

1. Maintainer reviews PR
2. Feedback provided (if needed)
3. Make requested changes
4. PR approved and merged
5. Changes pushed to main

---

## Community Standards

### Code of Conduct

We are committed to providing a welcoming community.

**Be**:
- Respectful
- Inclusive
- Helpful
- Constructive

**Don't**:
- Harass
- Troll
- Spam
- Derail discussions

### Communication Guidelines

- Use clear language
- Be patient with beginners
- Stay on topic
- Search before asking
- Provide context

---

## Resources

### Official Links

- **GitHub**: https://github.com/AcreetionOS-Code/aiden
- **Live Instance**: https://docs.acreetionos.org

### Related Projects

- [Ollama](https://ollama.com) - Local LLM runtime
- [Qdrant](https://qdrant.tech) - Vector database
- [Axum](https://github.com/tokio-rs/axum) - Web framework

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [RAG Tutorial](https://python.langchain.com/docs/use_cases/question_answering/)
