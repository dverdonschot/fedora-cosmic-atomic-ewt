# COSMIC Desktop Global Keyboard Shortcuts Research

**Date:** 2025-11-08
**Purpose:** Understanding COSMIC's keyboard shortcut system for implementing hint-based navigation

## Overview

COSMIC desktop uses a configuration-based keyboard shortcut system built into the `cosmic-comp` Wayland compositor. Shortcuts are defined using RON (Rusty Object Notation) format and managed through the cosmic-config system.

## How COSMIC Handles Global Shortcuts

### Architecture
- **Compositor-level**: Handled directly by `cosmic-comp` (Wayland compositor)
- **Configuration system**: Uses `cosmic-config` library
- **Protocol extensions**: `cosmic-protocols` for Wayland extensions
- **Runtime configuration**: Added in July 2024 update

### Configuration Files
- **Default keybindings**: `cosmic-comp/data/keybindings.ron`
- **User config path**: `$(sharedir)/cosmic/com.system76.CosmicSettings.Shortcuts/v1/defaults`
- **User interface**: COSMIC Settings > Input Devices > Keyboard > Keyboard shortcuts

### Keybinding Format

RON format structure:
```ron
(modifiers: [Modifier1, Modifier2], key: "KeyName"): Action
```

**Examples:**
```ron
// Single modifier
(modifiers: [Super], key: "q"): Close

// Multiple modifiers
(modifiers: [Super, Shift], key: "Escape"): System(LogOut)
(modifiers: [Super, Shift, Ctrl], key: "Left"): MoveToPreviousWorkspace

// No modifier (function keys only)
(modifiers: [], key: "Print"): System(Screenshot)

// Modifier key alone
(modifiers: [Super]): System(Launcher)
```

**Available Modifiers:**
- `Super` (Windows/Command key)
- `Alt`
- `Shift`
- `Ctrl`

**Action Types:**
- Window management: `Close`, `Focus(Left)`, `Focus(Right)`
- Workspace navigation: `MoveToPreviousWorkspace`, `MoveToNextWorkspace`
- System commands: `System(Terminal)`, `System(Launcher)`, `System(LogOut)`
- Media controls: `System(VolumeRaise)`, `System(VolumeLower)`

## Single-Key Shortcuts

### Technical Capability
✅ **Format supports it**: `(modifiers: [], key: "SomeKey"): Action`

### Practical Limitations
❌ **Unusable for letter keys**:
- Would intercept the key globally across ALL applications
- Prevents typing that letter in text editors, browsers, terminals, etc.
- Completely breaks normal text input

✅ **Works for special keys**:
- Function keys (F1-F12)
- Media keys
- Print Screen, Pause, etc.
- These don't interfere with typing

### Modal Behavior
❌ **Not supported natively**: COSMIC's shortcut system has no concept of "modes" or state-based keybindings. Shortcuts are either globally active or inactive.

## API for Registering Shortcuts

### For Third-Party Applications
❌ **No public programmatic API exists**

Global shortcuts can only be registered via:
1. Editing configuration files
2. Modifying cosmic-comp source code

### For COSMIC Applications
If building a COSMIC app using `libcosmic`:

**Application-level shortcuts** (not global):
```rust
use cosmic::widget::menu::key_bind::KeyBind;
use cosmic::keyboard_nav;
```

- `KeyBind` struct: Represents key combinations
- `cosmic::keyboard_nav`: Subscribe to common app shortcuts
- Scope: Application only, not system-wide

### Related Protocols
- **Shortcuts inhibit protocol**: Allows apps to request suppression of system shortcuts temporarily
- **XDG Desktop Portal GlobalShortcuts API**: Exists in specification, COSMIC implementation status unclear

## Repositories

- **cosmic-comp**: https://github.com/pop-os/cosmic-comp
- **libcosmic**: https://github.com/pop-os/libcosmic
- **cosmic-protocols**: https://github.com/pop-os/cosmic-protocols
- **cosmic-settings-daemon**: https://github.com/pop-os/cosmic-settings-daemon

## Feasible Approach: Hint-Based Navigation

Instead of modal vim-style shortcuts, use a **hint overlay system**:

### Design
1. **Trigger**: `Super+G` launches hint overlay application
2. **Display**: Overlay shows hints on clickable UI elements
3. **Input**: User types hint letters while overlay is active
4. **Action**: Application synthesizes click and closes
5. **Scope**: Only one global shortcut needed (Super+G)

### Advantages
✅ No single-key global shortcuts needed
✅ Input only captured when overlay is active
✅ Doesn't interfere with normal typing
✅ Works with existing COSMIC architecture
✅ Similar to browser extensions like Vimium

### Implementation Requirements
1. Register `Super+G` in cosmic-comp configuration
2. Build overlay application that:
   - Captures screen/window information
   - Identifies clickable elements
   - Renders hint labels
   - Captures keyboard input while active
   - Synthesizes mouse clicks via Wayland protocol
3. Package in BlueBuild recipe

### Technical Challenges
- **Element detection**: Need to identify clickable UI elements across different toolkits (GTK, Qt, libcosmic, etc.)
- **Wayland protocols**: May need accessibility protocols or window introspection
- **Click synthesis**: Requires appropriate Wayland input protocols
- **Overlay rendering**: Need to draw over all windows

## Existing Tools for Hint-Based Navigation

### Hints - Vimium for Linux Desktop
**Repository**: https://github.com/AlfredoSequeida/hints

A Python-based tool that brings browser-style Vimium navigation to the entire Linux desktop.

**Features:**
- Display keyboard hints on clickable GUI elements
- Actions: click, right-click, hover, drag
- Wayland support (with compositor-specific implementations)

**Technical Architecture:**
```
┌─────────────────────────────────────────┐
│  User presses trigger key (configurable)│
└──────────────┬──────────────────────────┘
               ▼
┌─────────────────────────────────────────┐
│  Hints daemon (hintsd) starts           │
│  - Uses AT-SPI for element detection    │
│  - GTK4 for overlay rendering           │
│  - Layer-shell for window positioning   │
└──────────────┬──────────────────────────┘
               ▼
┌─────────────────────────────────────────┐
│  Display overlay with hint labels       │
│  User types hint letters                │
└──────────────┬──────────────────────────┘
               ▼
┌─────────────────────────────────────────┐
│  Synthesize mouse action via uinput     │
│  Close overlay                          │
└─────────────────────────────────────────┘
```

**Dependencies:**
- **Element detection**: AT-SPI (Linux Accessibility framework)
- **Overlay positioning**: wlr-layer-shell protocol
- **Input synthesis**: uinput kernel module
- **Rendering**: GTK4 + gtk4-layer-shell

**COSMIC Compatibility**: ✅ **SHOULD WORK**

Reasoning:
1. ✅ COSMIC-comp supports wlr-layer-shell (via smithay)
2. ✅ AT-SPI works across Wayland compositors
3. ✅ uinput is compositor-agnostic (kernel-level)
4. ❌ NOT tested on COSMIC (as of Nov 2025)

**Known Limitations:**
- Does NOT work on GNOME+Wayland (mutter doesn't support layer-shell)
- Drag operations may not work on all compositors
- Requires proper udev rules for uinput access

### Alternative Tools

#### warpd
**Repository**: https://github.com/rvaiya/warpd

Modal keyboard-driven virtual pointer with multiple modes (normal, hint, grid).

**Wayland Status:**
- ⚠️ Limited Wayland support
- Cannot bind global hotkeys (must be bound in compositor config)
- See LIMITATIONS section in man page

#### keynav
**Repository**: https://github.com/jordansissel/keynav

Classic keyboard-driven mouse navigation tool.

**Wayland Status:**
- ❌ X11 only, no Wayland support
- keynav-wayland fork exists but author migrated to warpd

## Implementation Strategies for COSMIC

### Option 1: Use Existing Hints Tool ⭐ **RECOMMENDED**

**Pros:**
- ✅ Already implemented and working
- ✅ Supports Wayland via layer-shell
- ✅ Should work on COSMIC (smithay supports layer-shell)
- ✅ Uses standard accessibility APIs
- ✅ Easy to package in BlueBuild recipe

**Cons:**
- ⚠️ Untested on COSMIC (needs verification)
- ⚠️ Python dependency
- ⚠️ Requires uinput permissions

**Implementation Steps:**
1. Add Hints to BlueBuild recipe via dnf or Flatpak
2. Configure udev rules for uinput access
3. Add custom keybinding in COSMIC config:
   ```ron
   (modifiers: [Super], key: "g"): Spawn("hints")
   ```
4. Test and verify functionality

### Option 2: Build Custom COSMIC-Native Solution

**Architecture:**
```rust
// cosmic-hints (hypothetical)
use libcosmic;
use smithay_client_toolkit;
use atspi; // AT-SPI bindings

// 1. Register global shortcut via cosmic-comp config
// 2. Launch as libcosmic application
// 3. Use AT-SPI to find clickable elements
// 4. Render hints using libcosmic widgets
// 5. Use uinput or virtual-pointer protocol for clicks
```

**Pros:**
- ✅ Native COSMIC integration
- ✅ Rust performance and safety
- ✅ Can leverage libcosmic for UI

**Cons:**
- ❌ Significant development effort
- ❌ Need to implement element detection
- ❌ Need to handle multiple UI toolkits
- ❌ Maintenance burden

### Option 3: Configure warpd for COSMIC

**Steps:**
1. Install warpd
2. Configure cosmic-comp to launch warpd with oneshot flags
3. Bind `Super+G` to warpd hint mode

**Pros:**
- ✅ Lightweight C implementation
- ✅ Multiple navigation modes

**Cons:**
- ⚠️ Wayland support limited
- ⚠️ No automatic element detection (uses grid/hints manually)
- ⚠️ Less intuitive than Hints

## Protocol Requirements Summary

| Protocol/API | Purpose | COSMIC Support | Status |
|--------------|---------|----------------|---------|
| wlr-layer-shell | Overlay positioning | ✅ Yes (via smithay) | Working |
| AT-SPI | Element detection | ✅ Yes (standard Linux) | Working |
| uinput | Input synthesis | ✅ Yes (kernel module) | Working |
| wlr-virtual-pointer | Input synthesis (alt) | ❓ Unknown | Need to verify |
| GTK4 | UI rendering | ✅ Yes | Working |

## Next Steps: Testing Hints on COSMIC

### 1. Install Hints
```bash
# Via pipx (recommended)
pipx install hints

# Or via dnf (if packaged)
dnf install hints
```

### 2. Configure udev Rules
```bash
# Check if user is in input group
groups $USER

# Add udev rule for uinput
sudo tee /etc/udev/rules.d/99-uinput.rules << EOF
KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"
EOF

# Reload rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

### 3. Test Hints
```bash
# Start daemon
hintsd &

# Trigger hints (default: Alt+Shift+Space)
hints
```

### 4. Add to COSMIC Config
If successful, add to `recipes/recipe.yml` and configure keybinding.

## Conclusion

**Full vim-mode**: Not feasible with COSMIC's current architecture without major compositor modifications.

**Hint-based navigation (Super+G)**: ✅ **HIGHLY FEASIBLE**

**Recommended Approach**:
1. Test existing **Hints** tool on COSMIC
2. If compatible, package in BlueBuild recipe
3. Configure `Super+G` as trigger via COSMIC Settings or config file

This approach:
- Requires minimal development
- Leverages existing, maintained software
- Works within COSMIC's architecture
- Provides Vimium-like experience for desktop navigation

---

## References

- COSMIC Keybindings RON: https://github.com/pop-os/cosmic-comp/blob/master/data/keybindings.ron
- libcosmic Documentation: https://pop-os.github.io/libcosmic/cosmic/
- COSMIC Desktop Updates: https://blog.system76.com/post/cosmic-july-2024/
- Hints Project: https://github.com/AlfredoSequeida/hints
- warpd Project: https://github.com/rvaiya/warpd
- Smithay Layer Shell: https://github.com/Smithay/smithay
- AT-SPI Documentation: https://www.freedesktop.org/wiki/Accessibility/
