# COSMIC Vimified - Implementation Roadmap

**Status:** Ready to Implement
**Last Updated:** 2025-11-08

## Overview

This document outlines the step-by-step implementation plan for cosmic-vimified, organized into phases with clear milestones and deliverables.

## Current Status

✅ **Completed:**
- Project structure created
- Research completed (see TECHNICAL_RESEARCH.md)
- Dependencies identified and added to Cargo.toml
- Project builds successfully
- Technical feasibility confirmed

📍 **Current Phase:** Ready to start implementation

## Implementation Phases

### Phase 0: Foundation (Complete! ✅)

**Goal:** Set up project and confirm technical feasibility

- [x] Create project structure
- [x] Research COSMIC protocols and APIs
- [x] Identify required dependencies
- [x] Add dependencies to Cargo.toml
- [x] Verify project builds
- [x] Document research findings

**Deliverables:**
- ✅ Project compiles
- ✅ TECHNICAL_RESEARCH.md complete
- ✅ All dependencies resolved

---

### Phase 1: libcosmic Application Scaffold

**Goal:** Create a basic libcosmic application that launches and displays a window

**Tasks:**
1. Study libcosmic examples
   - Clone libcosmic repo and review examples/
   - Understand App trait implementation
   - Learn cosmic window/layer creation

2. Implement basic App structure
   - Create `src/app.rs` with App trait
   - Set up application state
   - Implement basic update/view cycle

3. Create layer-shell overlay window
   - Study smithay layer-shell documentation
   - Create full-screen transparent overlay
   - Position on "overlay" layer (top-most)

4. Add keyboard input handling
   - Capture all keyboard events
   - Implement ESC to close
   - Log keypresses for testing

**Success Criteria:**
- [ ] Application launches via `cargo run`
- [ ] Shows transparent full-screen overlay
- [ ] Responds to ESC key to close
- [ ] Clean shutdown on exit

**Files to Create:**
- `src/app.rs` - Main application struct
- `src/overlay/mod.rs` - Overlay window management
- `src/overlay/input.rs` - Keyboard input handling

**Estimated Time:** 1-2 days

---

### Phase 2: AT-SPI Element Detection

**Goal:** Query accessible tree and identify clickable elements

**Tasks:**
1. Establish AT-SPI connection
   - Connect to accessibility bus
   - Get desktop accessible object
   - Traverse accessible tree

2. Filter clickable elements
   - Query by roles (Button, Link, MenuItem, etc.)
   - Get element coordinates
   - Filter visible/enabled elements only

3. Data structure for elements
   - Create struct for detected elements
   - Store position, role, name
   - Implement debug display

4. Test element detection
   - Launch simple test applications
   - Verify element detection accuracy
   - Test with different toolkits (GTK, Qt, libcosmic)

**Success Criteria:**
- [ ] Successfully connects to AT-SPI
- [ ] Detects buttons/links in test apps
- [ ] Retrieves accurate screen coordinates
- [ ] Logs detected elements with positions

**Files to Create:**
- `src/detection/mod.rs` - Detection module
- `src/detection/atspi.rs` - AT-SPI integration
- `src/detection/filters.rs` - Element filtering
- `src/detection/types.rs` - Element data structures

**Reference Code:**
- atspi crate examples
- Hints project (Python, but same concepts)

**Estimated Time:** 2-3 days

---

### Phase 3: Hint Generation Algorithm

**Goal:** Generate optimal hint labels for detected elements

**Tasks:**
1. Implement label generator
   - Create home-row sequence (asdfjkl;)
   - Generate two-character combinations
   - Optimize for fewest keystrokes

2. Assign hints to elements
   - Map labels to elements
   - Handle large numbers of elements
   - Support different keyboard layouts

3. Test generation algorithm
   - Unit tests for label generation
   - Test with varying numbers of elements (1, 10, 100, 1000)
   - Verify no duplicate labels

**Success Criteria:**
- [ ] Generates unique labels for N elements
- [ ] Uses home-row keys first
- [ ] Scales to 100+ elements
- [ ] Passes all unit tests

**Files to Create:**
- `src/hints/mod.rs` - Hint module
- `src/hints/generator.rs` - Label generation algorithm
- `src/hints/layout.rs` - Keyboard layout definitions
- `tests/hint_generation_tests.rs` - Unit tests

**Algorithm:**
```
chars = ['a', 's', 'd', 'f', 'j', 'k', 'l', ';']
hints = []

# Single character
for c1 in chars:
    hints.append(c1)

# Two characters
for c1 in chars:
    for c2 in chars:
        hints.append(c1 + c2)
```

**Estimated Time:** 1-2 days

---

### Phase 4: Overlay Rendering

**Goal:** Display hint labels on screen overlay

**Tasks:**
1. Design hint visual style
   - Color scheme (cosmic-theme integration)
   - Font size and weight
   - Border/background styling

2. Implement hint widgets
   - Create libcosmic widgets for hints
   - Position widgets at element coordinates
   - Style hints attractively

3. Highlight matching hints
   - As user types, highlight partial matches
   - Dim non-matching hints
   - Show visual feedback

4. Performance optimization
   - Render efficiently for 100+ hints
   - Smooth animations
   - Low CPU/memory usage

**Success Criteria:**
- [ ] Hints render at correct positions
- [ ] Visually appealing and readable
- [ ] Interactive highlighting works
- [ ] Renders smoothly (60fps)

**Files to Create:**
- `src/overlay/renderer.rs` - Hint rendering
- `src/overlay/styles.rs` - Visual styling
- `src/overlay/widgets.rs` - Custom widgets

**Estimated Time:** 2-3 days

---

### Phase 5: Click Action Execution

**Goal:** Synthesize mouse clicks when user selects a hint

**Tasks:**
1. Set up uinput device
   - Create virtual mouse device
   - Handle permissions properly
   - Graceful error handling

2. Implement click synthesis
   - Move mouse to element position
   - Generate left-click event
   - Support right-click (Shift modifier)

3. Test click accuracy
   - Verify clicks hit correct elements
   - Test across different applications
   - Test multi-monitor setups

4. Error handling
   - Handle uinput permission errors
   - Graceful degradation
   - User-friendly error messages

**Success Criteria:**
- [ ] Successfully creates uinput device
- [ ] Clicks hit correct elements
- [ ] Works across different apps
- [ ] Handles permission errors gracefully

**Files to Create:**
- `src/actions/mod.rs` - Action module
- `src/actions/click.rs` - Click synthesis
- `src/actions/uinput.rs` - uinput device management

**System Files to Create:**
- `files/system/etc/udev/rules.d/99-cosmic-vimified.rules`

**Estimated Time:** 2 days

---

### Phase 6: Integration & MVP Testing

**Goal:** Integrate all components into working MVP

**Tasks:**
1. Wire all components together
   - Launch → Detect → Generate → Render → Click
   - Handle full interaction flow
   - Clean state management

2. End-to-end testing
   - Test on real COSMIC desktop
   - Use with different applications
   - Test edge cases

3. Bug fixing
   - Fix crashes and errors
   - Handle edge cases
   - Improve reliability

4. Basic configuration
   - Add simple config file support
   - Support appearance customization
   - Document configuration options

**Success Criteria:**
- [ ] Complete workflow works end-to-end
- [ ] Reliable on COSMIC desktop
- [ ] No critical bugs
- [ ] Basic config works

**Deliverable:** **MVP Release - v0.1.0**

**Estimated Time:** 1-2 days

---

### Phase 7: Configuration System

**Goal:** Implement cosmic-config integration

**Tasks:**
1. Define config schema
   - Keybindings settings
   - Appearance settings
   - Behavior settings

2. Implement config loading
   - Use cosmic-config
   - Default values
   - Validation

3. Support hot-reload
   - Watch for config changes
   - Update UI dynamically
   - No restart required

4. Create config examples
   - Document all options
   - Provide examples/config.ron.example

**Success Criteria:**
- [ ] Config loads on startup
- [ ] Settings apply correctly
- [ ] Hot-reload works
- [ ] Well-documented

**Files to Create:**
- `src/config/mod.rs` - Config module
- `src/config/schema.rs` - Config schema
- `examples/config.ron.example` - Example config

**Estimated Time:** 1-2 days

---

### Phase 8: Enhanced Features

**Goal:** Add right-click and vim scrolling

**Tasks:**
1. Right-click support
   - Detect Shift modifier
   - Synthesize right-click
   - Test context menus

2. Vim scrolling
   - Implement hjkl scrolling
   - Determine scroll target
   - Smooth scrolling

3. Multi-monitor support
   - Detect all monitors
   - Handle coordinate mapping
   - Test on multi-monitor setup

**Success Criteria:**
- [ ] Right-click works with Shift+hint
- [ ] Vim scrolling functional
- [ ] Works across multiple monitors

**Files to Create:**
- `src/actions/scroll.rs` - Scroll implementation

**Estimated Time:** 2-3 days

---

### Phase 9: BlueBuild Integration

**Goal:** Package for BlueBuild and integrate with image

**Tasks:**
1. Create build script
   - RPM spec file or packaging script
   - Install to correct locations
   - Include udev rules

2. Add to BlueBuild recipe
   - Add package to recipe.yml
   - Configure COSMIC keybinding
   - Test image build

3. Test on actual image
   - Build image with cosmic-vimified
   - Rebase and test
   - Verify permissions work

**Success Criteria:**
- [ ] Builds as RPM/package
- [ ] Installs via BlueBuild
- [ ] Works on fresh install
- [ ] Keybinding pre-configured

**Files to Create:**
- `scripts/build-rpm.sh` - RPM build script
- `cosmic-vimified.spec` - RPM spec file
- Updates to `../recipes/recipe.yml`

**Estimated Time:** 1-2 days

---

### Phase 10: Polish & Documentation

**Goal:** Polish UX and create user documentation

**Tasks:**
1. UX improvements
   - Smooth animations
   - Better visual feedback
   - Accessibility features

2. Performance tuning
   - Profile and optimize
   - Reduce memory usage
   - Faster activation

3. Documentation
   - User guide
   - Configuration reference
   - Troubleshooting guide

4. Code cleanup
   - Refactor for clarity
   - Add comprehensive comments
   - Improve error messages

**Success Criteria:**
- [ ] Polished, professional UX
- [ ] Comprehensive documentation
- [ ] Clean, maintainable code

**Files to Create:**
- `docs/USER_GUIDE.md` - End-user documentation
- `docs/CONFIGURATION.md` - Config reference
- `docs/TROUBLESHOOTING.md` - Common issues

**Estimated Time:** 2-3 days

---

## Total Estimated Timeline

**Development Time:** 15-25 days (3-5 weeks)

**Breakdown:**
- Phase 1: 1-2 days
- Phase 2: 2-3 days
- Phase 3: 1-2 days
- Phase 4: 2-3 days
- Phase 5: 2 days
- Phase 6: 1-2 days
- Phase 7: 1-2 days
- Phase 8: 2-3 days
- Phase 9: 1-2 days
- Phase 10: 2-3 days

## Development Workflow

### Daily Workflow
1. Pick a task from current phase
2. Create feature branch
3. Implement and test
4. Update documentation
5. Commit and push
6. Update roadmap progress

### Testing Strategy
- Unit tests for algorithms
- Integration tests for components
- Manual testing on COSMIC desktop
- Test with various applications

### Code Review
- Self-review before commit
- Check against specification
- Verify no regressions

## Success Metrics

**MVP (v0.1.0):**
- ✅ Basic hint navigation works
- ✅ Click actions functional
- ✅ Runs on COSMIC desktop
- ✅ No critical bugs

**v1.0.0:**
- ✅ All planned features implemented
- ✅ Configuration system complete
- ✅ Packaged for BlueBuild
- ✅ Comprehensive documentation
- ✅ Stable and reliable

## Next Steps

1. **Immediate:** Start Phase 1 - libcosmic Application Scaffold
2. **This Week:** Complete Phases 1-3 (MVP foundation)
3. **This Month:** Complete MVP (Phases 1-6)
4. **Next Month:** Complete v1.0 (All phases)

## Resources

**Required for Development:**
- COSMIC desktop environment (for testing)
- Test applications (Firefox, COSMIC Files, etc.)
- Multi-monitor setup (for testing)

**Documentation References:**
- libcosmic examples: https://github.com/pop-os/libcosmic/tree/master/examples
- atspi docs: https://docs.rs/atspi/
- Hints project: https://github.com/AlfredoSequeida/hints
- smithay layer shell: https://github.com/Smithay/smithay

---

**Let's build cosmic-vimified! 🚀**
