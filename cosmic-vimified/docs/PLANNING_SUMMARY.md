# COSMIC Vimified - Planning Summary

**Date:** 2025-11-07
**Phase:** Initial Planning Complete
**Next Phase:** Architecture Design & Prototyping

## What We Accomplished

### 1. Requirements Gathering ✓
- Conducted comprehensive requirements interview with 10 key questions
- Defined scope, features, and priorities
- Identified MVP features vs future enhancements

### 2. Specification Document ✓
Created comprehensive specification covering:
- Activation mechanism (f or Super+f, toggle mode)
- Feature scope (click, right-click, vim scrolling)
- Hint label generation (Vimium-style, home-row optimized)
- Element detection strategy (AT-SPI based)
- Application scope (COSMIC focus, allow/deny lists)
- Configuration system (RON files, highly customizable)
- Performance targets and constraints

**Document:** [SPEC.md](SPEC.md)

### 3. Repository Structure ✓
Set up complete project structure:

```
cosmic-vimified/
├── Cargo.toml                   # Rust project configuration
├── .gitignore                   # Git ignore rules
├── README.md                    # Project overview
├── docs/
│   ├── SPEC.md                 # Technical specification (complete)
│   ├── ARCHITECTURE.md         # Architecture design (placeholder)
│   ├── DEVELOPMENT.md          # Developer guide (template)
│   └── PLANNING_SUMMARY.md     # This document
├── src/
│   └── main.rs                 # Entry point (placeholder)
├── tests/                      # Integration tests (empty)
├── examples/
│   └── config.ron.example      # Example configuration
└── scripts/
    └── dev-build.sh            # Development build script
```

### 4. Build Integration Plan ✓
Designed multi-phase build integration:

**Phase 1 - Development (Now):**
- Local builds via `cargo build`
- Quick iteration with `./scripts/dev-build.sh`
- Manual install to `/usr/local/bin`

**Phase 2 - Image Integration (Future):**
- BlueBuild script to compile during image creation
- Install to `/usr/bin/cosmic-vimified`
- Systemd service auto-start

**Phase 3 - Package Distribution (Future):**
- RPM package via COPR
- Install via `rpm-ostree install cosmic-vimified`
- Separate update pipeline from image builds

## Key Decisions Made

### 1. Naming
- **Project Name:** COSMIC Vimified
- **Package Name:** cosmic-vimified
- **Binary Name:** cosmic-vimified

### 2. Focus Area
- **Primary:** COSMIC native applications (libcosmic/iced)
- **Secondary:** GTK, Qt, Firefox (best effort)
- **Why:** Quality over breadth, one-person project

### 3. Technology Stack
- **Language:** Rust
- **UI Framework:** libcosmic (iced)
- **Accessibility:** AT-SPI via cosmic-atspi protocol
- **Config Format:** RON (COSMIC standard)
- **Overlay:** Wayland layer-shell

### 4. MVP Feature Set
**In Scope:**
- Single-key activation (f)
- Click and right-click
- Vim scrolling (hjkl)
- Basic configuration
- COSMIC app support

**Deferred:**
- Double-click
- Drag and drop
- Hover actions
- Complex action chains

### 5. Configuration Philosophy
- User-friendly defaults (inspired by Vimium)
- Highly customizable (RON config files)
- Per-application overrides
- Hot-reload configuration changes

## Open Technical Questions

These need research during architecture phase:

1. **Global Keyboard Shortcuts**
   - How to register in cosmic-comp?
   - Can we use single key (f) without conflicts?
   - API/protocol to use?

2. **Layer Shell Overlays**
   - Which layer-shell protocol (wlr vs cosmic-specific)?
   - How to ensure hints appear above all windows?
   - Performance of overlay rendering?

3. **AT-SPI Integration**
   - How complete is cosmic-atspi implementation?
   - Can we query non-COSMIC apps?
   - Performance of element detection?

4. **Click Synthesis**
   - How to programmatically click via AT-SPI?
   - Or should we synthesize mouse events?
   - Which method is more reliable?

5. **Daemon vs Applet**
   - Should this be a panel applet or background service?
   - Can panel applets register global shortcuts?
   - Resource implications of each approach?

## Next Steps

### Immediate (Week 1)
1. **Research Phase:** Answer open technical questions
   - Study cosmic-comp source code for global shortcuts
   - Experiment with layer-shell protocols
   - Test AT-SPI on COSMIC desktop
   - Prototype click synthesis

2. **Architecture Design:** Document findings in ARCHITECTURE.md
   - Component diagram
   - Data flow
   - API interfaces
   - Integration points

### Short Term (Week 2-3)
3. **Prototype Development:**
   - Minimal viable overlay (show/hide on shortcut)
   - Basic AT-SPI element detection
   - Simple hint rendering
   - Proof of concept click action

4. **MVP Development:**
   - Complete hint generation algorithm
   - Full element detection
   - Click and right-click support
   - Basic configuration loading

### Medium Term (Week 4-6)
5. **Feature Completion:**
   - Vim scrolling
   - Allow/deny lists
   - Per-app configuration
   - Visual customization

6. **Testing & Polish:**
   - Unit tests
   - Integration tests
   - Performance optimization
   - Documentation

### Long Term (Month 2-3)
7. **Packaging:**
   - RPM spec file
   - COPR repository setup
   - Systemd service integration
   - BlueBuild integration

8. **Release:**
   - v0.1.0 beta release
   - User testing and feedback
   - Bug fixes
   - v1.0.0 stable release

## Success Criteria

### MVP Success
- [ ] Activation shortcut works globally
- [ ] Hints appear on COSMIC apps
- [ ] Can click elements via hints
- [ ] Configuration file works
- [ ] Performance: <100ms activation latency

### v1.0 Success
- [ ] All MVP features working
- [ ] Right-click support
- [ ] Vim scrolling
- [ ] Multi-monitor support
- [ ] RPM package available
- [ ] Documentation complete
- [ ] 5+ beta testers with positive feedback

## Resources & References

- [Vimium](https://github.com/philc/vimium) - Original inspiration
- [Hints for Linux](https://github.com/AlfredoSequeida/hints) - Similar project for GTK/X11
- [COSMIC Toolkit](https://pop-os.github.io/libcosmic-book/) - Development framework
- [COSMIC Protocols](https://github.com/pop-os/cosmic-protocols) - Wayland protocols
- [cosmic-atspi Protocol](https://wayland.app/protocols/cosmic-atspi-unstable-v1) - Accessibility
- [AT-SPI Rust Bindings](https://docs.rs/atspi/latest/atspi/) - Element detection

## Team & Contributions

**Lead Developer:** dverdonschot

**Contributions Welcome:**
- Code review during development
- Testing on different hardware
- Documentation improvements
- Bug reports and feature requests

## License

TBD (to be decided before first release)

---

## Summary

We've successfully completed the planning phase for COSMIC Vimified. We have:

1. A clear vision and scope
2. Comprehensive technical specification
3. Well-organized repository structure
4. Build and integration plan
5. Realistic roadmap

**We're ready to move into the Architecture Design & Prototyping phase.**

The next critical step is answering the open technical questions through research and experimentation, which will inform our architectural decisions.
