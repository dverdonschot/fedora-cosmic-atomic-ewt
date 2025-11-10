//! Absolutely positioned hint widgets using custom layout

use crate::hints::ElementHint;
use crate::overlay::styles::HintAppearance;
use crate::overlay::widgets::create_positioned_hints;
use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::mouse;
use iced::{Color, Element, Event, Length, Point, Rectangle, Size, Vector};

/// A widget that positions hint labels at absolute screen coordinates
pub struct AbsoluteHints<'a, Message> {
    hints: &'a [ElementHint],
    current_input: &'a str,
    appearance: &'a HintAppearance,
    _phantom: std::marker::PhantomData<Message>,
}

impl<'a, Message> AbsoluteHints<'a, Message> {
    pub fn new(
        hints: &'a [ElementHint],
        current_input: &'a str,
        appearance: &'a HintAppearance,
    ) -> Self {
        Self {
            hints,
            current_input,
            appearance,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AbsoluteHints<'a, Message>
where
    Renderer: advanced::Renderer + iced::advanced::text::Renderer,
    Theme: 'a,
    Message: 'a,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        // Take up the full screen
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::graphics::core::text::{LineHeight, Shaping};
        use iced::advanced::text::{Paragraph, Text};

        let bounds = layout.bounds();
        let positioned_hints = create_positioned_hints(self.hints, self.current_input);

        // Draw each hint at its absolute position
        for positioned in &positioned_hints {
            let hint_x = positioned.x;
            let hint_y = positioned.y;

            // Skip hints outside viewport
            if hint_x < 0.0
                || hint_y < 0.0
                || hint_x > viewport.width
                || hint_y > viewport.height
            {
                continue;
            }

            // Get colors based on state
            let bg_color = self.appearance.bg_color_for_state(positioned.state);
            let text_color = self.appearance.text_color_for_state(positioned.state);

            // Calculate hint size
            let label = &positioned.hint.label.text;
            let char_count = label.len() as f32;
            let hint_width = (char_count * 12.0).max(24.0);
            let hint_height = self.appearance.font_size as f32 + self.appearance.padding * 2.0;

            // Position hint centered on element
            let hint_x = hint_x - hint_width / 2.0;
            let hint_y = hint_y - hint_height / 2.0;

            // Draw background
            let hint_bounds = Rectangle {
                x: hint_x,
                y: hint_y,
                width: hint_width,
                height: hint_height,
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds: hint_bounds,
                    border: iced::Border {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.4),
                        width: 2.0,
                        radius: self.appearance.border_radius.into(),
                    },
                    shadow: iced::Shadow {
                        color: Color::from_rgba(0.0, 0.0, 0.0, 0.8),
                        offset: Vector::new(2.0, 2.0),
                        blur_radius: 6.0,
                    },
                },
                bg_color,
            );

            // Draw text
            let text_bounds = Rectangle {
                x: hint_x + self.appearance.padding,
                y: hint_y + self.appearance.padding / 2.0,
                width: hint_width - self.appearance.padding * 2.0,
                height: hint_height,
            };

            renderer.fill_text(
                iced::advanced::Text {
                    content: label.to_string(),
                    bounds: text_bounds.size(),
                    size: iced::Pixels(self.appearance.font_size as f32),
                    font: renderer.default_font(),
                    horizontal_alignment: iced::alignment::Horizontal::Center,
                    vertical_alignment: iced::alignment::Vertical::Center,
                    line_height: LineHeight::default(),
                    shaping: Shaping::default(),
                    wrapping: iced::advanced::text::Wrapping::default(),
                },
                Point::new(text_bounds.x, text_bounds.y),
                text_color,
                text_bounds,
            );
        }
    }

    fn on_event(
        &mut self,
        _state: &mut Tree,
        _event: Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::event::Status {
        // Events are handled by the parent app
        iced::event::Status::Ignored
    }
}

impl<'a, Message, Theme, Renderer> From<AbsoluteHints<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: advanced::Renderer + iced::advanced::text::Renderer + 'a,
{
    fn from(hints: AbsoluteHints<'a, Message>) -> Self {
        Element::new(hints)
    }
}

/// Helper function to create absolutely positioned hints
pub fn absolute_hints<'a, Message: 'a, Theme: 'a, Renderer: 'a>(
    hints: &'a [ElementHint],
    current_input: &'a str,
    appearance: &'a HintAppearance,
) -> Element<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer + iced::advanced::text::Renderer,
{
    AbsoluteHints::new(hints, current_input, appearance).into()
}
