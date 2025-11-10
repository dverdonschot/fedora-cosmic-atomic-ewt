//! Hint rendering at absolute positions on the overlay

use crate::hints::ElementHint;
use crate::overlay::styles::{HintAppearance, HintState};
use crate::overlay::widgets::{create_positioned_hints, PositionedHint};
use iced::advanced::layout::{self, Layout};
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::advanced::{self, Clipboard, Shell};
use iced::mouse;
use iced::{Element, Event, Length, Point, Rectangle, Size};

/// A widget that renders hints at absolute positions
pub struct HintOverlay<'a, Message> {
    hints: &'a [ElementHint],
    current_input: &'a str,
    appearance: HintAppearance,
    _phantom: std::marker::PhantomData<Message>,
}

impl<'a, Message> HintOverlay<'a, Message> {
    pub fn new(
        hints: &'a [ElementHint],
        current_input: &'a str,
        appearance: HintAppearance,
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
    for HintOverlay<'a, Message>
where
    Renderer: advanced::Renderer,
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
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        // Get the bounds of the overlay
        let bounds = layout.bounds();

        // Create positioned hints
        let positioned_hints = create_positioned_hints(self.hints, self.current_input);

        // For now, we'll just draw rectangles at hint positions
        // In a full implementation, we'd render the actual hint widgets
        for positioned in &positioned_hints {
            // Calculate hint position relative to overlay bounds
            let hint_x = positioned.x;
            let hint_y = positioned.y;

            // Skip hints outside the viewport
            if hint_x < bounds.x || hint_x > bounds.x + bounds.width
                || hint_y < bounds.y || hint_y > bounds.y + bounds.height
            {
                continue;
            }

            // Draw hint background (simplified for now)
            // In full implementation, this would render the actual widget
            let _hint_bounds = Rectangle {
                x: hint_x,
                y: hint_y,
                width: 40.0,  // Approximate width
                height: 30.0, // Approximate height
            };

            // Note: Actual rendering would be done using renderer.draw_quad or similar
            // This is a placeholder showing the structure
        }
    }

    fn on_event(
        &mut self,
        _state: &mut widget::Tree,
        event: Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> iced::event::Status {
        // Hints don't handle events directly - they're handled by the app
        iced::event::Status::Ignored
    }
}

impl<'a, Message, Theme, Renderer> From<HintOverlay<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: advanced::Renderer + 'a,
{
    fn from(overlay: HintOverlay<'a, Message>) -> Self {
        Self::new(overlay)
    }
}

/// Simple function to create hint widgets in a stack/overlay pattern
/// This is a simpler approach than the custom widget above
pub fn render_hints_simple<'a, Message: 'a>(
    hints: &'a [ElementHint],
    current_input: &'a str,
    appearance: &'a HintAppearance,
) -> Vec<PositionedHint> {
    create_positioned_hints(hints, current_input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detection::{DetectedElement, types::ElementBounds};
    use crate::hints::HintLabel;
    use atspi::Role;

    fn create_test_hint(label: &str, x: i32, y: i32) -> ElementHint {
        ElementHint::new(
            HintLabel::new(label.to_string()),
            DetectedElement::new(
                ElementBounds {
                    x,
                    y,
                    width: 100,
                    height: 30,
                },
                Role::Button,
                "Test".to_string(),
                "Test App".to_string(),
                "Test button".to_string(),
            ),
        )
    }

    #[test]
    fn test_hint_overlay_creation() {
        let hints = vec![create_test_hint("a", 100, 100)];
        let appearance = HintAppearance::default();

        let overlay = HintOverlay::<()>::new(&hints, "", appearance);

        assert_eq!(overlay.hints.len(), 1);
        assert_eq!(overlay.current_input, "");
    }

    #[test]
    fn test_render_hints_simple() {
        let hints = vec![
            create_test_hint("a", 100, 100),
            create_test_hint("s", 200, 200),
        ];
        let appearance = HintAppearance::default();

        let positioned = render_hints_simple::<()>(&hints, "", &appearance);

        assert_eq!(positioned.len(), 2);
        // Check positions (should be at element centers)
        assert_eq!(positioned[0].x, 150.0); // 100 + 100/2
        assert_eq!(positioned[0].y, 115.0); // 100 + 30/2
    }
}
