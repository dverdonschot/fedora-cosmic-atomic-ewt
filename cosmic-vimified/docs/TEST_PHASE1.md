  cargo fmt --check                                                                                                                                                                                [0/111]

  # Run clippy for warnings
  cargo clippy -- -D warnings

  # Run any existing tests
  cargo test

  6. Manual Interaction Tests

  While the app is running, try:

  - Pressing various keys (they should be logged if input handling is working)
  - Moving the mouse (overlay should stay on top)
  - Switching between workspaces (if applicable)
  - Opening other applications (overlay should remain visible on top)

  ---
  Expected Behavior Checklist

  Here's what Phase 1 should deliver:

  - cargo run successfully launches the app
  - Transparent full-screen overlay appears
  - ESC key closes the application
  - No panics or critical errors
  - Clean shutdown when exiting
  - Debug logs show proper initialization

  ---
  Common Issues & Troubleshooting

  If the app doesn't compile:

  cargo clean
  cargo update
  cargo build

  If you don't see an overlay:

  - Check that you're running on COSMIC desktop
  - Verify Wayland is being used (not X11)
  - Check logs for error messages about window creation

  If ESC doesn't work:

  - Check src/overlay/input.rs for keyboard event handling
  - Review debug logs to see if keyboard events are being captured

  ---
  Next Steps After Phase 1 Verification

  Once you confirm all Phase 1 criteria are met, you'll be ready for Phase 2: AT-SPI Element Detection, which involves:

  1. Connecting to the accessibility bus
  2. Detecting clickable elements on screen
  3. Retrieving element positions
  4. Testing across different applications (COSMIC Files, Firefox, etc.)

  Would you like me to help you run these tests, or do you have specific questions about any of the verification steps?
