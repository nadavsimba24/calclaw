# 🦾 Calclaw - AI Assistant Framework

<div align="center">

![Calclaw Logo](https://img.shields.io/badge/Calclaw-AI_Assistant-blue)
![Version](https://img.shields.io/badge/version-1.0.0-green)
![License](https://img.shields.io/badge/license-MIT-yellow)
![Rust](https://img.shields.io/badge/Rust-1.75+-orange)
![Docker](https://img.shields.io/badge/Docker-ready-blue)

**AI assistant framework for personal and enterprise use**

[📖 Documentation](#documentation) • [🚀 Quick Start](#quick-start) • [🏢 Enterprise](#enterprise) • [🤝 Contributing](#contributing)

</div>

## ✨ Features

### 🏠 **Personal Edition**
- 🤖 **Local AI** with Ollama integration
- 📝 **Hebrew NLP** with full RTL support
- 💬 **Smart Chat** with context compaction
- 🔊 **Voice TTS** with eSpeak NG
- 🤖 **Telegram Bot** with voice messages
- ⚡ **Fast & Private** - runs entirely locally

### 🏢 **Enterprise Edition**
- 🔒 **Advanced Security** with MCP Proxy approvals
- 👥 **Multi-tenant** architecture
- 🔗 **Integrations** with Slack, GitHub, Kubernetes
- 📊 **Monitoring** with Prometheus & Grafana
- 🐳 **Docker & Kubernetes** ready
- 📈 **Scalable** from 1 to 1000+ users

### 🏛️ **Claw Organ** (Advanced Enterprise)
- 🏗️ **Remote Sandbox** on Kubernetes
- ✅ **Approval Workflows** via Slack
- 🔐 **OAuth 2.0** permissions only
- 📁 **Single Source of Truth** configuration
- 🌐 **Network Policies** with zero-trust

## 🚀 Quick Start

### Option 1: Smart Installation (Recommended)
```bash
# Clone the repository
git clone https://github.com/calclaw/calclaw.git
cd calclaw

# Run smart installer
chmod +x install_smart.sh
./install_smart.sh
```

### Option 2: Personal Edition
```bash
./install_personal.sh
```

### Option 3: With Docker
```bash
./run_with_docker.sh
```

## 📦 Installation Types

Calclaw offers multiple installation options tailored to your needs:

| Type | Description | Best For |
|------|-------------|----------|
| **🏠 Personal** | Local installation, full access | Individuals, developers, testing |
| **🏢 Enterprise** | Server/cloud with security controls | Teams, organizations, production |
| **🏛️ Claw Organ** | Kubernetes-based sandbox | Large enterprises, regulated industries |
| **🔧 Custom** | Tailored to specific requirements | Unique use cases, special integrations |

## 🐳 Docker Support

### Build Images
```bash
./build_docker.sh
```

### Run with Docker
```bash
./run_with_docker.sh
```

### Docker Compose
```yaml
version: '3.8'
services:
  calclaw:
    image: calclaw/calclaw:latest
    ports:
      - "3000:3000"
  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
```

## 🔧 Architecture

### Core Components
1. **🤖 AI Engine** - Local Ollama models (Phi-3, Gemma, Llama)
2. **💬 Chat Interface** - Web, CLI, Telegram, Slack
3. **🔒 Security Layer** - MCP Proxy with approval workflows
4. **📊 Monitoring** - Prometheus, Grafana, Loki
5. **🗄️ Storage** - PostgreSQL, Redis, file system

### Integration Ecosystem
- **💬 Slack** - Real-time messaging and approvals
- **🐙 GitHub** - Code management and automation
- **🐳 Kubernetes** - Container orchestration
- **☁️ Cloud** - AWS, Azure, GCP integration
- **📁 Filesystem** - Local and remote file access

## 📚 Documentation

### Quick Links
- [📖 Full Documentation](https://docs.calclaw.com)
- [🏠 Personal Guide](INSTALL_SMART.md#personal-edition)
- [🏢 Enterprise Guide](ENTERPRISE_GUIDE.md)
- [🐳 Docker Guide](docker/README.md)
- [🔧 API Reference](docs/api.md)

### Getting Started Guides
1. [First Time Setup](docs/getting-started.md)
2. [Hebrew NLP Setup](docs/hebrew-nlp.md)
3. [Telegram Bot Setup](docs/telegram-bot.md)
4. [Ollama Integration](docs/ollama-integration.md)

## 🏢 Enterprise Features

### Security
- ✅ **MCP Proxy** with tool annotations
- ✅ **Approval Workflows** via Slack/email
- ✅ **Audit Logging** with 90-day retention
- ✅ **Network Policies** with zero-trust
- ✅ **OAuth 2.0** with MFA support

### Scalability
- 📈 **Horizontal Scaling** with Kubernetes
- 👥 **Multi-tenant** architecture
- 🔄 **Load Balancing** with Traefik
- 💾 **Persistent Storage** with backups
- 📊 **Performance Monitoring** with alerts

### Integrations
- 💬 **Slack** - Messaging and approvals
- 🐙 **GitHub/GitLab** - Code management
- 🐳 **Docker/Kubernetes** - Container orchestration
- 📁 **Filesystem** - Local and cloud storage
- 🗄️ **Database** - PostgreSQL, MySQL, Redis

## 🔌 API

### REST API
```bash
# Health check
GET /api/health

# Hebrew NLP processing
POST /api/hebrew
Content-Type: application/json
{
  "text": "שלום עולם",
  "action": "analyze"
}

# AI generation
POST /api/ollama/generate
Content-Type: application/json
{
  "model": "phi3:mini",
  "prompt": "Explain quantum computing",
  "stream": false
}
```

### WebSocket API
```javascript
const ws = new WebSocket('ws://localhost:3000/ws');
ws.onmessage = (event) => {
  console.log('AI response:', event.data);
};
```

## 🤝 Contributing

We welcome contributions! Here's how to get started:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Development Setup
```bash
# Clone with submodules
git clone --recursive https://github.com/calclaw/calclaw.git

# Install dependencies
cargo build
npm install  # for web UI

# Run tests
cargo test
./test_enterprise.sh
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- **Ollama** for local AI models
- **Rust** community for excellent libraries
- **MCP Protocol** for tool integration standard
- **OpenClaw** for the inspiration and foundation

## 📞 Support

- **📖 Documentation**: [docs.calclaw.com](https://docs.calclaw.com)
- **💬 Community**: [Discord](https://discord.gg/calclaw)
- **🐛 Issues**: [GitHub Issues](https://github.com/calclaw/calclaw/issues)
- **📧 Email**: support@calclaw.com

## 🌟 Star History

[![Star History Chart](https://api.star-history.com/svg?repos=calclaw/calclaw&type=Date)](https://star-history.com/#calclaw/calclaw&Date)

---

<div align="center">

**Made with ❤️ by the Calclaw Team**

[🏠 Website](https://calclaw.com) • [🐙 GitHub](https://github.com/calclaw) • [🐦 Twitter](https://twitter.com/calclaw_ai)

</div>