//! Hint generation and management
//!
//! This module provides the hint generation algorithm that creates optimal
//! hint labels for detected elements. The algorithm uses keyboard layout
//! configurations to generate hints using home-row keys for maximum efficiency.

pub mod generator;
pub mod layout;

pub use generator::{HintGenerator, HintLabel};
pub use layout::{KeyboardLayout, LayoutMode};

use crate::detection::DetectedElement;
use serde::{Deserialize, Serialize};

/// A hint associated with a detected element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementHint {
    pub label: HintLabel,
    pub element: DetectedElement,
}

impl ElementHint {
    pub fn new(label: HintLabel, element: DetectedElement) -> Self {
        Self { label, element }
    }

    /// Check if this hint matches the given input
    pub fn matches(&self, input: &str) -> bool {
        self.label.matches(input)
    }

    /// Check if this hint is an exact match for the input
    pub fn is_exact_match(&self, input: &str) -> bool {
        self.label.is_exact_match(input)
    }
}

/// Generate hints for a list of detected elements
pub fn generate_hints_for_elements(
    elements: Vec<DetectedElement>,
    layout: &KeyboardLayout,
) -> Vec<ElementHint> {
    let generator = HintGenerator::new(layout.clone());
    let labels = generator.generate(elements.len());

    elements
        .into_iter()
        .zip(labels.into_iter())
        .map(|(element, label)| ElementHint::new(label, element))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detection::DetectedElement;
    use crate::detection::types::ElementBounds;
    use atspi::Role;

    fn create_test_element(x: i32, y: i32) -> DetectedElement {
        DetectedElement::new(
            ElementBounds {
                x,
                y,
                width: 100,
                height: 30,
            },
            Role::Button,
            "Test Button".to_string(),
            "Test App".to_string(),
            "A test button".to_string(),
        )
    }

    #[test]
    fn test_generate_hints_for_elements() {
        let elements = vec![
            create_test_element(100, 100),
            create_test_element(200, 200),
            create_test_element(300, 300),
        ];

        let layout = KeyboardLayout::standard();
        let hints = generate_hints_for_elements(elements, &layout);

        assert_eq!(hints.len(), 3);
        assert_eq!(hints[0].label.text, "a");
        assert_eq!(hints[1].label.text, "s");
        assert_eq!(hints[2].label.text, "d");
    }

    #[test]
    fn test_element_hint_matches() {
        let element = create_test_element(100, 100);
        let label = HintLabel::new("as".to_string());
        let hint = ElementHint::new(label, element);

        assert!(hint.matches(""));
        assert!(hint.matches("a"));
        assert!(hint.matches("as"));
        assert!(!hint.matches("s"));
    }

    #[test]
    fn test_element_hint_exact_match() {
        let element = create_test_element(100, 100);
        let label = HintLabel::new("as".to_string());
        let hint = ElementHint::new(label, element);

        assert!(hint.is_exact_match("as"));
        assert!(!hint.is_exact_match("a"));
        assert!(!hint.is_exact_match("asd"));
    }
}
