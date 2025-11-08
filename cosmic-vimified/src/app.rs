use crate::detection::{atspi::AtSpiDetector, DetectedElement};
use crate::overlay::input;
use cosmic::app::{Core, Task};
use cosmic::iced::event::{self, Event};
use cosmic::iced::keyboard::{Event as KeyEvent, Key, Modifiers};
use cosmic::iced::{window, Alignment, Subscription};
use cosmic::{executor, Application, Element};

#[derive(Debug, Clone)]
pub enum Message {
    CloseOverlay,
    KeyPressed(Key, Modifiers),
    ElementsDetected(Vec<DetectedElement>),
    DetectionError(String),
    Ignore,
}

pub struct App {
    core: Core,
    overlay_active: bool,
    detected_elements: Vec<DetectedElement>,
}

impl Application for App {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "com.cosmic.Vimified";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        let app = App {
            core,
            overlay_active: true,
            detected_elements: Vec::new(),
        };

        let task = Task::perform(detect_elements(), |result| match result {
            Ok(elements) => cosmic::Action::App(Message::ElementsDetected(elements)),
            Err(e) => cosmic::Action::App(Message::DetectionError(e.to_string())),
        });

        (app, task)
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        match message {
            Message::CloseOverlay => {
                tracing::info!("Closing overlay");
                self.overlay_active = false;
                window::get_latest().and_then(|id| window::close(id))
            }
            Message::KeyPressed(key, _modifiers) => {
                tracing::debug!("Key pressed: {:?}", key);
                if input::is_escape_key(&key) {
                    return self.update(Message::CloseOverlay);
                }
                Task::none()
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
            Message::Ignore => Task::none(),
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let status_text = format!(
            "COSMIC Vimified - Overlay Active\nDetected {} clickable elements\nPress ESC to close",
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
        event::listen_with(|event, _status, _id| match event {
            Event::Keyboard(KeyEvent::KeyPressed {
                key,
                modifiers,
                ..
            }) => Some(Message::KeyPressed(key, modifiers)),
            _ => None,
        })
    }
}

async fn detect_elements() -> anyhow::Result<Vec<DetectedElement>> {
    let detector = AtSpiDetector::new().await?;
    detector.detect_all_elements().await
}
