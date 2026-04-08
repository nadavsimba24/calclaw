# CalcLaw Installation Guide

## 🚀 Quick Start (Linux/macOS)

### 1. Install Rust (if not installed)
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### 2. Clone and Build CalcLaw
```bash
# Clone your repository
git clone <your-repo-url>
cd calclaw

# Build in release mode
cargo build --release

# The binary will be at: ./target/release/calclaw
```

### 3. Run CalcLaw
```bash
# Run the server
./target/release/calclaw

# Or run directly with cargo
cargo run --release
```

### 4. Access the System
- Main server: http://127.0.0.1:3000
- Admin dashboard: http://127.0.0.1:3000/admin
- Health check: http://127.0.0.1:3000/health

## 📦 Docker Installation

```bash
# Build Docker image
docker build -t calclaw .

# Run container
docker run -p 3000:3000 calclaw
```

## 🔧 Configuration

### Environment Variables
Create a `.env` file:
```env
CALCLAW_PORT=3000
CALCLAW_DATABASE_URL=sqlite:calclaw.db
CALCLAW_JWT_SECRET=your-secret-key-here
CALCLAW_TELEGRAM_TOKEN=your-telegram-bot-token
```

### Configuration File
Create `config.toml`:
```toml
[server]
port = 3000
host = "127.0.0.1"

[database]
url = "sqlite:calclaw.db"

[telegram]
enabled = true
token = "your-bot-token"

[whatsapp]
enabled = false
# WhatsApp Business API credentials

[integrations]
monday_enabled = false
salesforce_enabled = false
servicenow_enabled = false
google_sheets_enabled = false
```

## 🎯 Initial Setup

### 1. Create Admin User
```bash
# Using the API (once server is running)
curl -X POST http://127.0.0.1:3000/api/users/register \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Admin User",
    "email": "admin@example.com",
    "password": "securepassword",
    "role": "admin",
    "department": "IT"
  }'
```

### 2. Configure Telegram Bot
1. Create a bot with @BotFather on Telegram
2. Get your bot token
3. Add to config.toml or .env file
4. Restart CalcLaw

### 3. Test Hebrew Support
```bash
curl -X POST http://127.0.0.1:3000/api/hebrew \
  -H "Content-Type: application/json" \
  -d '{"text": "שלום עולם", "user_id": "test"}'
```

## 🔌 Channel Integrations

### Telegram
- Already implemented in basic form
- Uses teloxide Rust library
- Supports Hebrew RTL text

### WhatsApp
- Requires WhatsApp Business API
- Business account approval needed
- Webhook configuration required

### Monday.com
- REST API integration
- OAuth2 authentication
- Board and item management

### Salesforce
- REST/SOAP API options
- OAuth2 or username/password
- Object query and manipulation

### ServiceNow
- REST API Table API
- Basic auth or OAuth
- Incident and ticket management

### Google Sheets
- Google Sheets API v4
- Service account or OAuth2
- Read/write spreadsheet operations

## 🗄️ Database Setup

CalcLaw uses SQLite by default (easy deployment). For production, consider PostgreSQL:

```bash
# Install PostgreSQL (Ubuntu/Debian)
sudo apt-get install postgresql postgresql-contrib

# Create database and user
sudo -u postgres psql
CREATE DATABASE calclaw;
CREATE USER calclaw_user WITH PASSWORD 'yourpassword';
GRANT ALL PRIVILEGES ON DATABASE calclaw TO calclaw_user;
```

Update `config.toml`:
```toml
[database]
url = "postgresql://calclaw_user:yourpassword@localhost/calclaw"
```

## 🚨 Security Notes

1. **Change default passwords** - Always use strong, unique passwords
2. **Use HTTPS in production** - Set up SSL/TLS certificates
3. **Regular backups** - Backup your database regularly
4. **Update dependencies** - Keep Rust and dependencies updated
5. **Monitor logs** - Set up logging and monitoring

## 🆘 Troubleshooting

### Common Issues:

1. **Port already in use**
   ```bash
   # Check what's using port 3000
   sudo lsof -i :3000
   # Kill process or change port in config
   ```

2. **Rust compilation errors**
   ```bash
   # Update Rust
   rustup update
   # Clean and rebuild
   cargo clean
   cargo build --release
   ```

3. **Database connection issues**
   ```bash
   # Check if SQLite file exists
   ls -la calclaw.db
   # Fix permissions
   chmod 644 calclaw.db
   ```

### Getting Help:
- Check logs: `tail -f calclaw.log`
- Enable debug mode: Set `RUST_LOG=debug` environment variable
- Open issue on GitHub repository

## 📈 Next Steps After Installation

1. **Configure your channels** (Telegram, WhatsApp, etc.)
2. **Set up user roles and departments**
3. **Connect business integrations** (Monday.com, Salesforce, etc.)
4. **Customize Hebrew responses** for your organization
5. **Set up monitoring and alerts**

## 🎉 Welcome to CalcLaw!

You now have a basic CalcLaw instance running. The system will evolve with:
- More channel integrations
- Advanced Hebrew NLP
- Better role management
- Data insights engine

Contribute or request features on our GitHub repository!