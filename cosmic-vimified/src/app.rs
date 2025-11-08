use crate::detection::{atspi::AtSpiDetector, DetectedElement};
use crate::commands::DaemonCommand;
use crate::overlay::input;
use cosmic::app::{Core, Task};
use cosmic::iced::event::{self, Event};
use cosmic::iced::keyboard::{Event as KeyEvent, Key, Modifiers};
use cosmic::iced::{window, Alignment, Subscription};
use cosmic::iced::futures;
use cosmic::{executor, Application, Element};
use tokio::sync::{mpsc, Mutex};
use std::sync::Arc;
use anyhow;

#[derive(Debug, Clone)]
pub enum Message {
    CloseOverlay,
    KeyPressed(Key, Modifiers),
    DaemonCommand(DaemonCommand),
    WindowOpened(window::Id),
    ElementsDetected(Vec<DetectedElement>),
    DetectionError(String),
}

pub struct App {
    core: Core,
    overlay_active: bool,
    window_id: Option<window::Id>,
    command_rx: Arc<Mutex<mpsc::UnboundedReceiver<DaemonCommand>>>,
    detected_elements: Vec<DetectedElement>,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = mpsc::UnboundedReceiver<DaemonCommand>;
    type Message = Message;
    const APP_ID: &'static str = "com.cosmic.Vimified";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, command_rx: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = App {
            core,
            overlay_active: false, // Start hidden
            window_id: None,
            command_rx: Arc::new(Mutex::new(command_rx)),
            detected_elements: Vec::new(),
        };

        // Start with no window - window will be created on "show" command
        (app, Task::none())
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::CloseOverlay => {
                tracing::info!("Hiding overlay");
                self.overlay_active = false;

                // Close the window if it exists
                if let Some(id) = self.window_id.take() {
                    return window::close(id);
                }
                Task::none()
            }
            Message::KeyPressed(key, _modifiers) => {
                tracing::debug!("Key pressed: {:?}", key);
                if input::is_escape_key(&key) {
                    return self.update(Message::CloseOverlay);
                }
                Task::none()
            }
            Message::WindowOpened(id) => {
                tracing::info!("Window opened with ID: {:?}", id);

                // Now that window is created, trigger element detection
                // Spawn in tokio runtime since AT-SPI needs tokio
                Task::perform(
                    async {
                        tokio::task::spawn(async {
                            detect_elements().await
                        }).await.unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {}", e)))
                    },
                    |result| match result {
                        Ok(elements) => cosmic::Action::App(Message::ElementsDetected(elements)),
                        Err(e) => cosmic::Action::App(Message::DetectionError(e.to_string())),
                    }
                )
            }
            Message::DaemonCommand(cmd) => {
                match cmd {
                    DaemonCommand::Show => {
                        tracing::info!("Showing overlay");
                        self.overlay_active = true;

                        // Create a new window if not already active
                        if self.window_id.is_none() {
                            let (id, task) = window::open(window::Settings {
                                size: cosmic::iced::Size::new(1920.0, 1080.0),
                                position: window::Position::Centered,
                                transparent: true,
                                decorations: false,
                                resizable: false,
                                level: window::Level::AlwaysOnTop,
                                exit_on_close_request: false,
                                ..Default::default()
                            });
                            self.window_id = Some(id);
                            return task.map(move |_| cosmic::Action::App(Message::WindowOpened(id)));
                        }

                        // If window already exists, trigger element detection
                        // Spawn in tokio runtime since AT-SPI needs tokio
                        Task::perform(
                            async {
                                tokio::task::spawn(async {
                                    detect_elements().await
                                }).await.unwrap_or_else(|e| Err(anyhow::anyhow!("Task join error: {}", e)))
                            },
                            |result| match result {
                                Ok(elements) => cosmic::Action::App(Message::ElementsDetected(elements)),
                                Err(e) => cosmic::Action::App(Message::DetectionError(e.to_string())),
                            }
                        )
                    }
                    DaemonCommand::Hide => {
                        self.update(Message::CloseOverlay)
                    }
                    DaemonCommand::Toggle => {
                        if self.overlay_active {
                            self.update(Message::CloseOverlay)
                        } else {
                            self.update(Message::DaemonCommand(DaemonCommand::Show))
                        }
                    }
                    DaemonCommand::Exit => {
                        tracing::info!("Exiting application");
                        window::get_latest().and_then(|id| window::close(id))
                    }
                }
            }
            Message::ElementsDetected(elements) => {
                tracing::info!("Detected {} elements", elements.len());
                for element in &elements {
                    tracing::debug!("  {}", element);
                }
                self.detected_elements = elements;
                Task::none()
            }
            Message::DetectionError(error) => {
                tracing::error!("Element detection failed: {}", error);
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        // Main window view - since we use no_main_window, this is rarely called
        cosmic::widget::container(cosmic::widget::Space::new(0, 0)).into()
    }

    fn view_window(&self, _id: window::Id) -> Element<Self::Message> {
        // View for all spawned windows (our overlay)
        if !self.overlay_active {
            return cosmic::widget::container(cosmic::widget::Space::new(0, 0)).into();
        }

        let status_text = format!(
            "COSMIC Vimified - Daemon Mode\nOverlay Active\nPress ESC to hide\n\nDetected {} elements",
            self.detected_elements.len()
        );

        cosmic::widget::container(cosmic::widget::text(status_text))
            .width(cosmic::iced::Length::Fill)
            .height(cosmic::iced::Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let keyboard_sub = event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(KeyEvent::KeyPressed {
                key,
                modifiers,
                ..
            }) => Some(Message::KeyPressed(key, modifiers)),
            _ => None,
        });

        let command_rx = Arc::clone(&self.command_rx);
        let command_sub = Subscription::run_with_id(
            "daemon-commands",
            command_stream(command_rx)
        ).map(Message::DaemonCommand);

        Subscription::batch(vec![keyboard_sub, command_sub])
    }
}

// Stream for receiving daemon commands
fn command_stream(
    command_rx: Arc<Mutex<mpsc::UnboundedReceiver<DaemonCommand>>>
) -> impl futures::Stream<Item = DaemonCommand> {
    futures::stream::unfold(command_rx, |rx| async move {
        let cmd = {
            let mut receiver = rx.lock().await;
            receiver.recv().await
        };
        cmd.map(|c| (c, rx))
    })
}

async fn detect_elements() -> anyhow::Result<Vec<DetectedElement>> {
    let detector = AtSpiDetector::new().await?;
    detector.detect_all_elements().await
}
