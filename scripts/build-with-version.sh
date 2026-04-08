#!/bin/bash

# CalcLaw Complete Build Script with Version Information
# Builds release binaries with embedded version info

set -e

echo "🔨 Building CalcLaw Complete with version info..."

# Get version from Cargo.toml or use default
VERSION=$(grep -m1 '^version =' Cargo.toml | cut -d'"' -f2)
if [ -z "$VERSION" ]; then
    VERSION="1.0.0"
fi

echo "📦 Version: $VERSION"

# Get git commit hash if available
if command -v git &> /dev/null && [ -d .git ]; then
    GIT_COMMIT_HASH=$(git rev-parse --short HEAD)
    echo "🔗 Git commit: $GIT_COMMIT_HASH"
else
    GIT_COMMIT_HASH="unknown"
fi

# Build date
BUILD_DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
echo "📅 Build date: $BUILD_DATE"

# Rust version
RUST_VERSION=$(rustc --version | cut -d' ' -f2)
echo "🦀 Rust version: $RUST_VERSION"

# Clean previous builds
cargo clean

echo ""
echo "📦 Building release binary with version info..."

# Build with environment variables for version info
RUSTFLAGS="--cfg version_info" \
CARGO_ENCODED_RUSTFLAGS="-C link-arg=-Wl,--build-id" \
GIT_COMMIT_HASH="$GIT_COMMIT_HASH" \
BUILD_DATE="$BUILD_DATE" \
RUST_VERSION="$RUST_VERSION" \
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

# Create version info file
cat > release/version.json << EOF
{
  "version": "$VERSION",
  "git_commit_hash": "$GIT_COMMIT_HASH",
  "build_date": "$BUILD_DATE",
  "rust_version": "$RUST_VERSION",
  "build_type": "release"
}
EOF

echo ""
echo "📦 Creating distribution packages..."

# Create tar.gz package
echo "📦 Creating tar.gz package..."
tar -czf release/calclaw-v${VERSION}-linux-x86_64.tar.gz \
    -C release calclaw-linux-x86_64 version.json \
    -C .. README.md LICENSE QUICKSTART.md config.example.toml

# Create DEB package
echo "📦 Creating DEB package..."
mkdir -p debian/DEBIAN
mkdir -p debian/usr/local/bin
mkdir -p debian/etc/calclaw
mkdir -p debian/var/lib/calclaw
mkdir -p debian/usr/share/doc/calclaw

# Control file
cat > debian/DEBIAN/control << EOF
Package: calclaw
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Your Name <your@email.com>
Description: CalcLaw Complete - Enterprise AI Assistant
 Enterprise AI Assistant with Hebrew Support, NVIDIA AI, TTS, and Skills System
Homepage: https://github.com/yourusername/calclaw
EOF

# Copy files
cp release/calclaw-linux-x86_64 debian/usr/local/bin/calclaw
cp config.example.toml debian/etc/calclaw/config.toml
cp README.md debian/usr/share/doc/calclaw/
cp LICENSE debian/usr/share/doc/calclaw/
cp QUICKSTART.md debian/usr/share/doc/calclaw/

# Create post-install script
cat > debian/DEBIAN/postinst << 'EOF'
#!/bin/bash
set -e

echo "CalcLaw Complete v${VERSION} installed successfully!"
echo ""
echo "Next steps:"
echo "1. Configure NVIDIA API key:"
echo "   sudo nano /etc/calclaw/config.toml"
echo "2. Start the service:"
echo "   sudo systemctl start calclaw"
echo "3. Access admin interface:"
echo "   http://localhost:3000/admin"
echo ""
echo "For updates, run: sudo calclaw-update"
EOF

chmod 755 debian/DEBIAN/postinst

# Create pre-remove script
cat > debian/DEBIAN/prerm << 'EOF'
#!/bin/bash
set -e

# Stop service before removal
systemctl stop calclaw 2>/dev/null || true
systemctl disable calclaw 2>/dev/null || true
EOF

chmod 755 debian/DEBIAN/prerm

# Set permissions
chmod 755 debian/usr/local/bin/calclaw
chmod 644 debian/etc/calclaw/config.toml
chmod 644 debian/usr/share/doc/calclaw/*

# Build package
dpkg-deb --build debian release/calclaw_${VERSION}_amd64.deb

# Create update script package
echo "📦 Creating update script package..."
cat > release/calclaw-update << 'EOF'
#!/bin/bash
# CalcLaw Update Script
# This script is installed by the DEB package

curl -fsSL https://raw.githubusercontent.com/yourusername/calclaw/main/update.sh | bash
EOF

chmod +x release/calclaw-update

# Create Docker image
echo "🐳 Building Docker image..."
docker build -t calclaw:latest -t calclaw:v${VERSION} .

echo ""
echo "🎉 Build complete! Files in release/:"
echo ""
ls -la release/
echo ""
echo "📦 Packages created:"
echo "  • release/calclaw-linux-x86_64 - Standalone binary"
echo "  • release/calclaw-v${VERSION}-linux-x86_64.tar.gz - Compressed archive"
echo "  • release/calclaw_${VERSION}_amd64.deb - Debian package"
echo "  • release/calclaw-update - Update script"
echo "  • calclaw:latest - Docker image"
echo "  • calclaw:v${VERSION} - Versioned Docker image"
echo ""
echo "📊 Version information:"
cat release/version.json
echo ""
echo "🚀 Next steps:"
echo "  1. Test the binary: ./release/calclaw-linux-x86_64 --check-updates"
echo "  2. Install DEB package: sudo dpkg -i release/calclaw_${VERSION}_amd64.deb"
echo "  3. Run Docker: docker run -p 3000:3000 calclaw:latest"
echo "  4. Create GitHub release with all packages"
echo ""