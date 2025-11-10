use crate::commands::DaemonCommand;
use crate::detection::{atspi::AtSpiDetector, DetectedElement};
use crate::hints::{generate_hints_for_elements, ElementHint, KeyboardLayout};
use crate::overlay::{HintAppearance, absolute_hints};
use iced::widget::{container, text};
use iced::{Color, Element, Event, Length, Task as Command, event};
use iced::futures;
use iced::Subscription;
use iced_layershell::Application;
use iced_layershell::settings::Settings;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, StartMode};
use iced_layershell::to_layer_message;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;

/// Global channel receiver for daemon commands.
///
/// This static is initialized once at application startup and provides the mechanism
/// for the overlay to receive commands from the D-Bus daemon service.
static COMMAND_RX: once_cell::sync::OnceCell<Arc<Mutex<mpsc::UnboundedReceiver<DaemonCommand>>>> =
    once_cell::sync::OnceCell::new();

/// Main overlay application state for the Wayland layer shell window.
///
/// This struct manages the full-screen transparent overlay that displays
/// keyboard hints for clickable elements detected via AT-SPI.
pub struct OverlayApp {
    /// Whether the overlay is currently visible to the user
    visible: bool,
    /// Elements detected from the accessibility tree via AT-SPI
    detected_elements: Vec<DetectedElement>,
    /// Generated hints with labels for each detected element
    hints: Vec<ElementHint>,
    /// Current user input for hint selection (e.g., "a", "as", "ad")
    current_input: String,
    /// Visual appearance configuration for hint badges
    appearance: HintAppearance,
}

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    /// Close/hide the overlay
    CloseOverlay,
    /// Elements detected from AT-SPI
    ElementsDetected(Vec<DetectedElement>),
    /// Detection error
    DetectionError(String),
    /// Daemon command received
    DaemonCommand(DaemonCommand),
    /// Iced event (for debugging)
    IcedEvent(Event),
    /// Character input for hint selection
    CharInput(char),
}

impl Application for OverlayApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Start HIDDEN - only show when user presses Super+G (sends DaemonCommand::Show)
        // Do NOT trigger element detection on startup
        (
            OverlayApp {
                visible: false,  // Changed from true to false - wait for Super+G
                detected_elements: Vec::new(),
                hints: Vec::new(),
                current_input: String::new(),
                appearance: HintAppearance::default(),
            },
            Command::none()  // Don't detect elements until user requests
        )
    }

    fn namespace(&self) -> String {
        String::from("cosmic-vimified-overlay")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update_impl(self, message)
    }

    fn view(&self) -> Element<'_, Message> {
        view_impl(self)
    }

    fn subscription(&self) -> Subscription<Message> {
        subscription_impl(self)
    }

    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        iced_layershell::Appearance {
            background_color: Color::TRANSPARENT,
            text_color: theme.palette().text,
        }
    }
}

/// Runs the overlay application in persistent daemon mode.
///
/// This creates a long-running layer shell window that stays hidden until commanded to show.
/// The window uses OnDemand keyboard interactivity to avoid blocking keyboard input when hidden.
///
/// # Arguments
///
/// * `command_rx` - Channel receiver for D-Bus daemon commands
///
/// # Errors
///
/// Returns an error if the layer shell initialization fails.
pub fn run(command_rx: mpsc::UnboundedReceiver<DaemonCommand>) -> Result<(), iced_layershell::Error> {
    // Store command receiver in global static
    COMMAND_RX.set(Arc::new(Mutex::new(command_rx)))
        .expect("COMMAND_RX already initialized");

    OverlayApp::run(Settings {
        layer_settings: LayerShellSettings {
            // Full screen transparent overlay
            size: None, // Let iced determine size from content
            exclusive_zone: 0, // Don't reserve space
            anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode: StartMode::Active,
            // OnDemand - only capture keyboard when explicitly requested
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::OnDemand,
            // CRITICAL: Allow pointer/mouse events to pass through to apps below
            events_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Runs a single one-shot overlay instance.
///
/// This creates a self-contained overlay window that captures keyboard exclusively
/// until closed with ESC or a hint selection. Used when spawning the overlay on-demand
/// from the daemon (e.g., when user presses Super+G).
///
/// # Errors
///
/// Returns an error if the layer shell initialization fails.
pub fn run_once() -> Result<(), iced_layershell::Error> {
    OverlayApp::run(Settings {
        layer_settings: LayerShellSettings {
            // Full screen visible overlay
            size: Some((0, 0)), // With full anchors, means fill screen
            exclusive_zone: 0, // Don't reserve space
            anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode: StartMode::Active,
            // Capture keyboard for ESC and hint input
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
            // CRITICAL: Allow pointer events to pass through
            events_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Creates subscriptions for application events and daemon commands.
///
/// Subscribes to both iced framework events (keyboard, mouse, etc.) and
/// daemon commands received via the D-Bus service.
fn subscription_impl(_app: &OverlayApp) -> iced::Subscription<Message> {
    // Subscribe to both iced events AND daemon commands
    iced::Subscription::batch(vec![
        event::listen().map(Message::IcedEvent),
        daemon_command_subscription(),
    ])
}

/// Creates a subscription stream for daemon commands from D-Bus.
///
/// This subscription listens to the global COMMAND_RX channel and converts
/// received commands into Message::DaemonCommand events for the application.
fn daemon_command_subscription() -> Subscription<Message> {
    Subscription::run_with_id(
        "daemon_commands",
        futures::stream::unfold((), |_| async {
            if let Some(rx) = COMMAND_RX.get() {
                let mut rx = rx.lock().await;
                if let Some(cmd) = rx.recv().await {
                    return Some((Message::DaemonCommand(cmd), ()));
                }
            }
            // Keep the stream alive
            futures::future::pending::<()>().await;
            None
        })
    )
}

/// Handles all application messages and updates state accordingly.
///
/// This function processes user input, daemon commands, and detection results,
/// updating the overlay's visibility and hint state as needed.
fn update_impl(app: &mut OverlayApp, message: Message) -> Command<Message> {
    match message {
        Message::IcedEvent(event) => {
            // Handle keyboard events only when visible
            if !app.visible {
                return Command::none();
            }

            if let Event::Keyboard(iced::keyboard::Event::KeyPressed {
                ref key,
                ..
            }) = event
            {
                // Handle ESC key
                if matches!(key, iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape)) {
                    tracing::info!("ESC pressed, hiding overlay");
                    return update_impl(app, Message::CloseOverlay);
                }

                // Handle character input for hints
                if let iced::keyboard::Key::Character(ref c) = key {
                    if let Some(ch) = c.chars().next() {
                        return update_impl(app, Message::CharInput(ch));
                    }
                }

                // Handle backspace
                if matches!(key, iced::keyboard::Key::Named(iced::keyboard::key::Named::Backspace)) {
                    if !app.current_input.is_empty() {
                        app.current_input.pop();
                        tracing::debug!("Backspace - current input: '{}'", app.current_input);
                    }
                    return Command::none();
                }
            }
            tracing::debug!("Iced event: {:?}", event);
            Command::none()
        }
        Message::CharInput(ch) => {
            // Add character to current input
            app.current_input.push(ch);
            tracing::info!("Input: '{}' (matching {} hints)",
                app.current_input,
                app.hints.iter().filter(|h| h.matches(&app.current_input)).count()
            );

            // Check if any hint matches exactly
            if let Some(hint) = app.hints.iter().find(|h| h.is_exact_match(&app.current_input)) {
                tracing::info!("✓ Matched hint '{}' for element: {} at ({}, {})",
                    hint.label, hint.element.name, hint.element.bounds.x, hint.element.bounds.y);

                // TODO: Phase 5 - Trigger click action here using uinput
                // For now, just log and hide overlay
                tracing::warn!("Click action not yet implemented (Phase 5) - would click at ({}, {})",
                    hint.element.bounds.x, hint.element.bounds.y);

                // Hide overlay and keep daemon running
                return update_impl(app, Message::CloseOverlay);
            }

            Command::none()
        }
        Message::CloseOverlay => {
            tracing::info!("Hiding overlay (not exiting daemon)");
            app.visible = false;
            app.current_input.clear();
            app.hints.clear();
            app.detected_elements.clear();

            // The view will render empty when visible=false
            // Layer shell messages are auto-generated by the macro
            Command::none()
        }
        Message::ElementsDetected(elements) => {
            tracing::info!("Detected {} elements", elements.len());
            for element in &elements {
                tracing::debug!("  - {} at ({}, {})", element.name, element.bounds.x, element.bounds.y);
            }

            // Generate hints for detected elements
            let layout = KeyboardLayout::standard();
            app.hints = generate_hints_for_elements(elements.clone(), &layout);
            app.detected_elements = elements;
            app.current_input.clear();

            tracing::info!("Generated {} hints", app.hints.len());
            for hint in &app.hints {
                tracing::debug!("  - Hint '{}' -> {}", hint.label, hint.element.name);
            }

            Command::none()
        }
        Message::DetectionError(error) => {
            tracing::error!("Element detection failed: {}", error);
            Command::none()
        }
        Message::DaemonCommand(cmd) => {
            match cmd {
                DaemonCommand::Show => {
                    tracing::info!("Showing overlay");
                    app.visible = true;

                    // Trigger element detection
                    Command::perform(
                        async {
                            tokio::task::spawn(async {
                                detect_elements().await
                            }).await.unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {}", e)))
                        },
                        |result| match result {
                            Ok(elements) => Message::ElementsDetected(elements),
                            Err(e) => Message::DetectionError(e.to_string()),
                        }
                    )
                }
                DaemonCommand::Hide => {
                    update_impl(app, Message::CloseOverlay)
                }
                DaemonCommand::Toggle => {
                    if app.visible {
                        update_impl(app, Message::CloseOverlay)
                    } else {
                        update_impl(app, Message::DaemonCommand(DaemonCommand::Show))
                    }
                }
                DaemonCommand::Exit => {
                    tracing::info!("Exiting application");
                    std::process::exit(0);
                }
            }
        }
        _ => {
            // Handle generated layer shell messages
            Command::none()
        }
    }
}

/// Renders the overlay user interface.
///
/// Returns an empty view when hidden, or displays hints at their absolute screen
/// positions when visible. Highlights hints based on current user input.
fn view_impl(app: &OverlayApp) -> Element<'_, Message> {
    // CRITICAL: When not visible, return truly empty view with zero size
    // This prevents keyboard capture and allows input to pass through
    if !app.visible {
        return container(text(""))
            .width(Length::Fixed(0.0))
            .height(Length::Fixed(0.0))
            .into();
    }

    // When visible but no hints detected yet, show helpful message
    if app.hints.is_empty() {
        return container(
            text("No clickable elements detected. Press ESC to close.")
                .size(16)
                .color(Color::WHITE)
        )
        .padding(20)
        .style(|_theme| container::Style {
            text_color: Some(Color::WHITE),
            background: Some(Color::from_rgba(0.0, 0.0, 0.0, 0.7).into()),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        })
        .into();
    }

    // Use the absolute positioning widget to render hints at their actual screen positions
    absolute_hints(&app.hints, &app.current_input, &app.appearance)
}

/// Detects clickable elements on screen using AT-SPI accessibility tree traversal.
///
/// This async function creates an AT-SPI detector and scans the accessibility tree
/// for all clickable elements (buttons, links, etc.) across all visible applications.
///
/// # Errors
///
/// Returns an error if AT-SPI connection fails or element detection encounters issues.
async fn detect_elements() -> anyhow::Result<Vec<DetectedElement>> {
    tracing::info!("Starting element detection via AT-SPI");

    let detector = AtSpiDetector::new().await?;
    let elements = detector.detect_all_elements().await?;

    tracing::info!("Detection complete: found {} elements", elements.len());
    Ok(elements)
}
