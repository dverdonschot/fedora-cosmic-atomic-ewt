# Overlay Window Status

## Current Implementation

The overlay window now has the following settings (src/app.rs:103-112):

```rust
window::open(window::Settings {
    size: cosmic::iced::Size::new(1920.0, 1080.0),
    position: window::Position::Centered,
    transparent: true,
    decorations: false,
    resizable: false,
    level: window::Level::AlwaysOnTop,
    exit_on_close_request: false,
    ..Default::default()
});
```

### What Works ✅
- **Transparent window**: Background is see-through
- **No decorations**: No title bar or window controls
- **Always on top**: Window stays above other applications
- **Not resizable**: User cannot resize the window
- **Centered position**: Starts in center of screen

### What Still Needs Work ❌

#### 1. **Click-Through (Mouse Events Pass Through)**
**Current State**: Window captures mouse events - you can click/drag the window
**Desired State**: Mouse events should pass through transparent areas to apps below
**Solution**: Requires platform-specific Wayland layer shell integration

#### 2. **True Fullscreen Coverage**
**Current State**: Fixed 1920x1080 size
**Desired State**: Should dynamically cover entire screen regardless of resolution
**Solution**: Query screen dimensions and set window size accordingly

#### 3. **Layer Shell Integration**
**Current State**: Using regular window with `AlwaysOnTop`
**Desired State**: Use Wayland layer shell for proper overlay behavior
**Why**: Layer shell provides:
- Proper click-through via input regions
- Better stacking control (overlay layer)
- Integration with compositor for performance
- No window decorations/controls at compositor level

## Solutions

### Option 1: Use `iced_layershell` Crate (Recommended)

Add to Cargo.toml:
```toml
iced_layershell = "0.13.7"
```

Advantages:
- Purpose-built for Wayland overlays
- Full layer shell protocol support
- Click-through regions
- Proper overlay layer positioning

Disadvantages:
- Major refactor required (different Application trait)
- Wayland-only (no X11 fallback)
- Learning curve for new API

### Option 2: Use libcosmic Applet Mode

libcosmic has built-in applet support which uses layer shell internally.

Advantages:
- Already part of libcosmic ecosystem
- Simpler integration
- COSMIC-native approach

Disadvantages:
- Designed for panel applets, not full-screen overlays
- May not provide all needed controls

### Option 3: Platform-Specific Window Hints (Interim)

Use `window::Settings::platform_specific` to set Wayland-specific properties.

Advantages:
- Minimal code changes
- Works with existing architecture

Disadvantages:
- Less control than layer shell
- May not solve click-through
- Compositor-dependent behavior

## Recommended Path Forward

### Phase 2.5: Improve Current Overlay (Quick Wins)

1. **Dynamic Screen Size**
   ```rust
   // Get screen dimensions from window manager
   let screen_size = get_screen_dimensions();
   window::Settings {
       size: screen_size,
       // ...
   }
   ```

2. **Test Current Behavior**
   - Verify AlwaysOnTop works on COSMIC
   - Check if window manager handles transparent windows properly
   - Document any compositor-specific quirks

### Phase 3: Full Layer Shell Integration (If Needed)

Only pursue if current approach doesn't meet requirements:

1. Add `iced_layershell` dependency
2. Create separate layer shell application implementation
3. Implement proper input regions for click-through
4. Set layer to "overlay" for topmost positioning
5. Define exclusive zone (likely 0 for overlay)

## Testing Current Build

```bash
# Start daemon
RUST_LOG=debug ./target/release/cosmic-vimified daemon

# Show overlay
./target/release/cosmic-vimified show
```

### What to Test:
1. Does overlay appear on top of all windows?
2. Can you still interact with windows below?
3. Are window controls (close, minimize) visible?
4. Does transparency work correctly?
5. Can you move/resize the overlay window?

### Expected Issues:
- ❌ Window can be moved/clicked (not click-through yet)
- ❌ May not cover entire screen on non-1920x1080 displays
- ✅ Should be transparent
- ✅ Should have no decorations
- ✅ Should stay on top

## Next Steps

After testing:
1. If click-through is critical for Phase 3, plan layer shell migration
2. If current approach acceptable, proceed with hint rendering
3. Add dynamic screen sizing
4. Consider fullscreen mode as alternative

---

**Current Priority**: Test what we have, then decide on layer shell migration
**Estimated Effort**: Layer shell migration = ~4-6 hours of refactoring
**Alternative**: Accept click-to-focus behavior and use Alt+ESC to hide
