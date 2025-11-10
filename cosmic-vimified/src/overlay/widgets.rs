//! Custom widgets for hint rendering

use crate::hints::ElementHint;
use crate::overlay::styles::{HintAppearance, HintState};
use iced::widget::{container, text, Container};
use iced::{Border, Color, Element, Length, Shadow};

/// Create a hint widget positioned absolutely
pub fn hint_widget<'a, Message: 'a>(
    hint: &'a ElementHint,
    state: HintState,
    appearance: &'a HintAppearance,
) -> Element<'a, Message> {
    let label_text = text(&hint.label.text)
        .size(appearance.font_size)
        .color(appearance.text_color_for_state(state));

    let hint_container = container(label_text)
        .padding(appearance.padding)
        .style(move |_theme| {
            let bg_color = appearance.bg_color_for_state(state);
            container::Style {
                text_color: Some(appearance.text_color_for_state(state)),
                background: Some(bg_color.into()),
                border: Border {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
                    width: 1.0,
                    radius: appearance.border_radius.into(),
                },
                shadow: Shadow {
                    color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
                    offset: iced::Vector::new(2.0, 2.0),
                    blur_radius: 4.0,
                },
            }
        });

    hint_container.into()
}

/// Create a positioned hint widget using absolute positioning
/// This will be used with a canvas or positioned container
pub struct PositionedHint {
    pub x: f32,
    pub y: f32,
    pub hint: ElementHint,
    pub state: HintState,
}

impl PositionedHint {
    pub fn new(hint: ElementHint, state: HintState) -> Self {
        // Calculate position from element bounds (center of element)
        let (center_x, center_y) = hint.element.center();

        Self {
            x: center_x as f32,
            y: center_y as f32,
            hint,
            state,
        }
    }

    /// Create a widget for this positioned hint
    pub fn widget<'a, Message: 'a>(
        &'a self,
        appearance: &'a HintAppearance,
    ) -> Element<'a, Message> {
        hint_widget(&self.hint, self.state, appearance)
    }
}

/// Create a list of positioned hints from element hints
pub fn create_positioned_hints(
    hints: &[ElementHint],
    current_input: &str,
) -> Vec<PositionedHint> {
    hints
        .iter()
        .map(|hint| {
            let state = if current_input.is_empty() {
                HintState::Normal
            } else if hint.matches(current_input) {
                HintState::Highlighted
            } else {
                HintState::Dimmed
            };

            PositionedHint::new(hint.clone(), state)
        })
        .collect()
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
    fn test_positioned_hint_creation() {
        let hint = create_test_hint("as", 100, 200);
        let positioned = PositionedHint::new(hint, HintState::Normal);

        // Should be centered on element (100 + 100/2, 200 + 30/2)
        assert_eq!(positioned.x, 150.0);
        assert_eq!(positioned.y, 215.0);
    }

    #[test]
    fn test_create_positioned_hints_no_input() {
        let hints = vec![
            create_test_hint("a", 100, 100),
            create_test_hint("s", 200, 200),
        ];

        let positioned = create_positioned_hints(&hints, "");

        assert_eq!(positioned.len(), 2);
        assert_eq!(positioned[0].state, HintState::Normal);
        assert_eq!(positioned[1].state, HintState::Normal);
    }

    #[test]
    fn test_create_positioned_hints_with_input() {
        let hints = vec![
            create_test_hint("as", 100, 100),
            create_test_hint("ad", 200, 200),
            create_test_hint("sd", 300, 300),
        ];

        let positioned = create_positioned_hints(&hints, "a");

        assert_eq!(positioned.len(), 3);
        // "as" and "ad" start with "a", so should be highlighted
        assert_eq!(positioned[0].state, HintState::Highlighted);
        assert_eq!(positioned[1].state, HintState::Highlighted);
        // "sd" doesn't start with "a", so should be dimmed
        assert_eq!(positioned[2].state, HintState::Dimmed);
    }

    #[test]
    fn test_create_positioned_hints_exact_match() {
        let hints = vec![
            create_test_hint("as", 100, 100),
            create_test_hint("ad", 200, 200),
        ];

        let positioned = create_positioned_hints(&hints, "as");

        // "as" matches exactly (highlighted)
        assert_eq!(positioned[0].state, HintState::Highlighted);
        // "ad" doesn't match (dimmed)
        assert_eq!(positioned[1].state, HintState::Dimmed);
    }
}
