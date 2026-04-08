# CalcLaw Complete 🦾🎤🔧

**Enterprise AI Assistant with Hebrew Support, NVIDIA AI, TTS, and Organizational Skills System**

## ✨ Complete Feature Set

### 🇮🇱 **Hebrew-First Design**
- Full RTL (right-to-left) support
- Hebrew text detection and processing
- Hebrew calendar support (optional)
- Locale-aware responses

### 🤖 **NVIDIA AI Integration**
- Access to 10+ NVIDIA NIM models
- Hebrew-aware prompt formatting
- Enterprise-grade AI performance
- Cost-effective token usage

### 🎤 **Text-to-Speech (TTS)**
- Multiple providers: Google Cloud, ElevenLabs, Azure, OpenAI
- Hebrew voice support
- Audio file generation and caching
- Voice customization (speed, pitch, volume)

### 🔧 **Organizational Skills System**
- Self-service skill creator for organizations
- Built-in templates (reports, notifications, automation)
- Department-specific skills
- Skill instance management
- Trigger-based automation

### 👥 **Role-Based Access Control**
- Admin, Superuser, User roles
- Department-based permissions
- Skill-level access control
- Audit logging

### 🔌 **Business Integrations** (Ready for Implementation)
- Telegram, WhatsApp
- Monday.com, Salesforce, ServiceNow
- Google Sheets, FTP/SFTP
- File uploader

## 🚀 Quick Start

### Prerequisites
- Rust 1.75+
- NVIDIA API key (from https://build.nvidia.com/)
- Optional: TTS provider API keys

### Installation
```bash
# Clone and build
git clone <repository-url>
cd calclaw

# Build with all features
cargo build --release

# Run
./target/release/calclaw
```

### Docker
```bash
docker build -t calclaw-complete .
docker run -p 3000:3000 calclaw-complete
```

## 📖 API Documentation

### Core Endpoints
- `GET /health` - Health check
- `POST /api/hebrew` - Hebrew text processing
- `GET /api/users` - User management

### NVIDIA AI Endpoints
- `GET /api/nvidia/models` - List available models
- `POST /api/nvidia/configure` - Configure NVIDIA API
- `POST /api/nvidia/generate` - Generate AI responses

### TTS Endpoints
- `POST /api/tts/generate` - Generate speech from text
- `GET /api/tts/voices` - List available voices
- `POST /api/tts/configure` - Configure TTS provider

### Skills System Endpoints
- `GET /api/skills/templates` - List skill templates
- `POST /api/skills/create` - Create skill from template
- `POST /api/skills/instance` - Create skill instance
- `GET /api/skills/organization/:id` - List organization skills
- `GET /api/skills/instances/:org_id` - List skill instances

## 🏗️ Architecture

```
CalcLaw Core (Rust + Axum)
├── API Server
├── Hebrew Processor
├── NVIDIA AI Client
├── TTS Engine (Multi-provider)
├── Skills System
│   ├── Skill Creator
│   ├── Template Manager
│   ├── Instance Manager
│   └── Execution Engine
├── User/Role Management
└── Integration Adapters
```

## 🔧 Configuration

1. Copy `config-complete.toml` to `config.toml`
2. Add your NVIDIA API key
3. Configure TTS providers (optional)
4. Set up organization departments

Example minimal config:
```toml
[server]
port = 3000

[nvidia]
api_key = "your-nvidia-api-key"
model = "meta/llama-3.1-8b-instruct"

[skills]
skills_dir = "./data/skills"
```

## 🎯 Use Cases

### For Sales Departments
- Daily sales reports via Telegram
- Lead qualification automation
- Customer follow-up reminders
- Sales forecasting with AI

### For Support Teams
- Ticket escalation alerts
- Knowledge base search
- Customer satisfaction analysis
- Support metric reporting

### For Marketing
- Campaign performance tracking
- Social media monitoring
- Content generation with AI
- Marketing analytics

### For Operations
- Inventory management alerts
- Supply chain monitoring
- Process automation
- Quality control checks

## 🔌 Integration Examples

### Create Daily Sales Report Skill
```bash
curl -X POST http://localhost:3000/api/skills/create \
  -H "Content-Type: application/json" \
  -d '{
    "template_name": "Daily Sales Report",
    "organization_id": "acme-corp",
    "department": "Sales",
    "author": "Sales Manager",
    "config": {
      "recipients": ["sales@acme.com", "manager@acme.com"],
      "sales_target": 50000
    }
  }'
```

### Generate Hebrew TTS
```bash
curl -X POST http://localhost:3000/api/tts/generate \
  -H "Content-Type: application/json" \
  -d '{
    "text": "דוח מכירות יומי מוכן. סך המכירות: 45,000 ש״ח",
    "provider": "GoogleCloud",
    "voice_id": "he-IL-Standard-A",
    "language": "he-IL"
  }'
```

### AI-Powered Customer Response
```bash
curl -X POST http://localhost:3000/api/nvidia/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "לקוח שואל על מוצר X. מה התשובה הטובה ביותר?",
    "model": "meta/llama-3.1-8b-instruct"
  }'
```

## 📊 Performance

- **Response Time:** < 100ms for most requests
- **NVIDIA AI:** ~1-3 seconds for generation
- **TTS:** ~2-5 seconds for audio generation
- **Concurrency:** 100+ simultaneous users
- **Storage:** SQLite (default) or PostgreSQL

## 🔒 Security

- JWT-based authentication
- Role-based access control
- Department isolation
- API key encryption
- Audit logging
- Rate limiting

## 🚀 Deployment

### Single Server
```bash
# Build and run
cargo build --release
./target/release/calclaw

# Or use systemd service
sudo cp calclaw.service /etc/systemd/system/
sudo systemctl enable calclaw
sudo systemctl start calclaw
```

### Docker Compose
```yaml
version: '3.8'
services:
  calclaw:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data
      - ./config.toml:/app/config.toml
    environment:
      - RUST_LOG=info
```

### Kubernetes
```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: calclaw
spec:
  replicas: 3
  selector:
    matchLabels:
      app: calclaw
  template:
    metadata:
      labels:
        app: calclaw
    spec:
      containers:
      - name: calclaw
        image: calclaw:latest
        ports:
        - containerPort: 3000
```

## 🤝 Contributing

1. Fork the repository
2. Create feature branch
3. Add tests
4. Submit pull request

### Development Priorities
1. Complete Telegram/WhatsApp integration
2. Add more skill templates
3. Implement advanced Hebrew NLP
4. Build web admin interface
5. Add monitoring dashboard

## 📄 License

MIT License - see [LICENSE](LICENSE) file

## 🆘 Support

- GitHub Issues: Bug reports and feature requests
- Documentation: Check the docs folder
- Community: Discord/Slack (coming soon)

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/)
- NVIDIA NIM API for AI capabilities
- Inspired by [OpenClaw](https://openclaw.ai/)
- Hebrew support via Unicode RTL

---

**CalcLaw Complete** - Your all-in-one organizational AI assistant 🦾🎤🔧🇮🇱