use serde::{Deserialize, Serialize};

/// Keyboard layout mode for hint generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LayoutMode {
    /// Standard layout using all home-row keys (asdfjkl;)
    Standard,
    /// Left-handed layout using only left-side keys (asdfg)
    LeftHanded,
    /// Right-handed layout using only right-side keys (jkl;)
    RightHanded,
    /// Custom layout with user-defined characters
    Custom,
}

impl Default for LayoutMode {
    fn default() -> Self {
        Self::Standard
    }
}

/// Keyboard layout configuration for hint generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardLayout {
    pub mode: LayoutMode,
    pub custom_chars: Option<Vec<char>>,
}

impl Default for KeyboardLayout {
    fn default() -> Self {
        Self {
            mode: LayoutMode::Standard,
            custom_chars: None,
        }
    }
}

impl KeyboardLayout {
    /// Get the character sequence for this layout
    pub fn chars(&self) -> Vec<char> {
        match self.mode {
            LayoutMode::Standard => vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';'],
            LayoutMode::LeftHanded => vec!['a', 's', 'd', 'f', 'g'],
            LayoutMode::RightHanded => vec!['h', 'j', 'k', 'l', ';'],
            LayoutMode::Custom => self
                .custom_chars
                .clone()
                .unwrap_or_else(|| vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';']),
        }
    }

    /// Create a new standard layout
    pub fn standard() -> Self {
        Self {
            mode: LayoutMode::Standard,
            custom_chars: None,
        }
    }

    /// Create a new left-handed layout
    pub fn left_handed() -> Self {
        Self {
            mode: LayoutMode::LeftHanded,
            custom_chars: None,
        }
    }

    /// Create a new right-handed layout
    pub fn right_handed() -> Self {
        Self {
            mode: LayoutMode::RightHanded,
            custom_chars: None,
        }
    }

    /// Create a custom layout with specific characters
    pub fn custom(chars: Vec<char>) -> Self {
        Self {
            mode: LayoutMode::Custom,
            custom_chars: Some(chars),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_layout() {
        let layout = KeyboardLayout::standard();
        assert_eq!(layout.chars(), vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';']);
    }

    #[test]
    fn test_left_handed_layout() {
        let layout = KeyboardLayout::left_handed();
        assert_eq!(layout.chars(), vec!['a', 's', 'd', 'f', 'g']);
    }

    #[test]
    fn test_right_handed_layout() {
        let layout = KeyboardLayout::right_handed();
        assert_eq!(layout.chars(), vec!['h', 'j', 'k', 'l', ';']);
    }

    #[test]
    fn test_custom_layout() {
        let custom_chars = vec!['q', 'w', 'e', 'r'];
        let layout = KeyboardLayout::custom(custom_chars.clone());
        assert_eq!(layout.chars(), custom_chars);
    }

    #[test]
    fn test_default_layout() {
        let layout = KeyboardLayout::default();
        assert_eq!(layout.mode, LayoutMode::Standard);
        assert_eq!(layout.chars(), vec!['a', 's', 'd', 'f', 'j', 'k', 'l', ';']);
    }
}
