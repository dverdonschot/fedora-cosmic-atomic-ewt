#!/bin/bash
# Quick development build and install script for COSMIC Vimified
# Usage: ./scripts/dev-build.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "╔══════════════════════════════════════════════════════════╗"
echo "║    COSMIC Vimified - Development Build                  ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""

cd "$PROJECT_DIR"

echo "→ Running cargo fmt check..."
cargo fmt --check || {
    echo "⚠️  Formatting issues found. Run 'cargo fmt' to fix."
}

echo ""
echo "→ Running clippy..."
cargo clippy -- -D warnings || {
    echo "⚠️  Clippy warnings found. Please address them."
}

echo ""
echo "→ Building in debug mode..."
cargo build

echo ""
echo "→ Running tests..."
cargo test

echo ""
echo "→ Installing to /usr/local/bin (requires sudo)..."
if [ -f "target/debug/cosmic-vimified" ]; then
    sudo install -Dm755 target/debug/cosmic-vimified /usr/local/bin/cosmic-vimified
    echo "✓ Installed to /usr/local/bin/cosmic-vimified"
else
    echo "✗ Build failed - binary not found"
    exit 1
fi

echo ""
echo "→ Checking for systemd service..."
if systemctl --user is-active cosmic-vimified.service &>/dev/null; then
    echo "→ Restarting cosmic-vimified service..."
    systemctl --user restart cosmic-vimified.service
    echo "✓ Service restarted"
else
    echo "ℹ️  No systemd service found (expected during development)"
fi

echo ""
echo "╔══════════════════════════════════════════════════════════╗"
echo "║    Build Complete!                                       ║"
echo "╚══════════════════════════════════════════════════════════╝"
echo ""
echo "To run manually:"
echo "  cosmic-vimified"
echo ""
echo "To see debug logs:"
echo "  RUST_LOG=cosmic_vimified=debug cosmic-vimified"
echo ""
