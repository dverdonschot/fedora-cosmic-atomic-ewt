# COSMIC Vimified - Technical Specification

**Version:** 0.1.0-draft
**Status:** Planning
**Last Updated:** 2025-11-07

## Project Overview

COSMIC Vimified is a keyboard-driven hint navigation system for the COSMIC desktop environment, inspired by Vimium browser extension. It enables mouseless interaction with GUI elements through keyboard shortcuts and visual hint labels.

## 1. Activation & Trigger Mechanism

### Primary Activation
- **Shortcut:** `Super+G`
- **Rationale:** `Super+F` conflicts with COSMIC's default file explorer shortcut
- **Behavior:** Toggle mode
  - First press: Show hints overlay
  - Second press (or ESC): Hide hints overlay
  - Typing hint label: Execute action and hide overlay

### Activation Modes
- **Single mode approach:** One activation shortcut triggers hint display
- **Action modifiers:** Actions determined by modifiers while typing hint:
  - No modifier: Left click (default)
  - `Shift + hint`: Right click
  - Scrolling via vim keys when no hints typed

### Technical Considerations
- Must register global keyboard shortcut via COSMIC compositor
- Should not conflict with application-specific shortcuts
- Must work even when applications have focus

## 2. Feature Scope

### Phase 1 (MVP - Minimum Viable Product)
1. **Single Click** ✓
   - Left mouse button click on hinted element
   - Primary interaction method

2. **Right Click** ✓
   - Activated with `Shift + hint`
   - Context menu access

3. **Vim-style Scrolling** ✓
   - `h`: Scroll left
   - `j`: Scroll down
   - `k`: Scroll up
   - `l`: Scroll right
   - Active when hints are visible but no hint typed yet

### Deferred Features (Post-MVP)
- Double-click
- Drag and drop
- Hover actions
- Custom action chains

## 3. Hint Label Generation

### Label Algorithm
- **Primary:** Vimium-style two-character combinations
- **Sequence:** Home row priority (asdfjkl;)
- **Generation order:**
  ```
  a, s, d, f, j, k, l, ;,
  aa, as, ad, af, aj, ak, al, a;,
  sa, ss, sd, sf, sj, sk, sl, s;,
  ...
  ```

### Keyboard Layouts
Support multiple keyboard modes via configuration:

1. **Standard (two-handed):**
   - Uses: `asdfjkl;` (home row)

2. **Left-handed:**
   - Uses: `asdfgqwert` (left side only)

3. **Right-handed:**
   - Uses: `jkl;uiop` (right side only)

### Visual Generation Rules
- **Adaptive sizing:**
  - Hints scale based on number of detected elements
  - Font size adapts to screen DPI
  - Minimum readable size: 14pt
  - Maximum size: 24pt

- **Placement:**
  - Position near top-left corner of target element
  - Avoid overlapping existing UI elements when possible
  - Smart positioning to stay within screen bounds

## 4. Element Detection Strategy

### Target Elements (via AT-SPI)

#### Priority 1 - Essential Clickable Elements
- Buttons (all types)
- Links and hyperlinks
- Input fields (text, checkbox, radio, etc.)
- Icons in panel/dock
- System tray items

#### Priority 2 - Navigation Elements
- Menu items
- Dropdown options
- Tabs
- Tree view items
- List items

#### Priority 3 - Window Controls
- Window title bars
- Close/minimize/maximize buttons
- Window borders (for resize)
- Split pane dividers

### Detection Method
- Query AT-SPI for all focusable/clickable elements
- Filter based on:
  - Element visibility (is displayed on screen)
  - Element position (within current viewport)
  - Element type (matches configured types)
  - Application allow/deny list

### Exclusions
- Hidden elements
- Disabled elements (optional via config)
- Elements outside visible area
- Duplicate element positions

## 5. Application Scope

### Priority Tiers

**Tier 1 - Full Support (Focus Area)**
- COSMIC native applications (iced/libcosmic)
- Guaranteed compatibility and performance
- Primary testing target

**Tier 2 - Best Effort Support**
- GTK applications
- Qt applications
- Firefox and other browsers (complements Vimium for browser UI)
- Electron applications

**Tier 3 - Community Support**
- X11 applications via XWayland
- Legacy applications
- Custom toolkits

### Application Control

#### Allow/Deny Lists
Configuration options in `~/.config/cosmic-vimified/config.ron`:

```ron
(
    application_control: (
        mode: AllowList, // or DenyList, or All
        allowed_apps: [
            "org.cosmic.Files",
            "org.cosmic.Terminal",
            "firefox",
        ],
        denied_apps: [
            "password-manager",
            "banking-app",
        ],
    ),
)
```

#### Use Cases for Deny List
- Password managers (security)
- Banking applications (avoid accidental clicks)
- Games (avoid shortcut conflicts)
- Applications with native vim-mode

## 6. Configuration & Customization

### Configuration File Location
- **System:** `/etc/cosmic-vimified/config.ron`
- **User:** `~/.config/cosmic-vimified/config.ron`
- User config overrides system config

### Configurable Parameters

#### Keybindings
```ron
(
    keybindings: (
        activate: "Super+g",     // Main activation shortcut
        cancel: "Escape",
        scroll_left: "h",
        scroll_down: "j",
        scroll_up: "k",
        scroll_right: "l",
        modifier_right_click: "Shift",
    ),
)
```

#### Visual Appearance
```ron
(
    appearance: (
        hint_bg_color: "#3daee9",        // Light blue background
        hint_text_color: "#ffffff",       // White text
        hint_border_color: "#1d99f3",     // Darker blue border
        hint_border_width: 2,             // pixels
        hint_font_family: "Hack Nerd Font",
        hint_font_size: 16,               // points
        hint_padding: 4,                  // pixels
        hint_border_radius: 4,            // pixels
        hint_opacity: 0.95,               // 0.0 to 1.0
    ),
)
```

#### Element Detection
```ron
(
    detection: (
        enabled_element_types: [
            "Button",
            "Link",
            "MenuItem",
            "Icon",
            "Input",
            "Tab",
            "ListItem",
            "WindowControl",
        ],
        include_disabled_elements: false,
        detection_refresh_rate: 100,     // milliseconds
    ),
)
```

#### Keyboard Layouts
```ron
(
    keyboard_layout: (
        mode: Standard,  // Standard, LeftHanded, RightHanded, Custom
        custom_keys: "asdfjkl;", // Used when mode: Custom
    ),
)
```

#### Per-Application Settings
```ron
(
    per_app_config: {
        "firefox": (
            hint_bg_color: "#ff9500",  // Orange for Firefox
            enabled_element_types: ["MenuItem", "Button"], // Only menus, not webpage
        ),
        "org.cosmic.Terminal": (
            keybindings: (
                activate: "Super+g",  // Consistent across all apps
            ),
        ),
    },
)
```

### Configuration Management
- Hot-reload configuration on file change (via file watcher)
- Validation on load with helpful error messages
- Fallback to defaults if config invalid
- Config migration on version updates

## 7. Repository Structure

```
fedora-cosmic-atomic-ewt/
├── cosmic-vimified/              # Top-level applet directory
│   ├── Cargo.toml                # Rust project manifest
│   ├── Cargo.lock
│   ├── README.md                 # Project overview & quick start
│   ├── LICENSE                   # License file
│   ├── .gitignore
│   │
│   ├── docs/                     # Documentation
│   │   ├── SPEC.md              # This file
│   │   ├── ARCHITECTURE.md      # Technical architecture
│   │   ├── DEVELOPMENT.md       # Developer guide
│   │   └── USER_GUIDE.md        # End-user documentation
│   │
│   ├── src/                      # Source code
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # Library exports
│   │   ├── config.rs            # Configuration management
│   │   ├── hint_generator.rs   # Label generation logic
│   │   ├── element_detector.rs # AT-SPI element detection
│   │   ├── overlay.rs           # Hint overlay rendering
│   │   ├── input_handler.rs    # Keyboard input processing
│   │   ├── action_executor.rs  # Click/scroll execution
│   │   └── app_filter.rs        # Allow/deny list logic
│   │
│   ├── tests/                    # Integration tests
│   │   ├── test_hint_generation.rs
│   │   ├── test_element_detection.rs
│   │   └── test_config.rs
│   │
│   ├── examples/                 # Example configs & usage
│   │   └── config.ron.example
│   │
│   ├── scripts/                  # Development & build scripts
│   │   ├── dev-build.sh         # Quick local build
│   │   ├── test-local.sh        # Run tests locally
│   │   └── install-dev.sh       # Install for development
│   │
│   └── packaging/                # Packaging files
│       ├── cosmic-vimified.spec # RPM spec file
│       └── cosmic-vimified.service # Systemd service (if needed)
│
├── recipes/
│   ├── recipe.yml               # Production image
│   └── recipe-dev.yml           # Development image (includes dev tools)
│
├── files/
│   ├── scripts/
│   │   └── setup-cosmic-vimified.sh  # Image build script
│   └── system/
│       └── etc/
│           └── cosmic-vimified/
│               └── config.ron   # Default system config
│
└── README.md                    # Repository overview
```

### Module Independence
- `cosmic-vimified/` should be decoupled enough to become standalone repo later
- All build logic self-contained in cosmic-vimified directory
- Image integration via scripts, not tight coupling

## 8. Build & Packaging Integration

### Build Methods

#### Method 1: Source Build During Image Creation (Initial)
```yaml
# In recipes/recipe.yml
- type: script
  scripts:
    - setup-cosmic-vimified.sh
```

Script builds from source:
```bash
#!/bin/bash
set -oue pipefail

cd /tmp
git clone file:///cosmic-vimified cosmic-vimified-build || cp -r /cosmic-vimified cosmic-vimified-build
cd cosmic-vimified-build
cargo build --release
install -Dm755 target/release/cosmic-vimified /usr/bin/cosmic-vimified
install -Dm644 examples/config.ron.example /etc/cosmic-vimified/config.ron
```

#### Method 2: RPM Package (Future)
- Create proper RPM spec file
- Build SRPM in COPR
- Install via DNF module:
  ```yaml
  - type: dnf
    repos:
      copr:
        - youruser/cosmic-vimified
    install:
      packages:
        - cosmic-vimified
  ```

### Separate Build Pipeline

**Goal:** Update running system without rebuilding entire image

#### Option A: Local RPM Build
```bash
# In cosmic-vimified/scripts/build-rpm.sh
cargo build --release
rpmbuild -bb packaging/cosmic-vimified.spec
sudo rpm-ostree install ./cosmic-vimified*.rpm
systemctl reboot
```

#### Option B: COPR Auto-builds
- Push to `cosmic-vimified` branch
- COPR webhook builds RPM automatically
- Install with `rpm-ostree install cosmic-vimified`
- No full image rebuild required

#### Option C: Development Install (Fastest Iteration)
```bash
# In cosmic-vimified/scripts/install-dev.sh
cargo build --release
sudo install -Dm755 target/release/cosmic-vimified /usr/local/bin/cosmic-vimified
systemctl --user restart cosmic-vimified
```

### Distribution Methods
- **Primary:** RPM package (standard Fedora method)
- **Alternative:** Flatpak (for broader distribution)
- **Development:** Direct binary install

## 9. Development Workflow

### Local Development Iteration

#### Quick Build & Test Loop
```bash
# cosmic-vimified/scripts/dev-build.sh
#!/bin/bash
set -e

echo "Building cosmic-vimified..."
cargo build

echo "Running tests..."
cargo test

echo "Installing to /usr/local/bin..."
sudo install -Dm755 target/debug/cosmic-vimified /usr/local/bin/cosmic-vimified

echo "Restarting service..."
systemctl --user restart cosmic-vimified

echo "Done! Press Super+G to test."
```

#### Watch Mode for Auto-rebuild
```bash
# Using cargo-watch
cargo watch -x 'build' -s './scripts/install-dev.sh'
```

### Development Environment Recipe

Create `recipes/recipe-dev.yml`:
```yaml
name: fedora-cosmic-atomic-ewt-dev
description: Development image with cosmic-vimified dev tools

base-image: quay.io/fedora-ostree-desktops/cosmic-atomic
image-version: 43

modules:
  # Import base config
  - from-file: recipe.yml

  # Add development tools
  - type: dnf
    install:
      packages:
        - rust-analyzer
        - rust-src
        - clippy
        - rustfmt
        - just
        - mold
        - cargo-watch
        - gtk4-devel
        - wayland-devel
        - at-spi2-core-devel

  # Install cosmic-vimified in development mode
  - type: script
    scripts:
      - setup-cosmic-vimified-dev.sh
```

### Testing Strategy

#### Unit Tests
```rust
// In src/hint_generator.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_generation_sequence() {
        let gen = HintGenerator::new("asdf");
        assert_eq!(gen.nth_label(0), "a");
        assert_eq!(gen.nth_label(1), "s");
        assert_eq!(gen.nth_label(8), "aa");
    }
}
```

#### Integration Tests
```rust
// In tests/test_element_detection.rs
#[test]
fn test_detect_buttons_in_cosmic_app() {
    let detector = ElementDetector::new();
    let elements = detector.detect_in_window("org.cosmic.Files");
    assert!(elements.iter().any(|e| e.role == "button"));
}
```

#### Manual Testing Checklist
- [ ] Activation shortcut works globally
- [ ] Hints appear on all COSMIC apps
- [ ] Click actions work correctly
- [ ] Right-click with Shift works
- [ ] Vim scrolling works
- [ ] Configuration changes apply without restart
- [ ] Multi-monitor support works
- [ ] Performance acceptable on 100+ elements

### CI/CD (Future)

#### GitHub Actions Workflow
```yaml
# .github/workflows/cosmic-vimified-test.yml
name: Test COSMIC Vimified

on:
  pull_request:
    paths:
      - 'cosmic-vimified/**'

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rust-lang/setup-rust-toolchain@v1
      - name: Run tests
        working-directory: cosmic-vimified
        run: cargo test
      - name: Run clippy
        run: cargo clippy -- -D warnings
```

## 10. Performance & Constraints

### Performance Targets

#### Latency Requirements
- **Hint activation to display:** < 100ms (target: 50ms)
- **Hint selection to action:** < 50ms (target: 20ms)
- **Element detection refresh:** < 200ms (target: 100ms)

#### Resource Constraints
- **Memory:** < 50MB idle, < 200MB with hints active
- **CPU:** < 5% idle, < 30% during element detection
- **Startup time:** < 500ms to become ready

### Multi-Monitor Support

#### Requirements
- Detect all connected displays
- Show hints on currently focused monitor only
- Handle monitor hotplug events
- Respect different DPI settings per monitor

#### Implementation
- Query COSMIC compositor for display configuration
- Calculate element positions relative to monitor
- Filter elements by monitor bounds

### Daemon Architecture

#### Always-On Requirements
- Run as systemd user service
- Auto-start on COSMIC session start
- Minimal resource usage when idle
- Fast wake-up on activation

#### Service Configuration
```ini
# /usr/lib/systemd/user/cosmic-vimified.service
[Unit]
Description=COSMIC Vimified Keyboard Hints
PartOf=graphical-session.target

[Service]
Type=simple
ExecStart=/usr/bin/cosmic-vimified
Restart=on-failure

[Install]
WantedBy=cosmic-session.target
```

### Caching Strategy

#### What to Cache
1. **Element positions:** Cache for 100ms
   - Invalidate on window focus change
   - Invalidate on window geometry change
   - Invalidate on Alt+Tab

2. **AT-SPI tree:** Cache application tree structure
   - Refresh when application launches
   - Partial refresh on window changes

3. **Configuration:** Cache in memory
   - Reload on file change (inotify)

#### Cache Invalidation Triggers
- Window focus change (Alt+Tab)
- Window geometry change (resize, move)
- Application launch/close
- Screen rotation/resolution change
- Monitor connect/disconnect

### Accessibility Considerations

#### Screen Reader Integration
- Announce hint mode activation: "Hints active"
- Announce hint mode deactivation: "Hints dismissed"
- Optionally announce number of hints: "23 hints available"
- Do not interfere with screen reader's own navigation

#### Compatibility
- Should not block AT-SPI for other assistive technologies
- Respect system accessibility settings
- High contrast mode support
- Large font support

#### Configuration
```ron
(
    accessibility: (
        announce_activation: true,
        announce_count: false,
        high_contrast_mode: false,  // Auto-detect from system
        minimum_hint_size: 18,      // Larger minimum for accessibility
    ),
)
```

---

## Open Questions & Research Needed

### Technical Investigations Required

1. **Global Keyboard Shortcut Registration**
   - How does COSMIC handle global shortcuts?
   - Can we register single-key shortcuts (e.g., `f`) without conflicts?
   - What's the API for this in cosmic-comp?

2. **Layer Shell Integration**
   - Can we use wlr-layer-shell or COSMIC-specific protocol?
   - What layer should hints appear on (overlay)?
   - How to ensure hints appear above all windows?

3. **AT-SPI Wayland Backend**
   - Does COSMIC's AT-SPI implementation support all needed queries?
   - Are non-COSMIC apps accessible via AT-SPI on COSMIC?
   - Performance characteristics of AT-SPI queries?

4. **Click Simulation**
   - How to programmatically click elements via AT-SPI?
   - Alternative: Synthesize mouse events via Wayland?
   - Which method is more reliable?

5. **COSMIC Applet vs Standalone Service**
   - Should this be a panel applet, or standalone background service?
   - What are the pros/cons of each approach?
   - Can applets register global shortcuts?

### Prototyping Priorities

1. Global shortcut registration (Week 1)
2. Layer shell overlay rendering (Week 1-2)
3. AT-SPI element detection (Week 2)
4. Basic hint generation and display (Week 2-3)
5. Click action execution (Week 3)
6. Configuration system (Week 4)

---

## Version History

- **0.1.0-draft** (2025-11-07): Initial specification based on requirements gathering

---

## References

- [Vimium Browser Extension](https://github.com/philc/vimium)
- [Hints for Linux Desktop](https://github.com/AlfredoSequeida/hints)
- [COSMIC Toolkit Documentation](https://pop-os.github.io/libcosmic-book/)
- [AT-SPI Specification](https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/)
- [Wayland Layer Shell Protocol](https://wayland.app/protocols/wlr-layer-shell-unstable-v1)
- [COSMIC AT-SPI Protocol](https://wayland.app/protocols/cosmic-atspi-unstable-v1)
