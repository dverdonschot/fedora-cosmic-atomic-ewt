use atspi::Role;
use serde::{Deserialize, Serialize};

/// Represents the bounding box of a UI element in screen coordinates.
///
/// Uses screen pixel coordinates where (0,0) is the top-left corner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementBounds {
    /// X coordinate of the element's left edge (pixels from left)
    pub x: i32,
    /// Y coordinate of the element's top edge (pixels from top)
    pub y: i32,
    /// Width of the element in pixels
    pub width: i32,
    /// Height of the element in pixels
    pub height: i32,
}

impl ElementBounds {
    /// Calculates the center point of the bounding box.
    ///
    /// Returns the (x, y) coordinates of the center point in screen coordinates.
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    /// Checks if the bounds represent a valid, non-zero-sized element.
    ///
    /// Returns `true` if both width and height are positive.
    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    /// Checks if the element is fully visible within the given screen dimensions.
    ///
    /// # Arguments
    ///
    /// * `screen_width` - Width of the screen in pixels
    /// * `screen_height` - Height of the screen in pixels
    ///
    /// Returns `true` if the element is entirely within screen bounds.
    pub fn is_on_screen(&self, screen_width: i32, screen_height: i32) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.x < screen_width
            && self.y < screen_height
            && self.x + self.width <= screen_width
            && self.y + self.height <= screen_height
    }
}

/// Represents a clickable UI element detected via AT-SPI.
///
/// This struct contains all information needed to identify and interact with
/// a UI element, including its screen position, role, and metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedElement {
    /// Screen bounding box of the element
    pub bounds: ElementBounds,
    /// AT-SPI role of the element (e.g., "Button", "Link", "Checkbox")
    pub role: String,
    /// Accessible name/label of the element
    pub name: String,
    /// Name of the application containing this element
    pub app_name: String,
    /// Accessible description of the element (may be empty)
    pub description: String,
}

impl DetectedElement {
    /// Creates a new detected element from AT-SPI data.
    ///
    /// # Arguments
    ///
    /// * `bounds` - The element's bounding box in screen coordinates
    /// * `role` - AT-SPI role from the accessibility tree
    /// * `name` - Accessible name/label text
    /// * `app_name` - Name of the containing application
    /// * `description` - Accessible description text
    pub fn new(
        bounds: ElementBounds,
        role: Role,
        name: String,
        app_name: String,
        description: String,
    ) -> Self {
        Self {
            bounds,
            role: format!("{:?}", role),
            name,
            app_name,
            description,
        }
    }

    /// Returns the center point of the element in screen coordinates.
    ///
    /// This is typically used as the click target position.
    pub fn center(&self) -> (i32, i32) {
        self.bounds.center()
    }
}

impl std::fmt::Display for DetectedElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{role}] {name} @ ({x},{y}) {w}x{h} in {app}",
            role = self.role,
            name = self.name,
            x = self.bounds.x,
            y = self.bounds.y,
            w = self.bounds.width,
            h = self.bounds.height,
            app = self.app_name
        )
    }
}
