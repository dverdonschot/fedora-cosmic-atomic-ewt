use cosmic::iced::keyboard::Key;

pub fn is_escape_key(key: &Key) -> bool {
    matches!(key, Key::Named(cosmic::iced::keyboard::key::Named::Escape))
}
