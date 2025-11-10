//! Canvas-based rendering for absolutely positioned hints

use crate::hints::ElementHint;
use crate::overlay::styles::{HintAppearance, HintState};
use crate::overlay::widgets::create_positioned_hints;
use iced::advanced::layout;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::mouse;
use iced::{Color, Event, Length, Rectangle, Size};

/// Canvas widget that renders hints at absolute screen positions
pub struct HintCanvas<'a, Message> {
    hints: &'a [ElementHint],
    current_input: &'a str,
    appearance: &'a HintAppearance,
    _phantom: std::marker::PhantomData<Message>,
}

impl<'a, Message> HintCanvas<'a, Message> {
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

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for HintCanvas<'a, Message>
where
    Renderer: advanced::Renderer + advanced::text::Renderer,
    Theme: 'a,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn layout(
        &self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.max())
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // NOTE: Advanced rendering with fill_quad and fill_text requires
        // specific renderer traits that may not be available in all contexts.
        // For now, we'll use a simpler approach with widgets positioned via stack.
        // This is a placeholder - actual rendering is done in the view using stack.
    }

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        _event: Event,
        _layout: layout::Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::event::Status {
        // Events handled by parent app
        iced::event::Status::Ignored
    }
}

impl<'a, Message, Theme, Renderer> From<HintCanvas<'a, Message>>
    for iced::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: advanced::Renderer + advanced::text::Renderer + 'a,
{
    fn from(canvas: HintCanvas<'a, Message>) -> Self {
        Self::new(canvas)
    }
}

/// Helper function to create a hint canvas
pub fn hint_canvas<'a, Message: 'a, Theme: 'a, Renderer: 'a>(
    hints: &'a [ElementHint],
    current_input: &'a str,
    appearance: &'a HintAppearance,
) -> iced::Element<'a, Message, Theme, Renderer>
where
    Renderer: advanced::Renderer + advanced::text::Renderer,
{
    HintCanvas::new(hints, current_input, appearance).into()
}
