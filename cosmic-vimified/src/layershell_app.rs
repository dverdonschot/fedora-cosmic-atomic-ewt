use crate::commands::DaemonCommand;
use crate::detection::{atspi::AtSpiDetector, DetectedElement};
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Color, Element, Event, Length, Task as Command, event};
use iced::futures;
use iced::Subscription;
use iced_layershell::Application;
use iced_layershell::settings::Settings;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, StartMode};
use iced_layershell::to_layer_message;
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use futures::stream::StreamExt;

// Global static for command receiver (not ideal, but works with build_pattern)
static COMMAND_RX: once_cell::sync::OnceCell<Arc<Mutex<mpsc::UnboundedReceiver<DaemonCommand>>>> =
    once_cell::sync::OnceCell::new();

pub struct OverlayApp {
    visible: bool,
    detected_elements: Vec<DetectedElement>,
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
}

impl Application for OverlayApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        // Start visible and trigger immediate element detection
        (
            OverlayApp {
                visible: true,
                detected_elements: Vec::new(),
            },
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
            ),
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

pub fn run(command_rx: mpsc::UnboundedReceiver<DaemonCommand>) -> Result<(), iced_layershell::Error> {
    // Store command receiver in global static
    COMMAND_RX.set(Arc::new(Mutex::new(command_rx)))
        .expect("COMMAND_RX already initialized");

    OverlayApp::run(Settings {
        layer_settings: LayerShellSettings {
            // Full screen transparent overlay (hidden initially)
            size: Some((0, 0)), // Start with 0 size (hidden)
            exclusive_zone: 0, // Don't reserve space
            anchor: Anchor::Top | Anchor::Bottom | Anchor::Left | Anchor::Right,
            start_mode: StartMode::Active,
            // Keep keyboard interactivity to capture ESC even when hidden
            // This should be safe since the window is 0x0 when hidden
            keyboard_interactivity: iced_layershell::reexport::KeyboardInteractivity::Exclusive,
            // CRITICAL: Allow pointer events to pass through to apps below
            events_transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}

/// Run a single overlay instance (called on-demand when user presses Super+G)
/// This creates a self-contained overlay that closes on ESC
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

fn subscription_impl(_app: &OverlayApp) -> iced::Subscription<Message> {
    // Just subscribe to iced events for keyboard input
    event::listen().map(Message::IcedEvent)
}

// Custom subscription to receive daemon commands
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
                if matches!(key, iced::keyboard::Key::Named(iced::keyboard::key::Named::Escape)) {
                    tracing::info!("ESC pressed, hiding overlay");
                    return update_impl(app, Message::CloseOverlay);
                }
            }
            tracing::debug!("Iced event: {:?}", event);
            Command::none()
        }
        Message::CloseOverlay => {
            tracing::info!("Closing overlay window");
            app.visible = false;
            // Exit the application - this will close the window and end the overlay instance
            std::process::exit(0);
        }
        Message::ElementsDetected(elements) => {
            tracing::info!("Detected {} elements", elements.len());
            for element in &elements {
                tracing::debug!("  - {} at ({}, {})", element.name, element.bounds.x, element.bounds.y);
            }
            app.detected_elements = elements;
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

                    // Show the window by making it full screen
                    Command::batch(vec![
                        // Use a very large size to cover the screen
                        // The anchor settings will make it cover the full screen
                        Command::done(Message::SizeChange((0, 0))), // With all anchors, 0,0 means full screen
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
                        ),
                    ])
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

fn view_impl(app: &OverlayApp) -> Element<'_, Message> {
    if !app.visible {
        // Return empty/transparent view when not visible
        return container(text(""))
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
    }

    let status_text = format!(
        "COSMIC Vimified - Overlay Active\nDetected {} elements\nPress ESC to hide",
        app.detected_elements.len()
    );

    container(
        column![
            text(status_text).size(24),
            text("Hint rendering will go here...").size(16)
        ]
        .spacing(20)
        .align_x(Alignment::Center)
        .padding(20)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}

// Element detection function (adapted from app.rs)
async fn detect_elements() -> anyhow::Result<Vec<DetectedElement>> {
    tracing::info!("Starting element detection via AT-SPI");

    let detector = AtSpiDetector::new().await?;
    let elements = detector.detect_all_elements().await?;

    tracing::info!("Detection complete: found {} elements", elements.len());
    Ok(elements)
}
