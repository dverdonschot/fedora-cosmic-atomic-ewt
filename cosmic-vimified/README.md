# COSMIC Vimified

Keyboard-driven hint navigation for COSMIC desktop environment - bringing Vimium-style mouseless interaction to your entire desktop.

## Status

**Current Phase:** Planning & Specification
**Version:** 0.1.0-dev (Not yet functional)

## What is COSMIC Vimified?

COSMIC Vimified enables you to interact with GUI elements using your keyboard, just like Vimium does for web browsers. Press a shortcut, see hint labels appear over clickable elements, type the label, and interact - all without touching your mouse.

### Features (Planned)

- Super+G activation (avoids conflict with COSMIC's Super+F file explorer)
- Vimium-style hint labels (home row optimized: asdfjkl;)
- Click and right-click actions
- Vim-style scrolling (hjkl)
- Multi-monitor support
- Highly configurable appearance and behavior
- Per-application settings
- One-handed mode (left or right hand)

## Documentation

- [Technical Specification](docs/SPEC.md) - Comprehensive feature and technical specification
- [Architecture](docs/ARCHITECTURE.md) - System architecture and design (TBD)
- [Development Guide](docs/DEVELOPMENT.md) - How to build and contribute (TBD)
- [User Guide](docs/USER_GUIDE.md) - End-user documentation (TBD)

## Quick Start (Future)

### For Users

Once released, installation will be as simple as:

```bash
# On fedora-cosmic-atomic-ewt image
# (Already included in the image)

# Or install manually
rpm-ostree install cosmic-vimified
systemctl reboot
```

### For Developers

```bash
# Clone the repository
git clone https://github.com/dverdonschot/fedora-cosmic-atomic-ewt.git
cd fedora-cosmic-atomic-ewt/cosmic-vimified

# Build and run locally
./scripts/dev-build.sh

# Run tests
cargo test
```

## Configuration

Configuration file location: `~/.config/cosmic-vimified/config.ron`

Example configuration:

```ron
(
    keybindings: (
        activate: "Super+g",
    ),
    appearance: (
        hint_bg_color: "#3daee9",
        hint_text_color: "#ffffff",
        hint_font_size: 16,
    ),
    keyboard_layout: (
        mode: Standard,  // or LeftHanded, RightHanded
    ),
)
```

See [examples/config.ron.example](examples/config.ron.example) for full configuration options.

## Project Goals

1. **Simplicity:** As simple as Vimium - one key to activate, type hint, done
2. **Performance:** Fast activation, minimal resource usage
3. **COSMIC Native:** Built with Rust and libcosmic for perfect integration
4. **Accessibility:** Work with screen readers and assistive technologies
5. **Extensibility:** Easy to configure and customize

## Technical Details

- **Language:** Rust
- **UI Framework:** libcosmic (iced)
- **Element Detection:** AT-SPI (COSMIC's Wayland accessibility protocol)
- **Configuration:** RON format (COSMIC standard)
- **Compositor Integration:** COSMIC layer shell protocol

## Inspired By

- [Vimium](https://github.com/philc/vimium) - The best browser extension for keyboard navigation
- [Hints](https://github.com/AlfredoSequeida/hints) - Vimium for Linux desktop (GTK/X11 based)

## Contributing

This project is in early planning stages. Contributions welcome!

See [DEVELOPMENT.md](docs/DEVELOPMENT.md) for developer setup and guidelines.

## License

TBD

## Roadmap

- [x] Requirements gathering
- [x] Technical specification
- [ ] Architecture design
- [ ] Prototype: Global shortcut registration
- [ ] Prototype: Layer shell overlay
- [ ] Prototype: AT-SPI element detection
- [ ] MVP: Basic click functionality
- [ ] MVP: Hint generation and display
- [ ] Configuration system
- [ ] Right-click support
- [ ] Vim scrolling
- [ ] Multi-monitor support
- [ ] RPM packaging
- [ ] v1.0 Release

## Authors

- dverdonschot - Project creator and maintainer

## Acknowledgments

- System76 for COSMIC desktop environment
- Vimium developers for the inspiration
- COSMIC community for accessibility work
