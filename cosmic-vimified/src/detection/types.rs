use atspi::Role;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementBounds {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl ElementBounds {
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn is_valid(&self) -> bool {
        self.width > 0 && self.height > 0
    }

    pub fn is_on_screen(&self, screen_width: i32, screen_height: i32) -> bool {
        self.x >= 0
            && self.y >= 0
            && self.x < screen_width
            && self.y < screen_height
            && self.x + self.width <= screen_width
            && self.y + self.height <= screen_height
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedElement {
    pub bounds: ElementBounds,
    pub role: String,
    pub name: String,
    pub app_name: String,
    pub description: String,
}

impl DetectedElement {
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
