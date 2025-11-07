# COSMIC Vimified - Development Guide

**Status:** Planning Phase

## Prerequisites

### System Requirements

- Fedora 43+ with COSMIC desktop
- Rust toolchain (stable)
- COSMIC development libraries

### Development Tools

Install development dependencies:

```bash
# On Fedora Atomic
rpm-ostree install \
    rust \
    cargo \
    rust-analyzer \
    clippy \
    rustfmt \
    gtk4-devel \
    wayland-devel \
    wayland-protocols-devel \
    at-spi2-core-devel \
    just \
    mold

systemctl reboot
```

Or use the development image: `fedora-cosmic-atomic-ewt-dev` (coming soon)

## Getting Started

### Clone the Repository

```bash
git clone https://github.com/dverdonschot/fedora-cosmic-atomic-ewt.git
cd fedora-cosmic-atomic-ewt/cosmic-vimified
```

### Build the Project

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

### Run Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_hint_generation

# With output
cargo test -- --nocapture
```

### Development Workflow

#### Quick Iteration Loop

Use the development script for fast iteration:

```bash
# Build, test, and install locally
./scripts/dev-build.sh

# Or use watch mode for auto-rebuild
cargo watch -x 'build' -s './scripts/install-dev.sh'
```

#### Manual Testing

```bash
# Build and run directly
cargo run

# Or install to /usr/local/bin
cargo build --release
sudo install -Dm755 target/release/cosmic-vimified /usr/local/bin/
cosmic-vimified
```

## Project Structure

```
cosmic-vimified/
├── src/               # Source code
│   ├── main.rs       # Entry point
│   ├── lib.rs        # Library exports
│   ├── config.rs     # Configuration
│   └── ...           # Other modules
├── tests/            # Integration tests
├── examples/         # Example configs
├── docs/             # Documentation
└── scripts/          # Development scripts
```

## Code Style

### Formatting

Use `rustfmt` for consistent formatting:

```bash
cargo fmt
```

### Linting

Use `clippy` for code quality:

```bash
cargo clippy -- -D warnings
```

### Pre-commit Checks

Before committing:

```bash
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## Debugging

### Enable Debug Logging

```bash
RUST_LOG=cosmic_vimified=debug cosmic-vimified
```

### Common Issues

(To be populated as development progresses)

## Testing Strategy

### Unit Tests

Located in each module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_something() {
        // Test implementation
    }
}
```

### Integration Tests

Located in `tests/` directory:

```rust
// tests/test_integration.rs
#[test]
fn test_full_workflow() {
    // Integration test
}
```

### Manual Testing Checklist

- [ ] Global shortcut activation
- [ ] Hints appear correctly
- [ ] Click actions work
- [ ] Config hot-reload works
- [ ] Multi-monitor support
- [ ] Performance acceptable

## Contributing

### Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests and linting
5. Submit a pull request

### Commit Messages

Use conventional commits format:

```
feat: add vim scrolling support
fix: correct hint positioning on multi-monitor
docs: update development guide
test: add tests for hint generation
```

## Architecture Research

During the initial development phase, we need to research:

1. **COSMIC Global Shortcuts:** How to register and listen for global keyboard shortcuts
2. **Layer Shell Integration:** How to create overlay windows using Wayland layer-shell
3. **AT-SPI Querying:** How to efficiently query and filter UI elements
4. **Click Synthesis:** How to programmatically trigger clicks on elements

See [SPEC.md - Open Questions](SPEC.md#open-questions--research-needed) for full list.

## Resources

- [COSMIC Toolkit Book](https://pop-os.github.io/libcosmic-book/)
- [COSMIC Applet Template](https://github.com/edfloreshz/cosmic-applet-template)
- [COSMIC Protocols](https://github.com/pop-os/cosmic-protocols)
- [Rust AT-SPI Bindings](https://docs.rs/atspi/latest/atspi/)
- [Wayland Layer Shell](https://wayland.app/protocols/wlr-layer-shell-unstable-v1)

---

This guide will be expanded as development progresses.
