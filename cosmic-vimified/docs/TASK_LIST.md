# COSMIC Vimified - Development Task List

**Project:** Keyboard-driven hint navigation for COSMIC desktop environment
**Current Status:** Phase 0 Complete ✅ (Foundation & Research)
**Next Phase:** Phase 1 - libcosmic Application Scaffold
**Created:** 2025-11-08

## Project Overview

COSMIC Vimified brings Vimium-style keyboard navigation to the entire COSMIC desktop. Press Super+G, see hint labels appear over clickable elements, type the label, and interact - all without touching your mouse.

**Key Features:**
- Super+G activation
- Vimium-style hints overlaid on all clickable elements
- Left/right-click actions
- Vim scrolling with hjkl
- Multi-monitor support
- Highly configurable via RON config files

**Technical Stack:**
- Rust with libcosmic (iced framework)
- AT-SPI for element detection
- Layer Shell for overlay rendering
- uinput for click synthesis
- cosmic-config for settings

---

## Development Phases

### Phase 0: Foundation ✅ COMPLETE

**Goal:** Set up project and confirm technical feasibility

**Completed Tasks:**
- [x] Create project structure
- [x] Research COSMIC protocols and APIs
- [x] Identify required dependencies
- [x] Add dependencies to Cargo.toml
- [x] Verify project builds
- [x] Document research findings

**Deliverables:**
- ✅ Project compiles successfully
- ✅ TECHNICAL_RESEARCH.md complete
- ✅ All dependencies resolved

---

### Phase 1: libcosmic Application Scaffold

**Goal:** Create a basic libcosmic application that launches and displays a window

**Status:** Ready to start 🚀

#### Tasks

1. **Study libcosmic examples and App trait implementation**
   - Clone libcosmic repo and review examples/
   - Understand App trait implementation
   - Learn cosmic window/layer creation
   - Reference: https://github.com/pop-os/libcosmic/tree/master/examples

2. **Create src/app.rs with basic App structure and state**
   - Create `src/app.rs` with App trait
   - Set up application state struct
   - Implement basic update/view cycle
   - Add proper error handling

3. **Implement layer-shell transparent full-screen overlay**
   - Study smithay layer-shell documentation
   - Create full-screen transparent overlay window
   - Position on "overlay" layer (top-most)
   - Create `src/overlay/mod.rs` module

4. **Add keyboard input handling with ESC to close**
   - Capture all keyboard events
   - Implement ESC to close overlay
   - Log keypresses for testing
   - Create `src/overlay/input.rs`

**Success Criteria:**
- [ ] Application launches via `cargo run`
- [ ] Shows transparent full-screen overlay
- [ ] Responds to ESC key to close
- [ ] Clean shutdown on exit
- [ ] No critical errors or panics

**Files to Create:**
- `src/app.rs` - Main application struct
- `src/overlay/mod.rs` - Overlay window management
- `src/overlay/input.rs` - Keyboard input handling

**Estimated Time:** 1-2 days

---

### Phase 2: AT-SPI Element Detection

**Goal:** Query accessible tree and identify clickable elements

**Status:** Pending

#### Tasks

5. **Establish AT-SPI connection and traverse accessible tree**
   - Connect to accessibility bus
   - Get desktop accessible object
   - Traverse accessible tree recursively
   - Handle connection errors gracefully

6. **Implement element filtering for clickable items**
   - Query by roles (Button, Link, MenuItem, Icon, etc.)
   - Get element screen coordinates
   - Filter visible/enabled elements only
   - Check element is within screen bounds

7. **Create data structures for detected elements with positions**
   - Create `DetectedElement` struct with:
     - Element position (x, y, width, height)
     - Element role and name
     - Application name
     - Accessibility path
   - Implement Debug display

8. **Test element detection with different toolkits**
   - Launch COSMIC Files and detect buttons
   - Test with Firefox (GTK)
   - Test with COSMIC Terminal
   - Verify accurate coordinate retrieval

**Success Criteria:**
- [ ] Successfully connects to AT-SPI
- [ ] Detects buttons/links in test apps
- [ ] Retrieves accurate screen coordinates
- [ ] Logs detected elements with positions
- [ ] Works across COSMIC, GTK, and Qt apps

**Files to Create:**
- `src/detection/mod.rs` - Detection module
- `src/detection/atspi.rs` - AT-SPI integration
- `src/detection/filters.rs` - Element filtering logic
- `src/detection/types.rs` - Element data structures

**Reference Code:**
- atspi crate examples: https://docs.rs/atspi/
- Hints project (Python): https://github.com/AlfredoSequeida/hints

**Estimated Time:** 2-3 days

---

### Phase 3: Hint Generation Algorithm

**Goal:** Generate optimal hint labels for detected elements

**Status:** Pending

#### Tasks

9. **Implement hint label generator using home-row keys (asdfjkl;)**
   - Create home-row character sequence
   - Generate single-character labels (a, s, d, f, j, k, l, ;)
   - Generate two-character combinations (aa, as, ad, ...)
   - Optimize for fewest keystrokes
   - Support different keyboard layouts (left-hand, right-hand, custom)
   - Map labels to detected elements

10. **Create unit tests for hint generation algorithm**
    - Test with 1, 10, 100, 1000 elements
    - Verify no duplicate labels
    - Test keyboard layout variations
    - Benchmark performance

**Algorithm:**
```rust
chars = ['a', 's', 'd', 'f', 'j', 'k', 'l', ';']

// Single character
for c1 in chars:
    hints.push(c1)

// Two characters
for c1 in chars:
    for c2 in chars:
        hints.push(c1 + c2)

// Three characters (if needed)
for c1 in chars:
    for c2 in chars:
        for c3 in chars:
            hints.push(c1 + c2 + c3)
```

**Success Criteria:**
- [ ] Generates unique labels for N elements
- [ ] Uses home-row keys first
- [ ] Scales to 100+ elements
- [ ] Passes all unit tests
- [ ] No performance issues

**Files to Create:**
- `src/hints/mod.rs` - Hint module
- `src/hints/generator.rs` - Label generation algorithm
- `src/hints/layout.rs` - Keyboard layout definitions
- `tests/hint_generation_tests.rs` - Unit tests

**Estimated Time:** 1-2 days

---

### Phase 4: Overlay Rendering

**Goal:** Display hint labels on screen overlay

**Status:** Pending

#### Tasks

11. **Design and implement hint visual styling**
    - Color scheme (cosmic-theme integration)
    - Font size and weight (default 16pt)
    - Border and background styling
    - Border radius (4px default)
    - Opacity (0.95 default)

12. **Create libcosmic widgets for hint rendering**
    - Create custom hint widget
    - Position widgets at element coordinates
    - Handle coordinate transformation
    - Ensure hints stay within screen bounds

13. **Implement interactive highlighting as user types**
    - Highlight hints matching typed characters
    - Dim non-matching hints
    - Show visual feedback
    - Clear highlighting on ESC

**Success Criteria:**
- [ ] Hints render at correct element positions
- [ ] Visually appealing and readable
- [ ] Interactive highlighting works smoothly
- [ ] Renders efficiently (60fps with 100+ hints)
- [ ] Low CPU/memory usage

**Files to Create:**
- `src/overlay/renderer.rs` - Hint rendering logic
- `src/overlay/styles.rs` - Visual styling
- `src/overlay/widgets.rs` - Custom hint widgets

**Estimated Time:** 2-3 days

---

### Phase 5: Click Action Execution

**Goal:** Synthesize mouse clicks when user selects a hint

**Status:** Pending

#### Tasks

14. **Set up uinput virtual mouse device with proper permissions**
    - Create virtual mouse device using uinput
    - Handle permissions properly (udev rules)
    - Graceful error handling for permission errors
    - Create udev rules file

15. **Implement click synthesis (left and right-click)**
    - Move mouse cursor to element center
    - Generate left-click event
    - Support right-click with Shift modifier
    - Return cursor to original position (optional)

16. **Test click accuracy across different applications**
    - Verify clicks hit correct elements
    - Test in COSMIC Files
    - Test in Firefox
    - Test in COSMIC Terminal
    - Test multi-monitor setups

**Success Criteria:**
- [ ] Successfully creates uinput device
- [ ] Clicks hit correct elements consistently
- [ ] Works across different applications
- [ ] Handles permission errors gracefully
- [ ] Right-click with Shift works

**Files to Create:**
- `src/actions/mod.rs` - Action module
- `src/actions/click.rs` - Click synthesis
- `src/actions/uinput.rs` - uinput device management
- `files/system/etc/udev/rules.d/99-cosmic-vimified.rules` - udev rules

**System Requirements:**
- uinput kernel module loaded
- Proper udev permissions for /dev/uinput

**Estimated Time:** 2 days

---

### Phase 6: MVP Integration & Testing 🎯

**Goal:** Integrate all components into working MVP

**Status:** Pending

#### Tasks

17. **Wire all components together for end-to-end flow**
    - Super+G activation → Element detection
    - Element detection → Hint generation
    - Hint generation → Overlay rendering
    - User input → Hint selection
    - Hint selection → Click execution
    - Clean state management
    - Proper error handling throughout

18. **Perform end-to-end testing on real COSMIC desktop**
    - Test full workflow repeatedly
    - Use with different COSMIC applications
    - Test edge cases (no elements, many elements, etc.)
    - Fix crashes and errors
    - Handle edge cases
    - Improve reliability

**Success Criteria:**
- [ ] Complete workflow works end-to-end
- [ ] Reliable activation with Super+G
- [ ] Hints appear correctly
- [ ] Clicks work accurately
- [ ] No critical bugs
- [ ] Graceful error handling

**Deliverable:** **MVP Release - v0.1.0**

Basic features working:
- Super+G activation
- Hint display on clickable elements
- Left-click on hint selection
- ESC to cancel

**Estimated Time:** 1-2 days

---

### Phase 7: Configuration System

**Goal:** Implement cosmic-config integration

**Status:** Pending

#### Tasks

19. **Implement cosmic-config integration for configuration**
    - Define config schema (keybindings, appearance, behavior)
    - Use cosmic-config for loading/saving
    - Implement default values
    - Add config validation
    - Support hot-reload (watch for changes)
    - Create `examples/config.ron.example`
    - Document all configuration options

**Configuration Structure:**
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
        hint_opacity: 0.95,
    ),
    keyboard_layout: (
        mode: Standard,  // LeftHanded, RightHanded, Custom
    ),
)
```

**Success Criteria:**
- [ ] Config loads on startup
- [ ] Settings apply correctly
- [ ] Hot-reload works without restart
- [ ] Well-documented with examples
- [ ] Validation with helpful error messages

**Files to Create:**
- `src/config/mod.rs` - Config module
- `src/config/schema.rs` - Config schema definitions
- `examples/config.ron.example` - Example configuration

**Estimated Time:** 1-2 days

---

### Phase 8: Enhanced Features

**Goal:** Add right-click, vim scrolling, and multi-monitor support

**Status:** Pending

#### Tasks

20. **Add vim scrolling (hjkl) support**
    - Implement h/j/k/l scrolling when hints active
    - Determine scroll target (focused window)
    - Implement smooth scrolling
    - Configure scroll amount in config

21. **Implement multi-monitor support**
    - Detect all connected monitors
    - Handle coordinate mapping per monitor
    - Show hints on currently focused monitor only
    - Handle monitor hotplug events
    - Test on multi-monitor setup

**Success Criteria:**
- [ ] Right-click works with Shift+hint
- [ ] Vim scrolling functional (hjkl)
- [ ] Works correctly across multiple monitors
- [ ] Handles monitor configuration changes

**Files to Create:**
- `src/actions/scroll.rs` - Scroll implementation
- `src/display/monitors.rs` - Multi-monitor handling

**Estimated Time:** 2-3 days

---

### Phase 9: BlueBuild Integration & Packaging

**Goal:** Package for BlueBuild and integrate with image

**Status:** Pending

#### Tasks

22. **Create RPM packaging and BlueBuild integration**
    - Create RPM spec file
    - Create build script for BlueBuild
    - Install to correct system locations:
      - `/usr/bin/cosmic-vimified`
      - `/etc/cosmic-vimified/config.ron`
      - `/etc/udev/rules.d/99-cosmic-vimified.rules`
    - Add to `../recipes/recipe.yml`
    - Configure COSMIC keybinding registration
    - Build image with cosmic-vimified
    - Test on actual image (rebase and verify)

**Success Criteria:**
- [ ] Builds as RPM package
- [ ] Installs via BlueBuild recipe
- [ ] Works on fresh image install
- [ ] Keybinding pre-configured
- [ ] All permissions set correctly

**Files to Create:**
- `packaging/cosmic-vimified.spec` - RPM spec file
- `scripts/build-rpm.sh` - RPM build script
- `files/scripts/setup-cosmic-vimified.sh` - Image build script
- Updates to `../recipes/recipe.yml`

**Estimated Time:** 1-2 days

---

### Phase 10: Polish & Documentation

**Goal:** Polish UX and create comprehensive documentation

**Status:** Pending

#### Tasks

23. **Polish UX and write comprehensive documentation**
    - Smooth animations for hint appearance
    - Better visual feedback
    - Accessibility features (screen reader announcements)
    - Performance profiling and optimization
    - Reduce memory usage
    - Faster activation time
    - Write comprehensive documentation:
      - User guide
      - Configuration reference
      - Troubleshooting guide
      - Architecture documentation
    - Code cleanup and refactoring
    - Add comprehensive code comments
    - Improve error messages

**Success Criteria:**
- [ ] Polished, professional UX
- [ ] Smooth animations
- [ ] Comprehensive user documentation
- [ ] Clean, maintainable code
- [ ] Performance targets met:
  - Activation to display: < 100ms
  - Hint selection to action: < 50ms
  - Memory usage: < 50MB idle
  - CPU usage: < 5% idle

**Files to Create:**
- `docs/USER_GUIDE.md` - End-user documentation
- `docs/CONFIGURATION.md` - Configuration reference
- `docs/TROUBLESHOOTING.md` - Common issues and solutions
- Update `docs/ARCHITECTURE.md` - Complete architecture docs

**Deliverable:** **v1.0.0 Release**

**Estimated Time:** 2-3 days

---

## Total Timeline Estimate

**Development Time:** 15-25 days (3-5 weeks)

**Phase Breakdown:**
- Phase 1: 1-2 days (Application Scaffold)
- Phase 2: 2-3 days (Element Detection)
- Phase 3: 1-2 days (Hint Generation)
- Phase 4: 2-3 days (Overlay Rendering)
- Phase 5: 2 days (Click Execution)
- Phase 6: 1-2 days (MVP Integration) 🎯
- Phase 7: 1-2 days (Configuration)
- Phase 8: 2-3 days (Enhanced Features)
- Phase 9: 1-2 days (Packaging)
- Phase 10: 2-3 days (Polish & Docs)

---

## Success Metrics

### MVP (v0.1.0) 🎯
- ✅ Basic hint navigation works
- ✅ Click actions functional
- ✅ Runs on COSMIC desktop
- ✅ No critical bugs
- ✅ Super+G activation works

### v1.0.0 🚀
- ✅ All planned features implemented
- ✅ Right-click support
- ✅ Vim scrolling
- ✅ Configuration system complete
- ✅ Packaged for BlueBuild
- ✅ Multi-monitor support
- ✅ Comprehensive documentation
- ✅ Stable and reliable

---

## Development Workflow

### Daily Workflow
1. Pick a task from current phase
2. Create feature branch (optional)
3. Implement and test locally
4. Run `./scripts/dev-build.sh`
5. Manual testing on COSMIC desktop
6. Update documentation
7. Commit changes
8. Update this task list

### Testing Strategy
- **Unit tests** for algorithms (hint generation, etc.)
- **Integration tests** for components
- **Manual testing** on COSMIC desktop
- **Test with various applications** (COSMIC, GTK, Qt)
- **Multi-monitor testing** (when available)

### Code Quality
- Run `cargo fmt` before commits
- Run `cargo clippy -- -D warnings`
- Run `cargo test`
- Self-review code
- Check against specification

---

## Required Resources

### Development Environment
- COSMIC desktop environment (Fedora 43+)
- Rust toolchain (stable)
- Development dependencies installed (see DEVELOPMENT.md)

### Testing Applications
- COSMIC Files
- COSMIC Terminal
- COSMIC Settings
- Firefox
- GTK applications
- Qt applications

### Hardware
- Multi-monitor setup (for testing)
- Mouse and keyboard

---

## Documentation References

- [libcosmic examples](https://github.com/pop-os/libcosmic/tree/master/examples)
- [atspi documentation](https://docs.rs/atspi/)
- [Hints project](https://github.com/AlfredoSequeida/hints) (Python implementation)
- [smithay layer shell](https://github.com/Smithay/smithay)
- [COSMIC protocols](https://github.com/pop-os/cosmic-protocols)
- [Vimium browser extension](https://github.com/philc/vimium)

---

## Next Immediate Steps

1. **Start Phase 1** - Study libcosmic examples
2. **Clone libcosmic repo** - Review example applications
3. **Create src/app.rs** - Basic application structure
4. **Get overlay window working** - First visual milestone

---

**Let's build cosmic-vimified! 🚀**

*Last updated: 2025-11-08*
