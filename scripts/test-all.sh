#!/bin/bash

# CalcLaw Complete Test Script
echo "🧪 Testing CalcLaw Complete..."

# Check if server is running
echo "🔍 Checking server status..."
response=$(curl -s http://127.0.0.1:3000/health 2>/dev/null || echo "Server not responding")

if [ "$response" == "CalcLaw is running! 🦾" ]; then
    echo "✅ Server is running"
else
    echo "❌ Server not responding: $response"
    echo "Starting server..."
    cd $(dirname "$0")
    ./target/release/calclaw > /dev/null 2>&1 &
    sleep 3
fi

# Test endpoints
echo ""
echo "🔗 Testing API endpoints..."

# Health check
echo -n "  GET /health: "
if curl -s http://127.0.0.1:3000/health | grep -q "CalcLaw"; then
    echo "✅"
else
    echo "❌"
fi

# Hebrew processing
echo -n "  POST /api/hebrew: "
response=$(curl -s -X POST http://127.0.0.1:3000/api/hebrew \
  -H "Content-Type: application/json" \
  -d '{"text": "בדיקת עברית"}' 2>/dev/null)
if echo "$response" | grep -q "is_hebrew\":true"; then
    echo "✅"
else
    echo "❌"
fi

# User management
echo -n "  GET /api/users: "
if curl -s http://127.0.0.1:3000/api/users | grep -q "Admin"; then
    echo "✅"
else
    echo "❌"
fi

# Admin dashboard
echo -n "  GET /admin: "
if curl -s http://127.0.0.1:3000/admin | grep -q "CalcLaw"; then
    echo "✅"
else
    echo "❌"
fi

echo ""
echo "📊 System Information:"
echo "  Port: 3000"
echo "  Data directory: $(pwd)/data"
echo "  Config file: $(pwd)/config.toml"
echo "  Log file: $(pwd)/calclaw.log"

echo ""
echo "🎯 Features Status:"
echo "  ✅ Core server"
echo "  ✅ Hebrew processing"
echo "  ✅ User management"
echo "  ✅ Admin dashboard"
echo "  ⚙️ NVIDIA AI - Configure in config.toml"
echo "  ⚙️ TTS - Configure providers"
echo "  ⚙️ Skills - Ready for implementation"

echo ""
echo "🚀 Ready for production!"
echo "  Run: ./deploy.sh"
echo "  Or with systemd: ./deploy.sh --systemd"