# COSMIC Vimified - Architecture Documentation

## Overview

COSMIC Vimified is a keyboard-driven navigation system for the COSMIC desktop environment, inspired by Vimium. It allows users to click any visible UI element using keyboard hints, similar to browser extensions like Vimium or Tridactyl.

**Current Status**: Phase 4 (Overlay Rendering) - Hints are displayed but coordinate positioning and click actions are still being refined.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    User Interaction                         │
│  (Super+G or CLI command: cosmic-vimified show)            │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                  D-Bus Daemon Service                       │
│              (com.cosmic.Vimified)                          │
│                                                             │
│  Listens for:                                              │
│  - Show    │  Commands sent via tokio::mpsc channel        │
│  - Hide    │                                               │
│  - Toggle  │                                               │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│            Wayland Layer Shell Overlay                     │
│              (iced_layershell)                             │
│                                                             │
│  Full-screen transparent window that:                      │
│  - Captures keyboard input when visible                    │
│  - Passes through mouse events (events_transparent: true)  │
│  - Renders hints at absolute screen positions              │
└────────────────────┬────────────────────────────────────────┘
                     │
         ┌───────────┴────────────┐
         │                        │
         ▼                        ▼
┌──────────────────┐    ┌─────────────────────┐
│   AT-SPI Tree    │    │   Hint Generation   │
│   Detection      │    │                     │
│                  │    │  Uses base-8 system │
│ Finds clickable  │───▶│  (a,s,d,f,j,k,l,;)  │
│ elements across  │    │                     │
│ all apps         │    │  Generates: a, as,  │
│                  │    │  ad, af, aj, etc.   │
└──────────────────┘    └─────────────────────┘
```

## Core Components

### 1. Main Entry Point (`main.rs`)

**Purpose**: Application bootstrap and command routing

**Flow**:
1. Initialize tracing/logging
2. Parse CLI arguments (using `clap`)
3. Create tokio async runtime
4. Route to appropriate mode:
   - **Daemon mode** (default): Start D-Bus service and overlay
   - **Show/Hide/Toggle**: Send command to running daemon

**Key Functions**:
- `main()` - Entry point with tokio runtime
- `send_daemon_command()` - Sends D-Bus method calls to running daemon

---

### 2. D-Bus Daemon Interface (`daemon.rs`)

**Purpose**: Provides D-Bus interface for external control

**D-Bus Interface**:
- **Service Name**: `com.cosmic.Vimified`
- **Object Path**: `/com/cosmic/Vimified`
- **Interface**: `com.cosmic.Vimified1`

**Methods**:
- `Show()` - Display hint overlay
- `Hide()` - Hide hint overlay
- `Toggle()` - Toggle overlay visibility

**Implementation**:
- `VimifiedDaemon` - zbus interface implementation
- `run_daemon()` - Main daemon loop that spawns overlay on demand
- Uses `tokio::mpsc` channels to communicate with overlay

**Lifecycle**:
```
Start daemon
    │
    ├─▶ Listen for D-Bus commands
    │
    ├─▶ On "Show": Spawn overlay thread
    │   └─▶ Run layershell_app::run_once()
    │
    └─▶ Overlay closes on ESC, daemon stays alive
```

---

### 3. Layer Shell Overlay Application (`layershell_app.rs`)

**Purpose**: Main overlay UI using Wayland layer shell protocol

**Architecture**: Elm-like architecture via `iced_layershell`

#### State (`OverlayApp`)
```rust
pub struct OverlayApp {
    visible: bool,                      // Overlay visibility
    detected_elements: Vec<DetectedElement>,  // AT-SPI results
    hints: Vec<ElementHint>,            // Generated hints
    current_input: String,              // User's typed chars
    appearance: HintAppearance,         // Visual styling
}
```

#### Messages (`Message` enum)
- `IcedEvent` - Keyboard/mouse events from iced
- `CharInput(char)` - Character typed for hint selection
- `ElementsDetected` - AT-SPI detection complete
- `CloseOverlay` - Hide overlay
- `DaemonCommand` - Command from D-Bus daemon

#### Layer Shell Configuration
```rust
LayerShellSettings {
    anchor: Top | Bottom | Left | Right,  // Fill entire screen
    exclusive_zone: 0,                    // Don't reserve space
    keyboard_interactivity: OnDemand,     // Only capture when visible
    events_transparent: true,             // Pass mouse through
}
```

**Key Behavior**:
- When **hidden**: Returns zero-size view to prevent keyboard blocking
- When **visible**: Displays absolute-positioned hints
- **ESC key**: Hides overlay, keeps daemon running
- **Hint match**: Logs click position (Phase 5: will trigger click)

---

### 4. Element Detection (`detection/`)

#### AT-SPI Integration (`atspi.rs`)

**Purpose**: Discover clickable UI elements across all applications

**Process**:
1. Connect to AT-SPI accessibility bus
2. Get root registry of all accessible applications
3. Recursively traverse accessibility tree
4. For each element:
   - Check role (Button, Link, etc.)
   - Check states (Visible, Enabled, Focusable)
   - Get screen coordinates via Component interface
   - Filter using `filters::should_process_element()`

**Key Type**: `AtSpiDetector`
- `new()` - Establish AT-SPI connection
- `detect_all_elements()` - Traverse entire tree
- `traverse_tree()` - Recursive tree walker
- `process_accessible()` - Filter and extract element data

#### Element Filtering (`filters.rs`)

**Purpose**: Determine which elements should show hints

**Filter Criteria**:

1. **Role-based filtering**:
   - **Primary clickable**: Button, Link, Checkbox, ToggleButton, etc.
   - **Secondary clickable**: MenuItem, ListItem (if has actions)
   - **Excluded**: Icon, Label, Text, Panel, StatusBar

2. **State filtering**:
   - Must have: Visible + Showing
   - Must have: Enabled or Sensitive
   - Should have: Focusable, Selectable, or Checkable

3. **Size filtering**:
   - Minimum: 10x10 pixels
   - Maximum: 2000 pixels per dimension
   - Filters out tiny decorations and huge containers

**Main Function**: `should_process_element(role, states, bounds) -> bool`

#### Data Types (`types.rs`)

**`ElementBounds`**:
```rust
pub struct ElementBounds {
    pub x: i32,       // Screen x (pixels from left)
    pub y: i32,       // Screen y (pixels from top)
    pub width: i32,
    pub height: i32,
}
```

**`DetectedElement`**:
```rust
pub struct DetectedElement {
    pub bounds: ElementBounds,
    pub role: String,
    pub name: String,         // Accessible name
    pub app_name: String,     // Containing application
    pub description: String,  // Accessible description
}
```

---

### 5. Hint Generation (`hints/`)

#### Keyboard Layouts (`layout.rs`)

**Purpose**: Define which keys are used for hint labels

**Layouts**:
- **Standard**: `a s d f j k l ;` (home row)
- **LeftHanded**: `a s d f w e r`
- **RightHanded**: `j k l ; i o p`
- **Custom**: User-defined key sequence

#### Hint Generator (`generator.rs`)

**Purpose**: Generate unique labels for each element

**Algorithm**: Base-8 numeral system using home row keys
- 1-8 elements: `a`, `s`, `d`, `f`, `j`, `k`, `l`, `;`
- 9-64 elements: `aa`, `as`, `ad`, ..., `;;`
- 65+ elements: `aaa`, `aas`, `aad`, ...

**Key Type**: `HintGenerator`
- `generate(count) -> Vec<HintLabel>` - Create labels
- `index_to_hint(index) -> String` - Convert index to label

**Matching**:
- As user types, hints are filtered by prefix match
- Exact match triggers selection (Phase 5: will click)
- Visual state changes:
  - **Default**: Blue badge (COSMIC theme)
  - **Partial match**: Green badge
  - **No match**: Grayed out

#### Integration (`mod.rs`)

**`ElementHint`**:
```rust
pub struct ElementHint {
    pub element: DetectedElement,  // What to click
    pub label: String,             // "a", "as", etc.
}
```

**Main Function**: `generate_hints_for_elements(elements, layout) -> Vec<ElementHint>`

---

### 6. Overlay Rendering (`overlay/`)

#### Absolute Positioning (`positioned.rs`)

**Purpose**: Position hints at exact screen coordinates

**Widget**: `AbsoluteHints`
- Takes list of `ElementHint`
- Renders each hint at `element.bounds.(x,y)`
- Uses iced's absolute positioning

#### Visual Styling (`styles.rs`)

**`HintAppearance`**: Configuration for hint badges
- Font size, padding, border radius
- Colors for different states

**`HintColor`**: RGBA color type
- Default: COSMIC blue (`#0088CC`)
- Match: Green
- No match: Gray (dimmed)

**State Management**: `HintState` enum
- `Default` - Normal blue badge
- `PartialMatch` - Green (user input matches prefix)
- `ExactMatch` - Fully matched
- `NoMatch` - Grayed out

#### Hint Widgets (`widgets.rs`)

**Purpose**: Create individual hint badge UI elements

**`hint_widget()`**: Creates a styled badge with hint label
- Small container with background color
- Text label (e.g., "as")
- Border, padding, border-radius

**`create_positioned_hints()`**: Maps hints to positioned widgets

---

### 7. Command Line Interface (`cli.rs`, `commands.rs`)

#### CLI (`cli.rs`)
```bash
cosmic-vimified           # Start daemon (default)
cosmic-vimified daemon    # Explicit daemon mode
cosmic-vimified show      # Show overlay
cosmic-vimified hide      # Hide overlay
cosmic-vimified toggle    # Toggle overlay
```

#### Internal Commands (`commands.rs`)
```rust
pub enum DaemonCommand {
    Show,    // Trigger detection + show hints
    Hide,    // Hide overlay, keep daemon
    Toggle,  // Toggle visibility
    Exit,    // Quit application
}
```

---

## Data Flow

### Show Overlay Flow

```
1. User presses Super+G or runs: cosmic-vimified show
   │
   ▼
2. CLI sends D-Bus method call: Show()
   │
   ▼
3. VimifiedDaemon receives call
   │
   ├─▶ Sends DaemonCommand::Show via mpsc channel
   │
   ▼
4. OverlayApp receives Message::DaemonCommand(Show)
   │
   ├─▶ Sets visible = true
   │
   ├─▶ Spawns async task: detect_elements()
   │   │
   │   ├─▶ AtSpiDetector::new() - Connect to AT-SPI
   │   │
   │   ├─▶ Traverse accessibility tree
   │   │
   │   ├─▶ Filter elements (role, state, size)
   │   │
   │   └─▶ Returns Vec<DetectedElement>
   │
   ▼
5. OverlayApp receives Message::ElementsDetected(elements)
   │
   ├─▶ generate_hints_for_elements(elements, layout)
   │   │
   │   └─▶ Returns Vec<ElementHint> with labels
   │
   ├─▶ Store in app.hints
   │
   ▼
6. view_impl() renders overlay
   │
   ├─▶ absolute_hints(hints, current_input, appearance)
   │
   └─▶ Displays blue hint badges at screen positions
```

### Hint Selection Flow

```
1. User types "as"
   │
   ▼
2. Message::CharInput('a')
   │
   ├─▶ app.current_input.push('a')
   │
   ├─▶ No exact match yet
   │
   ▼
3. view_impl() re-renders
   │
   ├─▶ Hints matching "a*" turn green
   │
   └─▶ Hints not matching turn gray
   │
   ▼
4. Message::CharInput('s')
   │
   ├─▶ app.current_input.push('s')  // Now "as"
   │
   ├─▶ find() checks for exact match
   │
   ├─▶ Match found!
   │
   ▼
5. Log click position (Phase 5: will trigger uinput click)
   │
   ├─▶ element.bounds.center() → (x, y)
   │
   └─▶ Send Message::CloseOverlay
   │
   ▼
6. Overlay hides, daemon stays running
```

---

## Phase Progress

### ✅ Phase 1: AT-SPI Detection
- Connect to accessibility bus
- Traverse application trees
- Extract element metadata and positions

### ✅ Phase 2: Filtering
- Role-based filtering (Button, Link, etc.)
- State filtering (Visible, Enabled, Focusable)
- Size filtering (10px-2000px)

### ✅ Phase 3: Hint Generation
- Base-8 label algorithm
- Keyboard layout support
- Progressive filtering as user types
- 29 unit tests passing

### ✅ Phase 4: Overlay Rendering
- Wayland layer shell integration
- Absolute positioning at screen coordinates
- Interactive highlighting (blue/green/gray)
- Keyboard passthrough when hidden

### 🚧 Phase 4 Issues (Current)
1. **Coordinate positioning** - Hints appear but not always on correct elements
2. **Too many hints** - Filtering needs more tuning
3. **Keyboard blocking** - OnDemand mode deployed, needs testing

### 🔜 Phase 5: Click Actions (TODO)
- Implement uinput for synthetic mouse clicks
- Click at `element.bounds.center()` coordinates
- Handle click failures gracefully
- Support different click types (left, middle, right)

### 🔜 Phase 6: COSMIC Integration (TODO)
- Add Super+G global keybinding via COSMIC settings
- Auto-start daemon on COSMIC session startup
- System tray integration (optional)

---

## Threading Model

```
┌──────────────────────────────────────────────┐
│        Main Thread (Tokio Runtime)           │
│                                              │
│  ┌─────────────────────────────────────┐    │
│  │      D-Bus Service Loop             │    │
│  │  (async, listens for commands)      │    │
│  └──────────────┬──────────────────────┘    │
│                 │                            │
│                 │ mpsc::channel              │
│                 ▼                            │
│  ┌─────────────────────────────────────┐    │
│  │   Overlay Application Thread        │    │
│  │   (spawned via std::thread)         │    │
│  │                                     │    │
│  │  Runs iced_layershell event loop   │    │
│  │  - Keyboard events                  │    │
│  │  - Rendering                        │    │
│  │  - AT-SPI detection (async tasks)   │    │
│  └─────────────────────────────────────┘    │
└──────────────────────────────────────────────┘
```

**Important**:
- Daemon stays alive when overlay closes
- Overlay can be spawned multiple times
- AT-SPI calls run as async tasks within iced

---

## Configuration

### Current (Hardcoded)
- Keyboard layout: Standard (home row)
- Hint colors: COSMIC blue, green, gray
- Size filters: 10px-2000px
- Font size: 12px

### Future (Phase 6)
- User-configurable layouts
- Custom color schemes
- Per-app exclusion list
- Size filter adjustments

---

## Testing

### Unit Tests
Location: Within each module (gated with `#[cfg(test)]`)

Coverage:
- **Hint generation**: 29 tests (all passing)
  - Label generation for 1-100+ elements
  - Keyboard layouts
  - Matching logic
- **Filtering**: Basic tests
- **Styles**: Color conversion tests

### Manual Testing Workflow
```bash
# Terminal 1: Start daemon with debug logging
cd cosmic-vimified
RUST_LOG=debug cargo run --release

# Terminal 2: Show overlay
./target/release/cosmic-vimified show

# Verify:
# - Hints appear on screen
# - ESC closes overlay
# - Daemon stays running
# - Can call 'show' again
```

---

## Dependencies

### Core Libraries
- **iced** / **iced_layershell**: UI framework and Wayland layer shell
- **atspi** / **atspi-connection**: Accessibility tree access
- **zbus**: D-Bus interface
- **tokio**: Async runtime
- **clap**: CLI parsing
- **tracing**: Structured logging

### Why These Choices?

**iced_layershell**:
- Native Wayland layer shell support
- Absolute positioning for hints
- Mouse passthrough capability

**AT-SPI**:
- Cross-desktop accessibility protocol
- Standardized element discovery
- Works with all AT-SPI enabled apps

**zbus**:
- Pure Rust D-Bus implementation
- Async/await support
- Macro-based interface definitions

---

## Known Issues & Limitations

### Current Bugs
1. **Coordinate mismatch**: Hints don't always appear exactly on elements
   - Possible AT-SPI coordinate system confusion
   - May need Wayland coordinate transformation

2. **Excessive hints**: Too many elements detected
   - Need more aggressive filtering
   - Consider Z-order/visibility checks

3. **Duplicate labels**: Sometimes same label appears twice
   - May be duplicate AT-SPI elements
   - Need deduplication logic

### Architectural Limitations
1. **AT-SPI dependency**: Apps must implement AT-SPI
   - Native Wayland apps: Usually supported
   - XWayland apps: May have issues
   - Games/custom renderers: Not detectable

2. **Phase approach**: Can't click yet (Phase 5 not implemented)

3. **Global state**: COMMAND_RX uses OnceCell
   - Works but not ideal pattern
   - Could refactor to Arc<Mutex<>> passed explicitly

---

## Future Enhancements

### Short Term (Phase 5)
- [ ] uinput click implementation
- [ ] Click type selection (left/middle/right)
- [ ] Error handling for failed clicks

### Medium Term (Phase 6)
- [ ] COSMIC keybinding integration
- [ ] Auto-start daemon
- [ ] Configuration file support
- [ ] Per-app hint customization

### Long Term
- [ ] Scroll mode (like Vimium's link hints for scrolling)
- [ ] Text selection mode
- [ ] Multi-monitor support improvements
- [ ] Performance optimizations for huge element counts
- [ ] Tab navigation integration

---

## Building & Development

### Build Release Binary
```bash
cd cosmic-vimified
cargo build --release
```

### Run Tests
```bash
cargo test
```

### Run with Debug Logging
```bash
RUST_LOG=debug cargo run
```

### Code Style
- Follow Rust API Guidelines
- Use `rustfmt` for formatting
- Run `clippy` for linting
- Add doc comments (///) for all public items

---

## Contributing

### Adding New Features

1. **New filter criteria**: Edit `detection/filters.rs`
2. **New keyboard layout**: Add to `hints/layout.rs`
3. **New hint style**: Modify `overlay/styles.rs`
4. **New command**: Add to `cli.rs` and `commands.rs`

### Testing Changes

1. Unit tests: Add to relevant module
2. Manual testing: Use debug logging
3. AT-SPI inspection: Use `accerciser` tool

---

## License

GPL-3.0-or-later

Copyright (C) 2025 COSMIC Vimified Contributors
