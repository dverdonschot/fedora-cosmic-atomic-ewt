# COSMIC Vimified - Technical Research

**Date:** 2025-11-08
**Phase:** 0 - Foundation
**Status:** Complete ✅

This document contains research findings and technical decisions for implementing Vimium-style keyboard navigation for the COSMIC desktop environment.

---

## Table of Contents

1. [Project Goal](#project-goal)
2. [Technical Stack](#technical-stack)
3. [Core Components Research](#core-components-research)
4. [Implementation Challenges](#implementation-challenges)
5. [Reference Implementations](#reference-implementations)
6. [Dependencies Analysis](#dependencies-analysis)
7. [Architecture Decisions](#architecture-decisions)

---

## Project Goal

Create a system-wide keyboard navigation tool for COSMIC desktop that:

- Activates with a global hotkey (Super+G)
- Displays hint labels over all clickable elements
- Allows clicking elements by typing their hint label
- Works across all applications (COSMIC, GTK, Qt, etc.)
- Provides vim-style scrolling
- Supports multi-monitor setups

**Inspiration:** Vimium browser extension, but for the entire desktop.

---

## Technical Stack

### Framework: libcosmic

**Why libcosmic?**
- Official COSMIC desktop UI framework
- Built on iced (Elm-architecture UI framework)
- Integrates natively with COSMIC
- Provides cosmic-config for settings
- Active development by System76

**Key APIs Used:**
- `cosmic::Application` - Main application trait
- `cosmic::widget` - UI widgets
- `cosmic::theme` - Theming support
- `cosmic::app::Command` - Async commands
- `cosmic_config::Config` - Configuration management

**Resources:**
- GitHub: https://github.com/pop-os/libcosmic
- Examples: https://github.com/pop-os/libcosmic/tree/master/examples
- Docs: https://pop-os.github.io/libcosmic/cosmic/

### Element Detection: AT-SPI

**What is AT-SPI?**
- Assistive Technology Service Provider Interface
- D-Bus protocol for accessibility
- Exposes UI element tree from applications
- Provides element properties (role, position, state, etc.)

**Why AT-SPI?**
- Industry standard for Linux accessibility
- Supported by GTK, Qt, COSMIC applications
- Provides element coordinates and metadata
- No need for image recognition or heuristics
- Well-maintained with Rust bindings

**AT-SPI Capabilities:**
- Query accessible tree recursively
- Filter by role (Button, Link, MenuItem, etc.)
- Get element screen coordinates
- Check element state (visible, enabled, focused)
- Receive accessibility events

**Rust Crate:** `atspi` (v0.19+)

**Key APIs:**
```rust
// Connect to accessibility bus
let atspi = atspi::AccessibilityConnection::new().await?;

// Get desktop accessible
let desktop = atspi.desktop(0)?;

// Traverse tree
for child in desktop.get_children()? {
    let role = child.get_role()?;
    let bounds = child.get_extents(CoordType::Screen)?;
    // ...
}
```

**Supported Roles:**
- `Button` - Buttons, toolbars
- `Link` - Hyperlinks
- `MenuItem` - Menu items
- `Icon` - Desktop icons
- `PushButton` - Push buttons
- `CheckBox` - Checkboxes
- `RadioButton` - Radio buttons
- `Toggle` - Toggle switches

**Limitations:**
- Not all apps expose accessibility tree
- Some custom widgets may not be detectable
- Coordinate accuracy depends on app implementation
- Performance impact when traversing large trees

**Resources:**
- Spec: https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/
- Rust crate: https://docs.rs/atspi/
- GNOME docs: https://gitlab.gnome.org/GNOME/at-spi2-core

### Overlay Rendering: Wayland Layer Shell

**What is Layer Shell?**
- Wayland protocol extension by wlroots
- Allows creating windows on specific layers
- Used for panels, overlays, backgrounds
- Supported by most Wayland compositors

**Why Layer Shell?**
- Creates full-screen transparent overlay
- Stays on top of all windows
- Doesn't interfere with window stacking
- Compositor-native (no hacks)
- libcosmic has built-in support

**Layer Shell Layers (bottom to top):**
1. Background - Wallpapers
2. Bottom - Desktop widgets
3. Top - Panels, docks
4. Overlay - Full-screen overlays, lock screens

**Our Usage:**
- Create window on `Overlay` layer
- Make it transparent
- Full-screen across all monitors
- Render hints as widgets
- Capture keyboard input

**libcosmic Integration:**
```rust
// Layer shell window configuration
let window = cosmic::app::window::Window::new()
    .layer(Layer::Overlay)
    .keyboard_interactivity(KeyboardInteractivity::Exclusive)
    .transparent(true)
    .size(screen_width, screen_height);
```

**Resources:**
- Protocol: https://wayland.app/protocols/wlr-layer-shell-unstable-v1
- Smithay implementation: https://github.com/Smithay/smithay
- libcosmic examples: https://github.com/pop-os/libcosmic/tree/master/examples

### Input Synthesis: uinput

**What is uinput?**
- Linux kernel module for creating virtual input devices
- Allows userspace programs to inject input events
- Used by game controllers, automation tools, etc.

**Why uinput?**
- Industry standard for input injection on Linux
- Low-level, reliable click synthesis
- Supported universally
- Works in Wayland and X11

**uinput Capabilities:**
- Create virtual mouse
- Inject mouse movement
- Inject mouse clicks (left, right, middle)
- Inject keyboard events
- Inject scroll events

**Rust Crate:** `uinput` (v0.1+)

**Key APIs:**
```rust
// Create virtual mouse device
let mut device = uinput::Device::new()?
    .name("cosmic-vimified")?
    .event(uinput::event::relative::Position::X)?
    .event(uinput::event::relative::Position::Y)?
    .event(uinput::event::controller::Mouse::Left)?
    .event(uinput::event::controller::Mouse::Right)?
    .create()?;

// Click at coordinates
device.click(&Mouse::Left)?;
```

**Permissions:**
- Requires access to `/dev/uinput`
- Need udev rule for non-root access
- User must be in `input` group

**udev Rule:**
```
# /etc/udev/rules.d/99-cosmic-vimified.rules
KERNEL=="uinput", GROUP="input", MODE="0660"
```

**Resources:**
- Kernel docs: https://www.kernel.org/doc/html/latest/input/uinput.html
- Rust crate: https://docs.rs/uinput/

---

## Core Components Research

### 1. Application Lifecycle

**libcosmic App Pattern:**

```rust
struct CosmicVimified {
    core: cosmic::app::Core,
    state: AppState,
}

enum Message {
    Activate,
    Deactivate,
    KeyPress(Key),
    ElementClicked(usize),
}

impl cosmic::Application for CosmicVimified {
    fn init(core: Core, flags: Flags) -> (Self, Command<Message>) {
        // Initialize application
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        // Handle state updates
    }

    fn view(&self) -> Element<Message> {
        // Render UI
    }
}
```

**State Machine:**
- `Inactive` - Waiting for activation
- `Detecting` - Scanning for elements
- `ShowingHints` - Displaying hints, capturing input
- `Executing` - Performing click action

### 2. Element Detection Pipeline

**Steps:**

1. **Connect to AT-SPI**
   - Establish D-Bus connection
   - Get accessibility bus

2. **Traverse Accessible Tree**
   - Start from desktop root
   - Recursively visit all children
   - Build element list

3. **Filter Elements**
   - Check role (Button, Link, etc.)
   - Verify visibility (STATE_VISIBLE)
   - Verify enabled (STATE_ENABLED)
   - Check on-screen bounds

4. **Extract Metadata**
   - Screen coordinates (x, y, width, height)
   - Element name/label
   - Application name
   - Accessibility path (for clicking)

**Data Structure:**
```rust
struct DetectedElement {
    id: usize,
    role: Role,
    name: String,
    app_name: String,
    bounds: Rectangle,
    atspi_path: String,
}
```

**Performance Considerations:**
- Large trees (100+ elements) can be slow
- Cache results to avoid re-scanning
- Run detection in background thread
- Limit depth of tree traversal

### 3. Hint Generation Algorithm

**Goal:** Generate shortest unique labels using home-row keys

**Home Row Characters:** `a s d f j k l ;`

**Algorithm:**
```
hints = []
chars = ['a', 's', 'd', 'f', 'j', 'k', 'l', ';']

// Generate single-char hints (8 hints)
for c in chars:
    hints.append(c)

// Generate two-char hints (64 hints)
for c1 in chars:
    for c2 in chars:
        hints.append(c1 + c2)

// Generate three-char hints if needed (512 hints)
for c1 in chars:
    for c2 in chars:
        for c3 in chars:
            hints.append(c1 + c2 + c3)

return hints[0..num_elements]
```

**Optimization:**
- Pre-generate hint table for common sizes
- Use shortest hints first
- Prioritize home-row characters
- Allow custom character sets

**Alternative Layouts:**
- Left-hand: `asdwerzxc`
- Right-hand: `jkluiopnm`
- Dvorak home row
- Custom user-defined

### 4. Overlay Rendering

**Requirements:**
- Full-screen transparent window
- Render hints at element coordinates
- Smooth animations
- Low latency input response

**libcosmic Rendering:**
```rust
fn view(&self) -> Element<Message> {
    let hints: Vec<Element<Message>> = self.hints
        .iter()
        .map(|hint| render_hint(hint))
        .collect();

    container(column(hints))
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn render_hint(hint: &Hint) -> Element<Message> {
    text(&hint.label)
        .size(16)
        .style(hint_style())
        .into()
}
```

**Positioning:**
- Use absolute positioning
- Transform from element coords to overlay coords
- Account for monitor scaling
- Handle multi-monitor setups

**Styling:**
- Background color from theme
- High contrast text
- Border for visibility
- Shadow for depth
- Slight transparency

### 5. Click Execution

**Steps:**

1. **User types hint label**
   - Filter matching hints
   - Highlight partial matches
   - Execute on complete match

2. **Get element center**
   - Calculate center point from bounds
   - Transform to screen coordinates

3. **Synthesize click**
   - Move cursor to position (optional)
   - Send mouse down event
   - Send mouse up event
   - Return cursor (optional)

**Click Types:**
- **Left-click:** Default action
- **Right-click:** Hold Shift while typing hint
- **Middle-click:** Future feature

**Code:**
```rust
async fn execute_click(element: &DetectedElement, click_type: ClickType) -> Result<()> {
    let (x, y) = element.bounds.center();

    // Create uinput device
    let mut device = create_mouse_device()?;

    // Move cursor (if configured)
    if config.move_cursor {
        device.move_to(x, y)?;
    }

    // Click
    match click_type {
        ClickType::Left => device.click(&Mouse::Left)?,
        ClickType::Right => device.click(&Mouse::Right)?,
    }

    Ok(())
}
```

---

## Implementation Challenges

### Challenge 1: Element Detection Accuracy

**Problem:** Not all applications expose accessibility tree properly

**Solutions:**
- Focus on well-behaved apps first (COSMIC, GTK, Firefox)
- Document incompatible applications
- Fall back to image recognition (future)
- Allow manual calibration

### Challenge 2: Coordinate Transformation

**Problem:** Screen coordinates vary with monitor scaling, multi-monitor

**Solutions:**
- Query monitor configuration from compositor
- Apply scaling factors correctly
- Test thoroughly on different setups
- Use Wayland output protocols

### Challenge 3: Performance

**Problem:** Traversing large accessibility trees is slow

**Solutions:**
- Run detection in background task
- Cache results
- Limit tree depth
- Filter early in traversal
- Use async/await for non-blocking

### Challenge 4: Permissions

**Problem:** uinput requires special permissions

**Solutions:**
- Provide udev rules
- Document setup clearly
- Graceful error messages
- Consider alternative input methods (future)

### Challenge 5: libcosmic Learning Curve

**Problem:** libcosmic is new and less documented than GTK/Qt

**Solutions:**
- Study official examples extensively
- Reference cosmic-files, cosmic-term, cosmic-settings
- Join COSMIC community (Discord, Matrix)
- Iterate on simple examples first

---

## Reference Implementations

### 1. Vimium (Browser Extension)

**URL:** https://github.com/philc/vimium

**What we learned:**
- Hint label generation algorithm
- Home-row key priority
- Two-character hint combinations
- Keyboard navigation patterns
- User experience patterns

**Differences:**
- Vimium uses DOM queries (we use AT-SPI)
- Vimium is browser-specific (we're desktop-wide)
- Vimium uses CSS overlays (we use Wayland layer-shell)

### 2. hints (Python/X11)

**URL:** https://github.com/AlfredoSequeida/hints

**What we learned:**
- AT-SPI usage patterns
- Element filtering strategies
- Click synthesis with X11 (we'll use uinput)
- Project structure

**Differences:**
- hints uses X11 (we use Wayland)
- hints uses Python (we use Rust)
- hints uses Tkinter for overlay (we use libcosmic)

### 3. warpd

**URL:** https://github.com/rvaiya/warpd

**What we learned:**
- Alternative keyboard-driven pointer control
- Grid-based navigation
- Modal interface patterns

**Differences:**
- warpd uses grid navigation (we use hints)
- Different interaction model
- More general-purpose cursor control

### 4. libcosmic Examples

**URL:** https://github.com/pop-os/libcosmic/tree/master/examples

**Examples studied:**
- `cosmic-app-template` - Basic application structure
- `cosmic-settings` - Configuration patterns
- `cosmic-files` - Complex application architecture
- Layer shell examples (if available)

---

## Dependencies Analysis

### Runtime Dependencies

| Crate | Version | Purpose | Size Impact |
|-------|---------|---------|-------------|
| libcosmic | git | UI framework | Large |
| cosmic-config | git | Configuration | Medium |
| atspi | 0.19 | Element detection | Medium |
| uinput | 0.1 | Click synthesis | Small |
| tokio | 1.35 | Async runtime | Medium |
| anyhow | 1.0 | Error handling | Small |
| thiserror | 1.0 | Error types | Small |
| serde | 1.0 | Serialization | Small |
| ron | 0.8 | Config format | Small |
| tracing | 0.1 | Logging | Small |
| tracing-subscriber | 0.3 | Log formatting | Small |

**Total estimated binary size:** ~10-15 MB (release build, stripped)

### System Dependencies

- **AT-SPI daemon:** `at-spi2-core` package
- **uinput module:** Kernel module (usually built-in)
- **Wayland compositor:** COSMIC compositor
- **D-Bus:** Session bus for AT-SPI

### Build Dependencies

- Rust toolchain (stable)
- cargo
- Standard build tools (gcc, pkg-config, etc.)

---

## Architecture Decisions

### Decision 1: Rust + libcosmic vs. Python + GTK

**Options:**
- A) Rust with libcosmic
- B) Python with GTK
- C) C with GTK

**Chosen:** A (Rust + libcosmic)

**Rationale:**
- Native COSMIC integration
- Better performance
- Memory safety
- Matches COSMIC desktop ecosystem
- Active upstream development
- Modern tooling (cargo, clippy, etc.)

**Trade-offs:**
- Steeper learning curve
- Less documentation than GTK
- Longer compile times

### Decision 2: AT-SPI vs. Image Recognition

**Options:**
- A) AT-SPI accessibility tree
- B) Image recognition (OCR, object detection)
- C) Hybrid approach

**Chosen:** A (AT-SPI)

**Rationale:**
- Standard Linux accessibility protocol
- Accurate element coordinates
- No ML/AI complexity
- Lower resource usage
- Supported by major toolkits
- Well-defined API

**Trade-offs:**
- Requires app accessibility support
- Not all apps expose elements correctly
- May miss custom widgets

**Future:** Consider hybrid approach for unsupported apps

### Decision 3: uinput vs. libei/XDG RemoteDesktop

**Options:**
- A) uinput (kernel module)
- B) libei (Wayland input emulation)
- C) XDG RemoteDesktop portal

**Chosen:** A (uinput)

**Rationale:**
- Mature, stable API
- Works universally (Wayland, X11)
- Simple Rust bindings
- No sandboxing issues
- Well-documented

**Trade-offs:**
- Requires permissions setup
- Needs udev rules
- Lower-level API

**Future:** Consider libei when more mature

### Decision 4: Layer Shell vs. Regular Window

**Options:**
- A) Layer shell overlay window
- B) Regular full-screen window
- C) Per-monitor windows

**Chosen:** A (Layer shell overlay)

**Rationale:**
- Designed for overlays
- Stays on top reliably
- Compositor integration
- Multi-monitor support
- Transparent background support

**Trade-offs:**
- Wayland-only (not X11)
- Compositor must support protocol
- More complex setup

### Decision 5: Home-Row Keys vs. Full Alphabet

**Options:**
- A) Home-row only (asdfjkl;)
- B) Full alphabet (a-z)
- C) Optimized based on frequency

**Chosen:** A (Home-row only, configurable)

**Rationale:**
- Faster typing (fingers on home row)
- Matches Vimium UX
- Muscle memory from vim
- Fewer keystrokes needed
- Less visual clutter

**Trade-offs:**
- Fewer single-character hints (8 vs 26)
- More two-character hints needed

**Future:** Allow configuration of character set

---

## Next Steps (Phase 1)

1. **Study libcosmic examples**
   - Clone repo
   - Build and run examples
   - Understand App trait
   - Learn layer shell usage

2. **Create basic application**
   - Implement App trait
   - Create window
   - Handle lifecycle

3. **Add layer shell overlay**
   - Full-screen transparent window
   - Overlay layer
   - Keyboard input capture

4. **Test ESC to close**
   - Capture keyboard events
   - Handle ESC key
   - Clean shutdown

**Success criteria for Phase 1:**
- Application launches
- Shows transparent overlay
- Responds to ESC
- No crashes

---

## Resources

### Documentation

- libcosmic: https://pop-os.github.io/libcosmic/
- AT-SPI: https://www.freedesktop.org/wiki/Accessibility/AT-SPI2/
- Layer Shell: https://wayland.app/protocols/wlr-layer-shell-unstable-v1
- uinput: https://www.kernel.org/doc/html/latest/input/uinput.html

### Code References

- COSMIC Apps: https://github.com/pop-os/cosmic-epoch
- Vimium: https://github.com/philc/vimium
- hints: https://github.com/AlfredoSequeida/hints
- libcosmic: https://github.com/pop-os/libcosmic

### Community

- COSMIC Discord: https://discord.gg/cosmic
- Pop!_OS Chat: https://chat.pop-os.org/
- System76 GitHub: https://github.com/pop-os

---

**Research complete ✅**
**Ready to proceed to Phase 1: Application Scaffold**
