# COSMIC Vimified - Technical Research Findings

**Date:** 2025-11-08
**Status:** Complete
**Purpose:** Document research findings on COSMIC protocols, dependencies, and implementation strategies

## Executive Summary

Based on extensive research into COSMIC desktop's architecture and existing hint-based navigation tools, **cosmic-vimified is highly feasible** and all required protocols are supported by COSMIC. This document outlines the technical findings that inform our implementation strategy.

## Research Questions Answered

### 1. How does COSMIC handle global shortcuts?

**Answer:** Configuration-based system via `cosmic-comp` compositor

**Details:**
- Uses RON (Rusty Object Notation) format
- Configuration location: `cosmic-comp/data/keybindings.ron`
- Runtime configurable via COSMIC Settings (added July 2024)
- Managed through `cosmic-config` library

**Format:**
```ron
(modifiers: [Super], key: "g"): Spawn("cosmic-vimified")
```

**Integration Strategy for cosmic-vimified:**
- Add custom keybinding to COSMIC config via BlueBuild recipe
- Configure `Super+G` as trigger (avoids conflict with COSMIC's Super+F file explorer)
- Use `Spawn()` action to launch cosmic-vimified overlay

### 2. Can we use single-key shortcuts without modifiers?

**Answer:** Technically yes, but practically NO for letter keys

**Technical Capability:**
- Format supports `modifiers: []`
- Example: `(modifiers: [], key: "Print"): System(Screenshot)` ✅

**Why NOT for letter keys:**
- Would intercept globally across ALL applications
- Breaks normal text input in browsers, editors, terminals
- Users couldn't type the letter anywhere

**Why it works for function keys:**
- Print Screen, F1-F12, media keys don't interfere with typing
- These are viable for single-key shortcuts

**Decision for cosmic-vimified:**
- Use `Super+G` as primary trigger (avoids Super+F file explorer conflict)
- No vim-style "modal" modes (not supported by COSMIC)
- Hints only active when overlay is visible

### 3. What's the API for programmatic shortcut registration?

**Answer:** NO public API - configuration file based only

**For Third-Party Apps:**
- ❌ No programmatic registration API
- ✅ Configuration file editing via BlueBuild recipe
- ✅ User configuration via COSMIC Settings GUI

**For COSMIC Applications:**
- libcosmic provides `cosmic::widget::menu::key_bind` for app-level shortcuts
- These are application-scoped, not global
- Not suitable for our needs

**Implementation for cosmic-vimified:**
- Add keybinding to cosmic-comp config during image build
- Users can reconfigure via COSMIC Settings if desired

## Rust Crate Dependencies Research

### 1. AT-SPI Element Detection

**Recommended Crate:** `atspi` v0.19+

**Details:**
- Higher-level, asynchronous, pure Rust AT-SPI2 implementation
- Uses `zbus` for D-Bus communication
- Part of Odilia screen reader project
- Requires async executor (tokio or async-std)

**Related Crates:**
- `atspi-common`: Primitive types for AT-SPI events
- `atspi-proxies`: Proxies to query/manipulate UI objects
- `atspi-connection`: Connection management

**Usage for cosmic-vimified:**
```rust
use atspi::AccessibilityConnection;
use atspi::accessible::Role;

// Query accessible tree for clickable elements
// Filter by roles: Button, Link, MenuItem, etc.
```

**Documentation:** https://docs.rs/atspi/latest/atspi/

### 2. Layer Shell Overlay Rendering

**Approach:** libcosmic with smithay layer-shell support

**COSMIC Compatibility:** ✅ **FULLY SUPPORTED**
- cosmic-comp built on smithay compositor library
- smithay supports wlr-layer-shell protocol
- Active development: PR #1726 (Oct 2025) for layer shell updates

**Advantages over GTK4:**
- Native COSMIC integration
- Rust-only stack (no GTK bindings)
- Consistent with COSMIC design language
- Better performance

**Implementation:**
```rust
use libcosmic::widget;
use libcosmic::app::Application;

// Create layer-shell surface
// Render hint labels as overlay widgets
// Position over all other windows
```

### 3. Mouse Click Synthesis

**Recommended Crate:** `mouse-keyboard-input` v0.7+

**Why this crate:**
- Built on top of `rust-uinput` (github.com/meh/rust-uinput)
- Simple, high-level API
- Well-maintained with modern dependencies
- Linux-specific (which is fine for COSMIC)

**Alternative:** Direct `uinput` crate
- Lower-level control
- More complex API
- Use if mouse-keyboard-input limitations found

**Usage:**
```rust
use mouse_keyboard_input::VirtualDevice;

let mut device = VirtualDevice::default()?;
device.click(BTN_LEFT)?; // Left click
device.click(BTN_RIGHT)?; // Right click
```

**Requirements:**
- udev rules for `/dev/uinput` access
- User must be in `input` group
- Will add to BlueBuild recipe

**Documentation:** https://docs.rs/mouse-keyboard-input/

### 4. Configuration Management

**Approach:** cosmic-config (COSMIC standard)

**Benefits:**
- Standard COSMIC configuration system
- RON format (matches COSMIC conventions)
- Hot-reload support
- Integration with COSMIC Settings (future)

**Usage:**
```rust
use cosmic_config::{Config, CosmicConfigEntry};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct VimifiedConfig {
    keybindings: KeyBindings,
    appearance: Appearance,
    behavior: Behavior,
}
```

## Protocol Compatibility Matrix

| Protocol/API | Purpose | COSMIC Support | Status | Notes |
|--------------|---------|----------------|---------|-------|
| wlr-layer-shell | Overlay positioning | ✅ Yes | Working | Via smithay |
| AT-SPI | Element detection | ✅ Yes | Working | Standard Linux |
| uinput | Input synthesis | ✅ Yes | Working | Kernel module |
| cosmic-config | Configuration | ✅ Yes | Working | COSMIC standard |
| wlr-virtual-pointer | Input synthesis (alt) | ❓ Unknown | Untested | May not be needed |

All critical protocols are supported! ✅

## Comparison with "Hints" Tool

The existing [Hints](https://github.com/AlfredoSequeida/hints) project provides valuable reference:

### What Hints Does Well
- ✅ AT-SPI element detection (proven approach)
- ✅ Layer shell overlay rendering (works on wlroots compositors)
- ✅ uinput for click synthesis

### Why Build Native COSMIC Version

1. **Technology Stack:**
   - Hints: Python + GTK4 + gtk4-layer-shell
   - cosmic-vimified: Rust + libcosmic
   - Benefit: Lighter weight, faster, native COSMIC integration

2. **Configuration:**
   - Hints: Python-based config
   - cosmic-vimified: cosmic-config (RON format)
   - Benefit: Consistent with COSMIC ecosystem

3. **Future Integration:**
   - Hints: Standalone tool
   - cosmic-vimified: Can integrate with COSMIC Settings
   - Benefit: First-class desktop feature

4. **Performance:**
   - Hints: Python runtime overhead
   - cosmic-vimified: Compiled Rust (zero-cost abstractions)
   - Benefit: Lower resource usage, faster activation

### What We Can Learn from Hints

- **Architecture pattern:** Daemon + overlay client works well
- **Element detection:** AT-SPI approach is proven
- **Compositor-specific code:** We'll need COSMIC-specific tweaks
- **Uinput permissions:** Need udev rules in our BlueBuild recipe

## Implementation Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────┐
│ User presses Super+G                                │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ COSMIC compositor (cosmic-comp)                     │
│ Spawns: cosmic-vimified                             │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ cosmic-vimified Application                         │
│ ┌─────────────────────────────────────────────────┐ │
│ │ 1. Init libcosmic Application                   │ │
│ │ 2. Load cosmic-config                           │ │
│ │ 3. Create layer-shell surface (overlay)         │ │
│ └─────────────────────────────────────────────────┘ │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ Element Detection (AT-SPI)                          │
│ ┌─────────────────────────────────────────────────┐ │
│ │ 1. Connect to AT-SPI bus                        │ │
│ │ 2. Query accessible tree                        │ │
│ │ 3. Filter clickable elements                    │ │
│ │ 4. Get screen coordinates                       │ │
│ └─────────────────────────────────────────────────┘ │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ Hint Generation                                     │
│ ┌─────────────────────────────────────────────────┐ │
│ │ 1. Generate labels (asdfjkl; sequence)          │ │
│ │ 2. Calculate label positions                    │ │
│ │ 3. Create hint widgets                          │ │
│ └─────────────────────────────────────────────────┘ │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ Overlay Rendering (libcosmic)                       │
│ ┌─────────────────────────────────────────────────┐ │
│ │ 1. Render hint labels                           │ │
│ │ 2. Listen for keyboard input                    │ │
│ │ 3. Highlight matching hints as user types       │ │
│ └─────────────────────────────────────────────────┘ │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ User types hint label (e.g., "as")                 │
└──────────────────┬──────────────────────────────────┘
                   ▼
┌─────────────────────────────────────────────────────┐
│ Action Execution (uinput)                           │
│ ┌─────────────────────────────────────────────────┐ │
│ │ 1. Get element coordinates                      │ │
│ │ 2. Create virtual mouse device                  │ │
│ │ 3. Move cursor to position                      │ │
│ │ 4. Synthesize click event                       │ │
│ │ 5. Close overlay                                │ │
│ └─────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────┘
```

### Module Structure

```
cosmic-vimified/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # libcosmic Application impl
│   ├── config/
│   │   ├── mod.rs           # Config module
│   │   ├── keys.rs          # Keybinding config
│   │   └── appearance.rs    # Visual config
│   ├── detection/
│   │   ├── mod.rs           # Element detection module
│   │   ├── atspi.rs         # AT-SPI integration
│   │   └── filters.rs       # Element filtering logic
│   ├── hints/
│   │   ├── mod.rs           # Hint generation module
│   │   ├── generator.rs     # Label generation algorithm
│   │   └── positioning.rs   # Hint positioning logic
│   ├── overlay/
│   │   ├── mod.rs           # Overlay module
│   │   ├── renderer.rs      # libcosmic rendering
│   │   └── input.rs         # Keyboard input handling
│   ├── actions/
│   │   ├── mod.rs           # Action execution module
│   │   ├── click.rs         # Click synthesis
│   │   └── scroll.rs        # Scroll actions
│   └── utils/
│       ├── mod.rs           # Utility module
│       └── geometry.rs      # Coordinate calculations
```

## Dependencies to Add

Update `Cargo.toml`:

```toml
[dependencies]
# COSMIC libraries (already present)
libcosmic = { git = "https://github.com/pop-os/libcosmic" }
cosmic-config = { git = "https://github.com/pop-os/libcosmic" }

# NEW: Accessibility
atspi = "0.19"
atspi-common = "0.3"
atspi-connection = "0.3"
zbus = "4.0"  # Required by atspi

# NEW: Input synthesis
mouse-keyboard-input = "0.7"
# OR: uinput = "0.1"  # If we need lower-level control

# Serialization (already present)
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"

# Async runtime (already present)
tokio = { version = "1.0", features = ["full"] }

# Logging (already present)
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Error handling (already present)
anyhow = "1.0"
thiserror = "1.0"
```

## System Requirements

### uinput Permissions

Add to BlueBuild recipe (`files/system/etc/udev/rules.d/99-cosmic-vimified.rules`):

```udev
# Allow users in input group to access uinput device
KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"
```

### COSMIC Keybinding

Add to BlueBuild recipe (method TBD - may need custom script):

```ron
// Add to cosmic-comp keybindings
(modifiers: [Super], key: "g"): Spawn("cosmic-vimified")
```

## Next Steps

### Phase 1: Prototype Core Functionality
1. ✅ Research complete
2. [ ] Set up basic libcosmic application skeleton
3. [ ] Implement AT-SPI connection and element query
4. [ ] Create simple hint generation algorithm
5. [ ] Render basic overlay with libcosmic
6. [ ] Implement click synthesis with uinput

### Phase 2: Polish & Configuration
7. [ ] Add cosmic-config integration
8. [ ] Implement keyboard layout modes
9. [ ] Add vim-style scrolling
10. [ ] Refine hint positioning algorithm
11. [ ] Add right-click support

### Phase 3: Integration & Packaging
12. [ ] Create BlueBuild recipe integration
13. [ ] Add udev rules
14. [ ] Configure COSMIC keybinding
15. [ ] Test on COSMIC desktop
16. [ ] Documentation and examples

## Open Questions

1. **Layer shell specifics:** Need to review libcosmic examples for exact API
2. **AT-SPI performance:** How fast can we query the tree? Need benchmarking
3. **Multi-monitor:** How does AT-SPI handle multi-monitor coordinates?
4. **Permission handling:** Best UX for requesting uinput permissions?
5. **COSMIC integration:** Can we add a settings panel to COSMIC Settings?

## References

- COSMIC comp keybindings: https://github.com/pop-os/cosmic-comp/blob/master/data/keybindings.ron
- libcosmic examples: https://github.com/pop-os/libcosmic/tree/master/examples
- atspi crate: https://docs.rs/atspi/latest/atspi/
- mouse-keyboard-input: https://docs.rs/mouse-keyboard-input/
- Hints project (reference): https://github.com/AlfredoSequeida/hints
- Smithay layer shell: https://github.com/Smithay/smithay
- COSMIC protocols: https://github.com/pop-os/cosmic-protocols

---

**Conclusion:** All technical prerequisites are met. cosmic-vimified is feasible and can be built as a native COSMIC application using the Rust ecosystem. Ready to proceed with implementation!
