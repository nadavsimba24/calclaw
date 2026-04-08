#!/bin/bash

# CalcLaw Complete Update Script
# Automatically updates CalcLaw to the latest version

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           CalcLaw Complete Update Tool                  ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if running as root
if [ "$EUID" -eq 0 ]; then 
    echo -e "${YELLOW}⚠️  Running as root. It's recommended to run as regular user.${NC}"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Installation directories
INSTALL_DIR="${INSTALL_DIR:-$HOME/.calclaw}"
BIN_DIR="/usr/local/bin"
CONFIG_DIR="/etc/calclaw"
DATA_DIR="/var/lib/calclaw"

# Check if CalcLaw is installed
if [ ! -f "$BIN_DIR/calclaw" ]; then
    echo -e "${RED}❌ CalcLaw not found in $BIN_DIR${NC}"
    echo "Please install CalcLaw first using install.sh"
    exit 1
fi

# Get current version
echo -e "${BLUE}📊 Checking current version...${NC}"
CURRENT_VERSION=$("$BIN_DIR/calclaw" --version 2>/dev/null || echo "0.1.0")
echo -e "${GREEN}✅ Current version: $CURRENT_VERSION${NC}"

# Check for updates
echo ""
echo -e "${BLUE}🔍 Checking for updates...${NC}"

# Get latest version from GitHub API
LATEST_VERSION=$(curl -s https://api.github.com/repos/yourusername/calclaw/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/' || echo "v0.1.0")

if [ "$LATEST_VERSION" = "v$CURRENT_VERSION" ] || [ "$LATEST_VERSION" = "$CURRENT_VERSION" ]; then
    echo -e "${GREEN}✅ You're already on the latest version ($CURRENT_VERSION)${NC}"
    exit 0
fi

echo -e "${GREEN}📦 New version available: $LATEST_VERSION${NC}"
echo -e "${YELLOW}📝 Release notes: https://github.com/yourusername/calclaw/releases/tag/$LATEST_VERSION${NC}"

# Ask for confirmation
read -p "Update to $LATEST_VERSION? (Y/n): " -n 1 -r
echo
if [[ $REPLY =~ ^[Nn]$ ]]; then
    echo "Update cancelled."
    exit 0
fi

echo ""
echo -e "${BLUE}🔄 Step 1: Stopping CalcLaw service...${NC}"

# Stop service if running
if systemctl is-active --quiet calclaw 2>/dev/null; then
    sudo systemctl stop calclaw
    echo -e "${GREEN}✅ Service stopped${NC}"
fi

# Also kill any running processes
pkill -f "calclaw" 2>/dev/null || true

echo ""
echo -e "${BLUE}🔄 Step 2: Backing up current installation...${NC}"

# Create backup directory
BACKUP_DIR="/tmp/calclaw-backup-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Backup files
sudo cp "$BIN_DIR/calclaw" "$BACKUP_DIR/calclaw.bak" 2>/dev/null || true
sudo cp -r "$CONFIG_DIR" "$BACKUP_DIR/config.bak" 2>/dev/null || true
sudo cp -r "$DATA_DIR" "$BACKUP_DIR/data.bak" 2>/dev/null || true

echo -e "${GREEN}✅ Backup created at $BACKUP_DIR${NC}"

echo ""
echo -e "${BLUE}🔄 Step 3: Downloading new version...${NC}"

# Download latest release
cd "$INSTALL_DIR"

# Check if git repository exists
if [ -d "$INSTALL_DIR/.git" ]; then
    echo -e "${GREEN}📥 Pulling latest changes from GitHub...${NC}"
    git pull origin main
else
    echo -e "${GREEN}📥 Downloading latest release...${NC}"
    # Download source code
    rm -rf "$INSTALL_DIR"/*
    git clone https://github.com/yourusername/calclaw.git .
fi

echo ""
echo -e "${BLUE}🔄 Step 4: Building new version...${NC}"

# Build new version
echo -e "${GREEN}🔨 Building release version...${NC}"
cargo build --release

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Build failed! Restoring from backup...${NC}"
    sudo cp "$BACKUP_DIR/calclaw.bak" "$BIN_DIR/calclaw" 2>/dev/null || true
    sudo systemctl start calclaw 2>/dev/null || true
    exit 1
fi

echo ""
echo -e "${BLUE}🔄 Step 5: Installing new binary...${NC}"

# Install new binary
sudo cp "target/release/calclaw" "$BIN_DIR/calclaw"
sudo chmod +x "$BIN_DIR/calclaw"
echo -e "${GREEN}✅ Binary updated${NC}"

echo ""
echo -e "${BLUE}🔄 Step 6: Updating configuration if needed...${NC}"

# Check if config needs migration
if [ -f "$CONFIG_DIR/config.toml" ]; then
    # Compare with new example config
    NEW_CONFIG_EXAMPLE="config.example.toml"
    
    # Check for new sections
    NEW_SECTIONS=$(grep -E "^\[.*\]" "$NEW_CONFIG_EXAMPLE" | sort -u)
    CURRENT_SECTIONS=$(grep -E "^\[.*\]" "$CONFIG_DIR/config.toml" 2>/dev/null | sort -u)
    
    if ! diff <(echo "$NEW_SECTIONS") <(echo "$CURRENT_SECTIONS") >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  New configuration sections detected${NC}"
        echo -e "${YELLOW}   Please review $NEW_CONFIG_EXAMPLE for new options${NC}"
    fi
fi

echo ""
echo -e "${BLUE}🔄 Step 7: Starting service...${NC}"

# Start service
if systemctl is-enabled --quiet calclaw 2>/dev/null; then
    sudo systemctl start calclaw
    echo -e "${GREEN}✅ Service started${NC}"
else
    echo -e "${YELLOW}⚠️  Service not enabled. Starting manually...${NC}"
    "$BIN_DIR/calclaw" > /dev/null 2>&1 &
    echo -e "${GREEN}✅ CalcLaunched in background${NC}"
fi

echo ""
echo -e "${BLUE}🔄 Step 8: Verifying update...${NC}"

# Wait a moment for service to start
sleep 2

# Check new version
NEW_VERSION=$("$BIN_DIR/calclaw" --version 2>/dev/null || echo "unknown")
echo -e "${GREEN}✅ New version: $NEW_VERSION${NC}"

# Check service status
if systemctl is-active --quiet calclaw 2>/dev/null; then
    echo -e "${GREEN}✅ Service is running${NC}"
else
    # Check if process is running
    if pgrep -f "calclaw" >/dev/null; then
        echo -e "${GREEN}✅ Process is running${NC}"
    else
        echo -e "${YELLOW}⚠️  Service not running. Check logs: journalctl -u calclaw${NC}"
    fi
fi

# Health check
echo -n "🧪 Health check: "
if curl -s http://localhost:3000/health 2>/dev/null | grep -q "CalcLaw"; then
    echo -e "${GREEN}✅ Server responding${NC}"
else
    echo -e "${YELLOW}⚠️  Server not responding (might need more time to start)${NC}"
fi

echo ""
echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 CalcLaw Complete Updated Successfully!${NC}"
echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}📋 Update Summary:${NC}"
echo "  From: $CURRENT_VERSION"
echo "  To:   $NEW_VERSION"
echo "  Backup: $BACKUP_DIR"
echo ""
echo -e "${BLUE}🚀 What's New:${NC}"
echo "  Check release notes:"
echo "  https://github.com/yourusername/calclaw/releases/tag/$LATEST_VERSION"
echo ""
echo -e "${BLUE}🔧 Next Steps:${NC}"
echo "  1. Review new configuration options if any"
echo "  2. Test your existing skills"
echo "  3. Report any issues on GitHub"
echo ""
echo -e "${BLUE}📊 Status Check:${NC}"
echo "  Service: sudo systemctl status calclaw"
echo "  Logs:    journalctl -u calclaw -f"
echo "  Health:  curl http://localhost:3000/health"
echo ""
echo -e "${GREEN}Update complete! Thank you for using CalcLaw Complete! 🦾🎤🔧${NC}"