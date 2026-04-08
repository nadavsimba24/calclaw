#!/bin/bash

# CalcLaw Complete Build Script
# Builds release binaries and packages for distribution

set -e

echo "🔨 Building CalcLaw Complete..."

# Clean previous builds
cargo clean

# Build release version
echo "📦 Building release binary..."
cargo build --release

if [ $? -ne 0 ]; then
    echo "❌ Build failed!"
    exit 1
fi

echo "✅ Build successful!"

# Create release directory
mkdir -p release

# Copy binary
cp target/release/calclaw release/calclaw-linux-x86_64

# Strip binary (reduce size)
strip release/calclaw-linux-x86_64

# Create tar.gz package
echo "📦 Creating tar.gz package..."
tar -czf release/calclaw-linux-x86_64.tar.gz \
    -C release calclaw-linux-x86_64 \
    README.md \
    LICENSE \
    config.example.toml

# Create DEB package (for Debian/Ubuntu)
echo "📦 Creating DEB package..."
mkdir -p debian/DEBIAN
mkdir -p debian/usr/local/bin
mkdir -p debian/etc/calclaw
mkdir -p debian/var/lib/calclaw

# Control file
cat > debian/DEBIAN/control << EOF
Package: calclaw
Version: 0.1.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Your Name <your@email.com>
Description: CalcLaw Complete - Enterprise AI Assistant
 Enterprise AI Assistant with Hebrew Support, NVIDIA AI, TTS, and Skills System
EOF

# Copy files
cp release/calclaw-linux-x86_64 debian/usr/local/bin/calclaw
cp config.example.toml debian/etc/calclaw/config.toml

# Set permissions
chmod 755 debian/usr/local/bin/calclaw
chmod 644 debian/etc/calclaw/config.toml

# Build package
dpkg-deb --build debian release/calclaw_0.1.0_amd64.deb

# Create Docker image
echo "🐳 Building Docker image..."
docker build -t calclaw:latest .

echo ""
echo "🎉 Build complete! Files in release/:"
echo ""
ls -la release/
echo ""
echo "📦 Packages created:"
echo "  • release/calclaw-linux-x86_64 - Standalone binary"
echo "  • release/calclaw-linux-x86_64.tar.gz - Compressed archive"
echo "  • release/calclaw_0.1.0_amd64.deb - Debian package"
echo "  • calclaw:latest - Docker image"
echo ""
echo "🚀 Next steps:"
echo "  1. Test the binary: ./release/calclaw-linux-x86_64"
echo "  2. Install DEB package: sudo dpkg -i release/calclaw_0.1.0_amd64.deb"
echo "  3. Run Docker: docker run -p 3000:3000 calclaw:latest"
echo ""