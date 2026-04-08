# CalcLaw Project Plan

## 🎯 Vision
A specialized, efficient OpenClaw variant for Hebrew-speaking organizations with integrated business tools.

## 📋 Core Requirements

### 1. Hebrew Language Support
- RTL text handling
- Hebrew NLP processing
- Date/time formatting (Hebrew calendar)
- Locale-aware responses

### 2. Channel Integrations
- WhatsApp (via WhatsApp Business API)
- Telegram (already have experience)
- Monday.com (REST API)
- Salesforce (REST/SOAP API)
- ServiceNow (REST API)
- Google Sheets (Google Sheets API)
- FTP/SFTP (file operations)
- File Uploader (web interface)

### 3. User & Role Management
- Admin (full access)
- Superuser (department-level access)
- Department-based permissions
- Audit logging
- Session management

### 4. Data Insights Engine
- Connect to organizational data sources
- Generate insights/reports
- Natural language queries
- Scheduled reporting

### 5. Technical Stack
- **Language**: Rust (performance, safety)
- **Web Framework**: Axum or Actix-web
- **Database**: SQLite (simple) or PostgreSQL (scalable)
- **ORM**: SeaORM or Diesel
- **Authentication**: JWT + OAuth2
- **Message Queue**: Redis or in-memory channels

## 🗺️ Development Phases

### Phase 1: Foundation (2-4 weeks)
- Basic Rust web server
- Hebrew text processing
- Simple role system
- Telegram integration (reuse OpenClaw knowledge)

### Phase 2: Core Integrations (4-6 weeks)
- WhatsApp integration
- Google Sheets connector
- File upload handler
- Basic data query engine

### Phase 3: Business Integrations (4-8 weeks)
- Monday.com integration
- Salesforce connector
- ServiceNow integration
- Advanced role system

### Phase 4: Polish & Deployment (2-4 weeks)
- Easy installation (Docker, install script)
- Configuration UI
- Documentation
- Testing suite

## 🏗️ Architecture Overview

```
CalcLaw Core (Rust)
├── API Server
├── Channel Handlers
│   ├── Telegram
│   ├── WhatsApp
│   └── Web Upload
├── Integration Adapters
│   ├── Monday.com
│   ├── Salesforce
│   ├── ServiceNow
│   └── Google Sheets
├── User/Role Management
├── Hebrew Processor
└── Data Insights Engine
```

## 🔌 OpenClaw Compatibility
- Reuse OpenClaw's model abstraction
- Compatible plugin system
- Similar configuration format
- Migration path from OpenClaw

## 🚀 Getting Started

### Immediate Next Steps:
1. Set up Rust development environment
2. Create basic web server with Hebrew support
3. Implement Telegram channel (copy from OpenClaw)
4. Design role-based permission system

### Quick Win:
Build a Telegram bot with Hebrew support and basic role system first.