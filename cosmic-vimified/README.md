# COSMIC Vimified

**Vimium-style keyboard navigation for the COSMIC desktop environment**

Stop using your mouse. Navigate your entire desktop with just your keyboard.

## What is COSMIC Vimified?

COSMIC Vimified brings the power of [Vimium](https://github.com/philc/vimium) browser extension to your entire COSMIC desktop. Press `Super+G`, see hint labels appear over every clickable element on your screen, type the label, and interact - all without touching your mouse.

### Demo

```
Press Super+G → Hints appear → Type "as" → Click executed!
```

## Features

- **Keyboard Hints**: Vimium-style hint labels appear on all clickable elements
- **Super+G Activation**: Quick, intuitive activation with a single keybinding
- **Left & Right Click**: Left-click by default, right-click with Shift modifier
- **Vim Scrolling**: Use `hjkl` keys to scroll while hints are active
- **Multi-Monitor Support**: Works seamlessly across all your displays
- **Highly Configurable**: Customize keybindings, appearance, and behavior via RON config files
- **Universal Compatibility**: Works with COSMIC, GTK, Qt, and any AT-SPI compatible application
- **Layer Shell Overlay**: Non-intrusive transparent overlay that stays on top

## How It Works

1. **Press Super+G** to activate hint mode
2. **See hint labels** appear over buttons, links, and clickable elements
3. **Type the label** (e.g., "as", "df") to select an element
4. **Element is clicked** automatically
5. **Press ESC** to cancel anytime

### Advanced Usage

- **Right-click**: Hold `Shift` while typing hint label
- **Vim scrolling**: Use `h` (left), `j` (down), `k` (up), `l` (right) to scroll
- **Quick cancel**: Press `ESC` to exit hint mode

## Installation

### From BlueBuild Image (Recommended)

COSMIC Vimified is pre-installed on the `fedora-cosmic-atomic-ewt` image:

```bash
rpm-ostree rebase ostree-image-signed:docker://ghcr.io/dverdonschot/fedora-cosmic-atomic-ewt:latest
systemctl reboot
```

### Manual Build

```bash
# Clone the repository
git clone https://github.com/dverdonschot/fedora-cosmic-atomic-ewt
cd fedora-cosmic-atomic-ewt/cosmic-vimified

# Build and install
cargo build --release
sudo cp target/release/cosmic-vimified /usr/bin/

# Set up udev rules for uinput permissions
sudo cp files/system/etc/udev/rules.d/99-cosmic-vimified.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

## Configuration

COSMIC Vimified uses `cosmic-config` for configuration. Configuration files are in RON format.

Default config location: `~/.config/cosmic-vimified/config.ron`

### Example Configuration

```ron
(
    keybindings: (
        activate: "Super+g",
        cancel: "Escape",
        scroll_left: "h",
        scroll_down: "j",
        scroll_up: "k",
        scroll_right: "l",
    ),
    appearance: (
        hint_bg_color: "#3daee9",
        hint_text_color: "#ffffff",
        hint_font_size: 16,
        hint_border_radius: 4,
        hint_opacity: 0.95,
    ),
    keyboard_layout: (
        mode: Standard,  // Options: Standard, LeftHanded, RightHanded, Custom
        custom_chars: None,
    ),
    behavior: (
        scroll_amount: 100,
        return_cursor: false,
    ),
)
```

See [`docs/CONFIGURATION.md`](docs/CONFIGURATION.md) for complete configuration reference (coming soon).

## Technical Architecture

- **Language**: Rust
- **UI Framework**: libcosmic (iced-based)
- **Element Detection**: AT-SPI (Assistive Technology Service Provider Interface)
- **Overlay Rendering**: Wayland Layer Shell protocol
- **Input Synthesis**: uinput kernel module
- **Configuration**: cosmic-config (RON format)

### How It Works Internally

1. **Activation**: Listens for `Super+G` keybinding via COSMIC
2. **Element Detection**: Queries AT-SPI accessibility tree for clickable elements
3. **Hint Generation**: Generates optimal hint labels using home-row keys (asdfjkl;)
4. **Overlay Rendering**: Creates transparent full-screen layer-shell overlay
5. **User Input**: Captures keyboard input and filters matching hints
6. **Click Synthesis**: Uses uinput to synthesize mouse clicks at element coordinates

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) for detailed technical documentation (coming soon).

## Development

### Prerequisites

- Rust toolchain (stable)
- COSMIC desktop environment (Fedora 43+)
- AT-SPI enabled
- uinput kernel module loaded

### Building from Source

```bash
# Clone repository
git clone https://github.com/dverdonschot/fedora-cosmic-atomic-ewt
cd fedora-cosmic-atomic-ewt/cosmic-vimified

# Check dependencies
cargo check

# Run in development mode
cargo run

# Build release binary
cargo build --release
```

### Running Tests

```bash
# Run unit tests
cargo test

# Run benchmarks
cargo bench

# Run with logging
RUST_LOG=debug cargo run
```

### Project Structure

```
cosmic-vimified/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application struct (libcosmic App trait)
│   ├── overlay/             # Overlay window and rendering
│   │   ├── mod.rs
│   │   ├── renderer.rs
│   │   ├── styles.rs
│   │   ├── widgets.rs
│   │   └── input.rs
│   ├── detection/           # AT-SPI element detection
│   │   ├── mod.rs
│   │   ├── atspi.rs
│   │   ├── filters.rs
│   │   └── types.rs
│   ├── hints/               # Hint generation algorithm
│   │   ├── mod.rs
│   │   ├── generator.rs
│   │   └── layout.rs
│   ├── actions/             # Click and scroll actions
│   │   ├── mod.rs
│   │   ├── click.rs
│   │   ├── scroll.rs
│   │   └── uinput.rs
│   ├── config/              # Configuration management
│   │   ├── mod.rs
│   │   └── schema.rs
│   └── display/             # Multi-monitor support
│       └── monitors.rs
├── docs/
│   ├── TASK_LIST.md         # Development roadmap
│   ├── TECHNICAL_RESEARCH.md # Technical research and decisions
│   ├── USER_GUIDE.md        # User documentation (coming soon)
│   ├── CONFIGURATION.md     # Configuration reference (coming soon)
│   └── ARCHITECTURE.md      # Technical architecture (coming soon)
├── files/
│   └── system/
│       └── etc/
│           └── udev/
│               └── rules.d/
│                   └── 99-cosmic-vimified.rules
├── tests/                   # Integration tests
├── benches/                 # Benchmarks
├── examples/                # Example configurations
└── Cargo.toml
```

## Roadmap

See [`docs/TASK_LIST.md`](docs/TASK_LIST.md) for detailed development roadmap.

### Current Status

- ✅ **Phase 0**: Foundation & Research - **COMPLETE**
- 🚧 **Phase 1**: libcosmic Application Scaffold - **IN PROGRESS**

### Upcoming Features

- 🎯 **MVP (v0.1.0)** - Basic hint navigation (Target: ~2 weeks)
  - Super+G activation
  - Hint display on clickable elements
  - Left-click on hint selection
  - ESC to cancel

- 🚀 **v1.0.0** - Full featured release (Target: ~4-5 weeks)
  - Right-click support
  - Vim scrolling (hjkl)
  - Configuration system
  - Multi-monitor support
  - BlueBuild packaging
  - Comprehensive documentation

## Troubleshooting

### Hints don't appear

- Ensure AT-SPI is enabled: `gsettings get org.gnome.desktop.interface toolkit-accessibility`
- Check if applications support accessibility (most COSMIC, GTK, Qt apps do)
- Try running with debug logging: `RUST_LOG=debug cosmic-vimified`

### Permission errors with uinput

- Verify udev rules are installed: `ls /etc/udev/rules.d/99-cosmic-vimified.rules`
- Reload udev rules: `sudo udevadm control --reload-rules && sudo udevadm trigger`
- Check uinput module is loaded: `lsmod | grep uinput`
- Add user to `input` group: `sudo usermod -aG input $USER` (then log out/in)

### Clicks miss the target

- Check monitor scaling settings
- Verify multi-monitor setup is detected correctly
- Report issue with application name and coordinates

See [`docs/TROUBLESHOOTING.md`](docs/TROUBLESHOOTING.md) for more help (coming soon).

## Contributing

Contributions welcome! This is currently a personal project integrated into a BlueBuild image.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Format code: `cargo fmt`
6. Check with clippy: `cargo clippy -- -D warnings`
7. Submit a pull request

## License

GPL-3.0-or-later

## Credits

- Inspired by [Vimium](https://github.com/philc/vimium) browser extension
- Built on [libcosmic](https://github.com/pop-os/libcosmic) by System76
- Uses [AT-SPI](https://gitlab.gnome.org/GNOME/at-spi2-core) for accessibility
- Python reference implementation: [hints](https://github.com/AlfredoSequeida/hints)

## Related Projects

- [Vimium](https://github.com/philc/vimium) - The original browser extension
- [hints](https://github.com/AlfredoSequeida/hints) - Python implementation for X11
- [keynav](https://github.com/jordansissel/keynav) - Alternative keyboard navigation
- [warpd](https://github.com/rvaiya/warpd) - Modal keyboard-driven pointer manipulation

---

**Let's make COSMIC keyboard-first! 🚀**
