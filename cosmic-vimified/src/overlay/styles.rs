//! Visual styling for hint overlays

use iced::Color;
use serde::{Deserialize, Serialize};

/// Visual appearance configuration for hints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HintAppearance {
    /// Background color of hint labels
    pub bg_color: HintColor,
    /// Text color of hint labels
    pub text_color: HintColor,
    /// Background color for highlighted/matching hints
    pub highlight_bg_color: HintColor,
    /// Text color for highlighted/matching hints
    pub highlight_text_color: HintColor,
    /// Background color for dimmed/non-matching hints
    pub dimmed_bg_color: HintColor,
    /// Text color for dimmed/non-matching hints
    pub dimmed_text_color: HintColor,
    /// Font size for hint labels
    pub font_size: u16,
    /// Border radius in pixels
    pub border_radius: f32,
    /// Opacity (0.0 - 1.0)
    pub opacity: f32,
    /// Padding around hint text
    pub padding: f32,
}

impl Default for HintAppearance {
    fn default() -> Self {
        Self {
            // COSMIC blue accent color
            bg_color: HintColor::from_hex("#3daee9"),
            text_color: HintColor::from_hex("#ffffff"),
            // Brighter highlight for matching hints
            highlight_bg_color: HintColor::from_hex("#00ff00"),
            highlight_text_color: HintColor::from_hex("#000000"),
            // Dimmed colors for non-matching hints
            dimmed_bg_color: HintColor::from_hex("#666666"),
            dimmed_text_color: HintColor::from_hex("#cccccc"),
            font_size: 16,
            border_radius: 4.0,
            opacity: 0.95,
            padding: 8.0,
        }
    }
}

/// Color representation that can be serialized
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HintColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl HintColor {
    /// Create a new color from RGBA values (0.0 - 1.0)
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Create a color from RGB values (0.0 - 1.0) with full opacity
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self::new(r, g, b, 1.0)
    }

    /// Create a color from a hex string (e.g., "#3daee9" or "#3daee9ff")
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');

        let (r, g, b, a) = if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            (r, g, b, 255)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            (r, g, b, a)
        } else {
            (0, 0, 0, 255)
        };

        Self::new(
            r as f32 / 255.0,
            g as f32 / 255.0,
            b as f32 / 255.0,
            a as f32 / 255.0,
        )
    }

    /// Convert to iced Color
    pub fn to_iced(&self) -> Color {
        Color::from_rgba(self.r, self.g, self.b, self.a)
    }

    /// Apply opacity multiplier
    pub fn with_alpha(&self, alpha: f32) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a * alpha,
        }
    }
}

impl From<HintColor> for Color {
    fn from(c: HintColor) -> Self {
        c.to_iced()
    }
}

/// State of a hint for styling purposes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintState {
    /// Normal state - hint is visible and selectable
    Normal,
    /// Highlighted - hint matches current input
    Highlighted,
    /// Dimmed - hint doesn't match current input
    Dimmed,
}

impl HintAppearance {
    /// Get the background color for a given hint state
    pub fn bg_color_for_state(&self, state: HintState) -> Color {
        let color = match state {
            HintState::Normal => self.bg_color,
            HintState::Highlighted => self.highlight_bg_color,
            HintState::Dimmed => self.dimmed_bg_color,
        };
        color.with_alpha(self.opacity).to_iced()
    }

    /// Get the text color for a given hint state
    pub fn text_color_for_state(&self, state: HintState) -> Color {
        match state {
            HintState::Normal => self.text_color.to_iced(),
            HintState::Highlighted => self.highlight_text_color.to_iced(),
            HintState::Dimmed => self.dimmed_text_color.to_iced(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hint_color_from_hex() {
        let color = HintColor::from_hex("#3daee9");
        assert!((color.r - 0.239).abs() < 0.01); // 0x3d / 255
        assert!((color.g - 0.682).abs() < 0.01); // 0xae / 255
        assert!((color.b - 0.913).abs() < 0.01); // 0xe9 / 255
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_hint_color_from_hex_with_alpha() {
        let color = HintColor::from_hex("#3daee980");
        assert!((color.r - 0.239).abs() < 0.01);
        assert!((color.a - 0.502).abs() < 0.01); // 0x80 / 255
    }

    #[test]
    fn test_hint_color_with_alpha() {
        let color = HintColor::rgb(1.0, 0.5, 0.25);
        let dimmed = color.with_alpha(0.5);
        assert_eq!(dimmed.r, 1.0);
        assert_eq!(dimmed.g, 0.5);
        assert_eq!(dimmed.b, 0.25);
        assert_eq!(dimmed.a, 0.5);
    }

    #[test]
    fn test_default_appearance() {
        let appearance = HintAppearance::default();
        assert_eq!(appearance.font_size, 16);
        assert_eq!(appearance.border_radius, 4.0);
        assert_eq!(appearance.opacity, 0.95);
    }

    #[test]
    fn test_color_for_state() {
        let appearance = HintAppearance::default();

        let normal = appearance.bg_color_for_state(HintState::Normal);
        let highlighted = appearance.bg_color_for_state(HintState::Highlighted);
        let dimmed = appearance.bg_color_for_state(HintState::Dimmed);

        // Colors should be different for different states
        assert_ne!(normal, highlighted);
        assert_ne!(normal, dimmed);
        assert_ne!(highlighted, dimmed);
    }
}
