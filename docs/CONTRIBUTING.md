# Contributing to CalcLaw Complete

Thank you for your interest in contributing to CalcLaw Complete! This document provides guidelines and instructions for contributors.

## 🎯 Code of Conduct

Please be respectful and considerate of others when contributing to this project.

## 🚀 Getting Started

### Prerequisites
- Rust 1.75 or later
- Git
- Basic understanding of Rust and web development

### Development Setup
1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/yourusername/calclaw.git
   cd calclaw
   ```
3. Build the project:
   ```bash
   cargo build
   ```
4. Run tests:
   ```bash
   cargo test
   ```

## 📁 Project Structure

```
src/
├── main.rs              # Entry point and CLI
├── hebrew.rs           # Hebrew text processing
├── nvidia.rs           # NVIDIA AI integration
├── tts.rs              # Text-to-speech engine
├── skills.rs           # Skills system
└── lib.rs              # Library exports (if needed)
```

## 🔧 Development Workflow

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/amazing-feature
   ```

2. **Make your changes:**
   - Follow Rust coding conventions
   - Add tests for new functionality
   - Update documentation as needed

3. **Run tests and checks:**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   ```

4. **Commit your changes:**
   ```bash
   git add .
   git commit -m "Add amazing feature"
   ```

5. **Push to your fork:**
   ```bash
   git push origin feature/amazing-feature
   ```

6. **Create a Pull Request:**
   - Go to the original repository
   - Click "New Pull Request"
   - Select your feature branch
   - Fill in the PR template

## 📝 Coding Standards

### Rust Conventions
- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `rustfmt` for formatting
- Run `clippy` for linting
- Document public APIs with doc comments

### Error Handling
- Use `anyhow::Result` for application code
- Use `thiserror` for library error types
- Provide helpful error messages

### Testing
- Write unit tests for functions
- Write integration tests for APIs
- Test edge cases and error conditions

## 🎨 Feature Areas

### Hebrew Language Support
- RTL text processing
- Hebrew calendar integration
- Locale-aware responses

### AI Integration
- NVIDIA NIM API client
- Model management
- Prompt engineering

### TTS Engine
- Multiple provider support
- Audio caching
- Voice customization

### Skills System
- Skill templates
- Trigger conditions
- Action execution

## 📚 Documentation

### Code Documentation
- Document all public functions and types
- Include examples in doc comments
- Update README.md for user-facing changes

### API Documentation
- Document all API endpoints
- Include request/response examples
- Update OpenAPI specification if applicable

## 🐛 Bug Reports

When reporting bugs, please include:
1. Clear description of the issue
2. Steps to reproduce
3. Expected vs actual behavior
4. Environment details (OS, Rust version, etc.)
5. Relevant logs or error messages

## 💡 Feature Requests

When requesting features, please:
1. Describe the use case
2. Explain the expected behavior
3. Suggest implementation approach if possible
4. Consider backward compatibility

## 🔒 Security Issues

Please report security vulnerabilities privately to the maintainers.

## 🏆 Recognition

Contributors will be recognized in:
- GitHub contributors list
- Release notes
- Project documentation

## ❓ Questions?

- Check the [README.md](README.md)
- Look at existing issues and PRs
- Ask in GitHub Discussions
- Contact maintainers if needed

## 📄 License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to CalcLaw Complete! 🦾🎤🔧