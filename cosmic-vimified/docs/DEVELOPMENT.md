# COSMIC Vimified - Development Guide

Guide for setting up a development environment for COSMIC Vimified on Fedora Atomic.

## Prerequisites

### System Requirements

- Fedora 43+ with COSMIC desktop
- Rust toolchain (stable)
- Development tools

### Development on Fedora Atomic

Fedora Atomic (ostree-based) systems require special handling for development dependencies.

**Option 1: Use Toolbox (Recommended)**

Toolbox provides a containerized development environment:

```bash
# Create a Fedora toolbox
toolbox create cosmic-dev

# Enter the toolbox
toolbox enter cosmic-dev

# Install development dependencies
sudo dnf install -y \
    rust cargo \
    systemd-devel \
    at-spi2-core-devel \
    pkg-config \
    gcc \
    gcc-c++

# Navigate to project
cd ~/fedora-cosmic-atomic-ewt/cosmic-vimified

# Now cargo commands work
cargo check
cargo build
cargo run
```

**Option 2: Layer packages on host system**

Install development dependencies directly (requires reboot):

```bash
# Install development dependencies
rpm-ostree install \
    systemd-devel \
    at-spi2-core-devel \
    pkg-config

# Reboot to apply
systemctl reboot

# After reboot, build normally
cd ~/fedora-cosmic-atomic-ewt/cosmic-vimified
cargo check
```

**Option 3: Use distrobox**

Similar to toolbox but with more flexibility:

```bash
# Install distrobox if not available
rpm-ostree install distrobox
systemctl reboot

# Create Fedora container
distrobox create -n cosmic-dev -i fedora:43

# Enter container
distrobox enter cosmic-dev

# Install dependencies
sudo dnf install -y systemd-devel at-spi2-core-devel pkg-config gcc rust cargo

# Development commands work here
cargo check
```

## Required System Dependencies

### Runtime Dependencies

These are needed to run cosmic-vimified:

- `systemd-libs` (libudev) - Usually pre-installed
- `at-spi2-core` - Accessibility infrastructure
- `dbus` - Session bus for AT-SPI

### Build Dependencies

These are needed to compile cosmic-vimified:

- `systemd-devel` - libudev development headers
- `at-spi2-core-devel` - AT-SPI development headers (future)
- `pkg-config` - Dependency discovery
- `gcc` / `gcc-c++` - C compiler for native dependencies
- `rust` / `cargo` - Rust toolchain

Install build dependencies in toolbox:

```bash
sudo dnf install -y \
    systemd-devel \
    pkg-config \
    gcc \
    gcc-c++
```

## Setting up Rust

### Install Rust (if not available)

```bash
# Using rustup (recommended)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Source cargo environment
source ~/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### Update Rust

```bash
rustup update stable
```

## Building the Project

### First-time setup

```bash
# Clone repository (if not already)
git clone https://github.com/dverdonschot/fedora-cosmic-atomic-ewt
cd fedora-cosmic-atomic-ewt/cosmic-vimified

# Check dependencies (downloads and checks, doesn't build)
cargo check

# Build debug version
cargo build

# Build release version (optimized)
cargo build --release
```

### Development workflow

```bash
# Run in development mode (with debug logging)
RUST_LOG=debug cargo run

# Run with specific log level
RUST_LOG=info cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run benchmarks
cargo bench

# Format code
cargo fmt

# Lint code
cargo clippy

# Check for warnings
cargo clippy -- -D warnings
```

### Clean build

```bash
# Remove build artifacts
cargo clean

# Rebuild from scratch
cargo build
```

## Project Structure

```
cosmic-vimified/
├── Cargo.toml              # Project manifest and dependencies
├── src/
│   └── main.rs            # Application entry point
├── docs/
│   ├── TASK_LIST.md       # Development roadmap
│   ├── TECHNICAL_RESEARCH.md
│   └── DEVELOPMENT.md     # This file
├── tests/                 # Integration tests
├── benches/               # Performance benchmarks
├── examples/              # Example configurations
└── files/
    └── system/            # Files to copy to system
```

## Running Tests

### Unit tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Integration tests

```bash
# Run integration tests only
cargo test --test '*'
```

### Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench hint_generation
```

## Debugging

### Enable debug logging

```bash
# Debug level
RUST_LOG=debug cargo run

# Trace level (very verbose)
RUST_LOG=trace cargo run

# Module-specific logging
RUST_LOG=cosmic_vimified=debug cargo run
```

### Using rust-gdb

```bash
# Build with debug symbols
cargo build

# Run with gdb
rust-gdb target/debug/cosmic-vimified
```

### Using lldb

```bash
# Build with debug symbols
cargo build

# Run with lldb
rust-lldb target/debug/cosmic-vimified
```

## Common Issues

### "libudev.pc not found"

**Problem:** Missing systemd-devel package

**Solution:** Install systemd-devel in toolbox:
```bash
sudo dnf install systemd-devel
```

### "cannot find crate for libcosmic"

**Problem:** libcosmic is a git dependency and might be slow to clone

**Solution:** Wait for git clone to complete. First build can take 10-15 minutes.

### "linker `cc` not found"

**Problem:** Missing C compiler

**Solution:** Install gcc:
```bash
sudo dnf install gcc
```

### Permission denied: /dev/uinput

**Problem:** User doesn't have access to uinput device

**Solution:** Install udev rules and add user to input group:
```bash
sudo cp files/system/etc/udev/rules.d/99-cosmic-vimified.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
sudo usermod -aG input $USER
# Log out and back in
```

## Code Style

### Formatting

We use rustfmt with default settings:

```bash
# Format all code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Linting

We use clippy for linting:

```bash
# Run clippy
cargo clippy

# Fail on warnings
cargo clippy -- -D warnings
```

### Code organization

- Keep modules small and focused
- Use descriptive names
- Add doc comments to public APIs
- Write unit tests for algorithms
- Use `Result` for error handling
- Prefer `anyhow::Result` for application errors
- Use `thiserror` for library errors

## Git Workflow

### Making changes

```bash
# Create feature branch (optional)
git checkout -b feature/my-feature

# Make changes
# ... edit files ...

# Format and check
cargo fmt
cargo clippy -- -D warnings
cargo test

# Commit changes
git add .
git commit -m "feat: add my feature"
```

### Commit message style

We follow conventional commits:

- `feat:` - New feature
- `fix:` - Bug fix
- `docs:` - Documentation changes
- `style:` - Code style changes (formatting, etc.)
- `refactor:` - Code refactoring
- `test:` - Adding or updating tests
- `chore:` - Maintenance tasks

## Performance Profiling

### Using perf

```bash
# Build release with debug symbols
cargo build --release

# Profile with perf
perf record --call-graph=dwarf ./target/release/cosmic-vimified

# View results
perf report
```

### Using flamegraph

```bash
# Install cargo-flamegraph
cargo install flamegraph

# Generate flamegraph (requires root)
sudo cargo flamegraph
```

## Documentation

### Building docs

```bash
# Build project documentation
cargo doc

# Build and open in browser
cargo doc --open

# Include private items
cargo doc --document-private-items
```

### Writing documentation

- Add doc comments with `///` for public items
- Add examples in doc comments
- Use `//!` for module-level documentation
- Include code examples that compile (test with `cargo test`)

Example:
```rust
/// Generates hint labels for detected elements.
///
/// # Arguments
///
/// * `num_elements` - Number of elements to generate hints for
///
/// # Returns
///
/// Vector of hint labels
///
/// # Example
///
/// ```
/// let hints = generate_hints(10);
/// assert_eq!(hints.len(), 10);
/// ```
pub fn generate_hints(num_elements: usize) -> Vec<String> {
    // ...
}
```

## Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [libcosmic Documentation](https://pop-os.github.io/libcosmic/)
- [libcosmic Examples](https://github.com/pop-os/libcosmic/tree/master/examples)
- [AT-SPI Documentation](https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/)

## Getting Help

- Review [TASK_LIST.md](TASK_LIST.md) for project status
- Check [TECHNICAL_RESEARCH.md](TECHNICAL_RESEARCH.md) for technical details
- Search existing issues in repository
- Ask in COSMIC Discord: https://discord.gg/cosmic

---

Happy hacking! 🚀
