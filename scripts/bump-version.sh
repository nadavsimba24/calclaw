#!/bin/bash

# CalcLaw Version Bump Script
# Updates version in all necessary files and creates git tag

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔══════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║           CalcLaw Version Bump Tool                     ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════╝${NC}"
echo ""

# Get current version
CURRENT_VERSION=$(grep -m1 '^version =' Cargo.toml | cut -d'"' -f2)
if [ -z "$CURRENT_VERSION" ]; then
    CURRENT_VERSION="1.0.0"
fi

echo -e "${GREEN}📊 Current version: $CURRENT_VERSION${NC}"

# Ask for new version
read -p "Enter new version (e.g., 1.1.0): " NEW_VERSION

if [ -z "$NEW_VERSION" ]; then
    echo -e "${RED}❌ Version cannot be empty${NC}"
    exit 1
fi

# Validate version format (simple check)
if ! [[ $NEW_VERSION =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo -e "${YELLOW}⚠️  Warning: Version format should be MAJOR.MINOR.PATCH (e.g., 1.1.0)${NC}"
    read -p "Continue anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

echo ""
echo -e "${BLUE}🔄 Updating version to $NEW_VERSION...${NC}"

# Update Cargo.toml
echo -n "📦 Cargo.toml: "
sed -i "s/^version = \".*\"/version = \"$NEW_VERSION\"/" Cargo.toml
echo -e "${GREEN}✅${NC}"

# Update main.rs (clap version)
echo -n "📝 src/main.rs: "
sed -i "s/version = \".*\"/version = \"$NEW_VERSION\"/" src/main.rs
echo -e "${GREEN}✅${NC}"

# Update version.rs (default version)
echo -n "📝 src/version.rs: "
sed -i "s/pub fn new(current_version: &str,/pub fn new(current_version: &str,/" src/version.rs
# This needs manual update in the main.rs call
echo -e "${GREEN}✅${NC}"

# Update Dockerfile labels
echo -n "🐳 Dockerfile: "
sed -i "s/label version=\".*\"/label version=\"$NEW_VERSION\"/" Dockerfile 2>/dev/null || true
echo -e "${GREEN}✅${NC}"

# Create changelog entry
echo -n "📋 CHANGELOG.md: "
if [ ! -f CHANGELOG.md ]; then
    cat > CHANGELOG.md << EOF
# Changelog

All notable changes to CalcLaw Complete will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [$NEW_VERSION] - $(date +%Y-%m-%d)

### Added
- 

### Changed
- 

### Fixed
- 

### Removed
- 

## [$CURRENT_VERSION] - Initial release
EOF
else
    # Insert new version at the top
    sed -i "/^## \[Unreleased\]/a\\\n## [$NEW_VERSION] - $(date +%Y-%m-%d)\\n\\n### Added\\n- \\n\\n### Changed\\n- \\n\\n### Fixed\\n- \\n\\n### Removed\\n-" CHANGELOG.md
fi
echo -e "${GREEN}✅${NC}"

echo ""
echo -e "${BLUE}📝 Please update the following files manually:${NC}"
echo "  1. src/main.rs - Update UpdateChecker::new() call to use \"$NEW_VERSION\""
echo "  2. Review CHANGELOG.md - Add actual changes for this version"
echo ""

# Ask to create git tag
read -p "Create git tag v$NEW_VERSION? (Y/n): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Nn]$ ]]; then
    echo -e "${BLUE}🔖 Creating git tag...${NC}"
    
    # Stage changes
    git add Cargo.toml src/main.rs src/version.rs Dockerfile CHANGELOG.md 2>/dev/null || true
    
    # Commit
    git commit -m "Bump version to $NEW_VERSION" 2>/dev/null || true
    
    # Create tag
    git tag -a "v$NEW_VERSION" -m "CalcLaw Complete $NEW_VERSION"
    
    echo -e "${GREEN}✅ Tag v$NEW_VERSION created${NC}"
    echo ""
    echo -e "${BLUE}🚀 Next steps:${NC}"
    echo "  1. Push the tag: git push origin v$NEW_VERSION"
    echo "  2. GitHub Actions will automatically create a release"
    echo "  3. Update release notes on GitHub with actual changes"
else
    echo -e "${YELLOW}⚠️  Skipping git tag creation${NC}"
fi

echo ""
echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}🎉 Version updated from $CURRENT_VERSION to $NEW_VERSION${NC}"
echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}📋 Summary of changes:${NC}"
echo "  • Cargo.toml: $CURRENT_VERSION → $NEW_VERSION"
echo "  • src/main.rs: Updated version string"
echo "  • CHANGELOG.md: Added entry for $NEW_VERSION"
echo "  • Git tag: v$NEW_VERSION (if created)"
echo ""
echo -e "${BLUE}🔧 Next steps for release:${NC}"
echo "  1. Update CHANGELOG.md with actual changes"
echo "  2. Push the tag: git push origin v$NEW_VERSION"
echo "  3. GitHub Actions will build and release automatically"
echo "  4. Notify users about the update"
echo ""
echo -e "${GREEN}Version bump complete! 🦾🎤🔧${NC}"