  1. Build the Project

    First, ensure the project compiles without errors:

    cd cosmic-vimified
    cargo build

    Expected: Clean build with no errors.

    2. Run the Application

    Launch the application with debug logging enabled:

    RUST_LOG=debug cargo run

    What to verify:
    - ✅ Application launches without panicking
    - ✅ You see log output: "COSMIC Vimified starting..." and version info
    - ✅ A window or overlay appears on your screen
    - ✅ The overlay is transparent (you can see through it)
    - ✅ The overlay covers the full screen

    3. Test ESC Key Handling

    With the application running:

    1. Press the ESC key
    2. The application should close cleanly

    What to verify:
    - ✅ ESC key closes the application
    - ✅ Clean shutdown with no error messages
    - ✅ No zombie processes left behind

    4. Check for Errors

    Review the terminal output for:
    - ✅ No panic messages
    - ✅ No critical errors
    - ✅ Proper initialization logs
    - ✅ Clean shutdown logs

    5. Additional Verification Tests

    Run these commands to ensure code quality:

    # Check code formatting
    cargo fmt --check

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
