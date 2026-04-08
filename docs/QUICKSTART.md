# CalcLaw Complete - Quick Start Guide

Get up and running with CalcLaw Complete in under 5 minutes!

## 🚀 Installation Methods

### Method 1: One-Line Install (Recommended)
```bash
curl -fsSL https://raw.githubusercontent.com/yourusername/calclaw/main/install.sh | bash
```

### Method 2: Docker
```bash
docker run -p 3000:3000 -v ./data:/app/data yourusername/calclaw
```

### Method 3: From Source
```bash
git clone https://github.com/yourusername/calclaw.git
cd calclaw
cargo build --release
./target/release/calclaw
```

## ⚡ 5-Minute Setup

### Step 1: Get NVIDIA API Key
1. Go to [NVIDIA AI Foundation](https://build.nvidia.com/)
2. Sign up (free)
3. Generate API key
4. Copy the key

### Step 2: Configure CalcLaw
```bash
# Copy example config
cp config.example.toml config.toml

# Edit config (add your NVIDIA key)
nano config.toml
```

Edit the `[nvidia]` section:
```toml
[nvidia]
api_key = "your-nvidia-api-key-here"
model = "meta/llama-3.1-8b-instruct"
```

### Step 3: Start CalcLaw
```bash
# Run directly
./target/release/calclaw

# Or with Docker
docker-compose up -d
```

### Step 4: Verify Installation
```bash
# Check health
curl http://localhost:3000/health

# Should return: "CalcLaw Complete is running! 🦾🎤🔧"
```

### Step 5: Access Admin Interface
Open in browser: http://localhost:3000/admin

## 🎯 First Tasks

### 1. Test Hebrew Processing
```bash
curl -X POST http://localhost:3000/api/hebrew \
  -H "Content-Type: application/json" \
  -d '{"text": "שלום עולם מ-CalcLaw"}'
```

### 2. Test AI Integration
```bash
curl -X POST http://localhost:3000/api/nvidia/generate \
  -H "Content-Type: application/json" \
  -d '{
    "prompt": "מה זה CalcLaw?",
    "model": "meta/llama-3.1-8b-instruct"
  }'
```

### 3. Create Your First Skill
```bash
curl -X POST http://localhost:3000/api/skills/create \
  -H "Content-Type: application/json" \
  -d '{
    "template_name": "Daily Sales Report",
    "organization_id": "my-company",
    "department": "Sales",
    "author": "You",
    "config": {
      "recipients": ["you@example.com"],
      "sales_target": 10000
    }
  }'
```

## 📚 Common Use Cases

### For Sales Teams
```bash
# Create daily sales report skill
curl -X POST http://localhost:3000/api/skills/create \
  -H "Content-Type: application/json" \
  -d '{
    "template_name": "Daily Sales Report",
    "organization_id": "acme-corp",
    "department": "Sales",
    "author": "Sales Manager",
    "config": {
      "recipients": ["sales@acme.com"],
      "sales_target": 50000
    }
  }'
```

### For Support Teams
```bash
# Create support ticket alert
curl -X POST http://localhost:3000/api/skills/create \
  -H "Content-Type: application/json" \
  -d '{
    "template_name": "Support Ticket Alert",
    "organization_id": "acme-corp",
    "department": "Support",
    "author": "Support Lead",
    "config": {
      "notification_channels": ["telegram", "email"],
      "priority_levels": ["high", "critical"]
    }
  }'
```

### Generate Hebrew Audio
```bash
curl -X POST http://localhost:3000/api/tts/generate \
  -H "Content-Type: application/json" \
  -d '{
    "text": "דוח מכירות יומי מוכן. סך המכירות: 45,000 ש״ח",
    "provider": "GoogleCloud",
    "voice_id": "he-IL-Standard-A"
  }'
```

## 🔧 Configuration Examples

### Basic Config (`config.toml`)
```toml
[server]
port = 3000
host = "0.0.0.0"

[nvidia]
api_key = "nvapi-your-key-here"
model = "meta/llama-3.1-8b-instruct"
temperature = 0.7
max_tokens = 1024

[skills]
skills_dir = "./data/skills"

[hebrew]
default_language = "he"
rtl_enabled = true
```

### TTS Configuration
```toml
[tts]
default_provider = "GoogleCloud"
default_language = "he-IL"

[tts.api_keys]
google_cloud = "your-google-cloud-key"
elevenlabs = "your-elevenlabs-key"
```

## 🐳 Docker Quick Start

### Single Container
```bash
docker run -d \
  -p 3000:3000 \
  -v ./data:/app/data \
  -v ./config.toml:/app/config.toml \
  --name calclaw \
  yourusername/calclaw:latest
```

### Docker Compose
```bash
# Create docker-compose.yml
cat > docker-compose.yml << EOF
version: '3.8'
services:
  calclaw:
    image: yourusername/calclaw:latest
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data
      - ./config.toml:/app/config.toml
    restart: unless-stopped
EOF

# Start services
docker-compose up -d
```

## 📊 Monitoring

### Check Logs
```bash
# Docker
docker logs calclaw

# Systemd
journalctl -u calclaw -f

# Direct
tail -f calclaw.log
```

### Health Checks
```bash
# Basic health
curl http://localhost:3000/health

# Detailed status
curl http://localhost:3000/api/status
```

### Metrics
```bash
# Prometheus metrics (if enabled)
curl http://localhost:3000/metrics
```

## 🚨 Troubleshooting

### Common Issues

1. **"Connection refused" error**
   ```bash
   # Check if server is running
   ps aux | grep calclaw
   
   # Check port
   netstat -tulpn | grep :3000
   ```

2. **NVIDIA API errors**
   ```bash
   # Verify API key
   curl -X POST "https://integrate.api.nvidia.com/v1/chat/completions" \
     -H "Authorization: Bearer YOUR_KEY" \
     -H "Content-Type: application/json" \
     -d '{"model": "meta/llama-3.1-8b-instruct", "messages": [{"role": "user", "content": "test"}]}'
   ```

3. **Permission errors**
   ```bash
   # Fix data directory permissions
   sudo chown -R $USER:$USER ./data
   ```

### Getting Help
- Check logs: `tail -f calclaw.log`
- Enable debug: `RUST_LOG=debug calclaw`
- GitHub Issues: https://github.com/yourusername/calclaw/issues

## 🎉 Next Steps

1. **Explore the admin interface** at http://localhost:3000/admin
2. **Create skills** for your organization
3. **Integrate with your tools** (Telegram, WhatsApp, etc.)
4. **Scale up** with Docker Compose or Kubernetes
5. **Join the community** for support and updates

## 📞 Support

- **Documentation:** https://github.com/yourusername/calclaw/wiki
- **Issues:** https://github.com/yourusername/calclaw/issues
- **Discord:** [Join our community](https://discord.gg/your-invite)

---

**You're ready to go!** Start automating your organization with CalcLaw Complete today! 🦾🎤🔧