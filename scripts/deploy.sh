#!/bin/bash

# CalcLaw Complete Deployment Script
echo "🚀 Deploying CalcLaw Complete..."

# Build release version
echo "📦 Building release version..."
source $HOME/.cargo/env
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

# Create data directories
echo "📁 Creating data directories..."
mkdir -p data/skills data/skill_templates data/skill_instances tts_cache logs

# Check if systemd service should be installed
if [ "$1" == "--systemd" ]; then
    echo "⚙️ Installing systemd service..."
    sudo cp calclaw.service /etc/systemd/system/
    sudo systemctl daemon-reload
    sudo systemctl enable calclaw
    echo "✅ Service installed. Start with: sudo systemctl start calclaw"
else
    echo "🎯 Manual deployment ready!"
    echo ""
    echo "To run CalcLaw Complete:"
    echo "  cd $(pwd)"
    echo "  ./target/release/calclaw"
    echo ""
    echo "Or run in background:"
    echo "  ./target/release/calclaw > calclaw.log 2>&1 &"
fi

echo ""
echo "🔗 Access points:"
echo "  Server: http://127.0.0.1:3000"
echo "  Admin: http://127.0.0.1:3000/admin"
echo "  Health: http://127.0.0.1:3000/health"
echo ""
echo "📋 Next steps:"
echo "  1. Edit config.toml with your NVIDIA API key"
echo "  2. Configure TTS providers if needed"
echo "  3. Start creating skills for your organization"
echo ""
echo "🎉 CalcLaw Complete deployment ready!"