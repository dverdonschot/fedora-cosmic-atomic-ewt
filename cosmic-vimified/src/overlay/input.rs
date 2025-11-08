use iced::keyboard::Key;
use iced::keyboard::key::Named;

pub fn is_escape_key(key: &Key) -> bool {
    matches!(key, Key::Named(Named::Escape))
}
