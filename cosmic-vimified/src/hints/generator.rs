use super::layout::KeyboardLayout;
use serde::{Deserialize, Serialize};

/// A hint label (e.g., "a", "as", "asd")
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HintLabel {
    pub text: String,
}

impl HintLabel {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    /// Check if this hint matches the given input
    pub fn matches(&self, input: &str) -> bool {
        self.text.starts_with(input)
    }

    /// Check if this hint is an exact match for the input
    pub fn is_exact_match(&self, input: &str) -> bool {
        self.text == input
    }
}

impl std::fmt::Display for HintLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

/// Hint generator that creates optimal hint labels
pub struct HintGenerator {
    layout: KeyboardLayout,
}

impl HintGenerator {
    /// Create a new hint generator with the given keyboard layout
    pub fn new(layout: KeyboardLayout) -> Self {
        Self { layout }
    }

    /// Generate hint labels for the given number of elements
    ///
    /// The algorithm generates hints in the following order:
    /// 1. Single characters (a, s, d, f, j, k, l, ;)
    /// 2. Two-character combinations (aa, as, ad, ..., ;;)
    /// 3. Three-character combinations (aaa, aas, ..., ;;;)
    /// And so on until we have enough hints for all elements.
    pub fn generate(&self, count: usize) -> Vec<HintLabel> {
        if count == 0 {
            return Vec::new();
        }

        let chars = self.layout.chars();
        if chars.is_empty() {
            return Vec::new();
        }

        let mut hints = Vec::with_capacity(count);
        let base = chars.len();

        // Generate hints until we have enough
        let mut index = 0;
        while hints.len() < count {
            hints.push(HintLabel::new(self.index_to_hint(index, &chars, base)));
            index += 1;
        }

        hints
    }

    /// Convert an index to a hint string
    ///
    /// This uses a base-N numeral system where N is the number of characters.
    /// Index 0 -> "a", 1 -> "s", 2 -> "d", ..., 7 -> ";", 8 -> "aa", 9 -> "as", etc.
    fn index_to_hint(&self, mut index: usize, chars: &[char], base: usize) -> String {
        let mut result = String::new();

        // Single character hints (0 to base-1)
        if index < base {
            result.push(chars[index]);
            return result;
        }

        // Multi-character hints
        // Adjust index to account for single-character hints
        index -= base;

        // Determine the length of the hint
        let mut length = 2;
        let mut capacity = base.pow(2);
        let mut accumulated = 0;

        while accumulated + capacity <= index {
            accumulated += capacity;
            length += 1;
            capacity = base.pow(length as u32);
        }

        // Get the position within hints of this length
        let position = index - accumulated;

        // Convert position to base-N representation
        let mut pos = position;
        let mut chars_vec = Vec::with_capacity(length);

        for _ in 0..length {
            chars_vec.push(chars[pos % base]);
            pos /= base;
        }

        // Reverse because we built it backwards
        chars_vec.reverse();

        chars_vec.into_iter().collect()
    }
}

impl Default for HintGenerator {
    fn default() -> Self {
        Self::new(KeyboardLayout::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hints::layout::LayoutMode;

    #[test]
    fn test_generate_zero_hints() {
        let generator = HintGenerator::default();
        let hints = generator.generate(0);
        assert_eq!(hints.len(), 0);
    }

    #[test]
    fn test_generate_single_hints() {
        let generator = HintGenerator::default();
        let hints = generator.generate(8);

        assert_eq!(hints.len(), 8);
        assert_eq!(hints[0].text, "a");
        assert_eq!(hints[1].text, "s");
        assert_eq!(hints[2].text, "d");
        assert_eq!(hints[3].text, "f");
        assert_eq!(hints[4].text, "j");
        assert_eq!(hints[5].text, "k");
        assert_eq!(hints[6].text, "l");
        assert_eq!(hints[7].text, ";");
    }

    #[test]
    fn test_generate_two_char_hints() {
        let generator = HintGenerator::default();
        let hints = generator.generate(10);

        assert_eq!(hints.len(), 10);
        assert_eq!(hints[8].text, "aa");
        assert_eq!(hints[9].text, "as");
    }

    #[test]
    fn test_generate_many_hints() {
        let generator = HintGenerator::default();
        let hints = generator.generate(100);

        assert_eq!(hints.len(), 100);

        // All hints should be unique
        let unique_hints: std::collections::HashSet<_> = hints.iter().map(|h| &h.text).collect();
        assert_eq!(unique_hints.len(), 100);
    }

    #[test]
    fn test_hint_label_matches() {
        let hint = HintLabel::new("as".to_string());

        assert!(hint.matches(""));
        assert!(hint.matches("a"));
        assert!(hint.matches("as"));
        assert!(!hint.matches("s"));
        assert!(!hint.matches("asd"));
    }

    #[test]
    fn test_hint_label_exact_match() {
        let hint = HintLabel::new("as".to_string());

        assert!(hint.is_exact_match("as"));
        assert!(!hint.is_exact_match("a"));
        assert!(!hint.is_exact_match("asd"));
    }

    #[test]
    fn test_left_handed_layout() {
        let layout = KeyboardLayout::left_handed();
        let generator = HintGenerator::new(layout);
        let hints = generator.generate(5);

        assert_eq!(hints.len(), 5);
        assert_eq!(hints[0].text, "a");
        assert_eq!(hints[1].text, "s");
        assert_eq!(hints[2].text, "d");
        assert_eq!(hints[3].text, "f");
        assert_eq!(hints[4].text, "g");
    }

    #[test]
    fn test_custom_layout() {
        let layout = KeyboardLayout::custom(vec!['q', 'w']);
        let generator = HintGenerator::new(layout);
        let hints = generator.generate(4);

        assert_eq!(hints.len(), 4);
        assert_eq!(hints[0].text, "q");
        assert_eq!(hints[1].text, "w");
        assert_eq!(hints[2].text, "qq");
        assert_eq!(hints[3].text, "qw");
    }

    #[test]
    fn test_no_duplicates_1000_hints() {
        let generator = HintGenerator::default();
        let hints = generator.generate(1000);

        assert_eq!(hints.len(), 1000);

        let unique_hints: std::collections::HashSet<_> = hints.iter().map(|h| &h.text).collect();
        assert_eq!(unique_hints.len(), 1000, "All hints must be unique");
    }

    #[test]
    fn test_hint_ordering() {
        let generator = HintGenerator::default();
        let hints = generator.generate(20);

        // First 8 should be single characters
        for i in 0..8 {
            assert_eq!(hints[i].text.len(), 1);
        }

        // Next hints should be two characters
        for i in 8..20 {
            assert_eq!(hints[i].text.len(), 2);
        }
    }
}
