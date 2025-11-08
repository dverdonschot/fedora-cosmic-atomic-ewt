// COSMIC Vimified - Vimium-style keyboard navigation for COSMIC desktop
// Copyright (C) 2025 COSMIC Vimified Contributors
// Licensed under GPL-3.0-or-later

mod app;
mod detection;
mod overlay;

use cosmic::app::Settings;
use cosmic::iced::Size;
use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("COSMIC Vimified starting...");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));

    let settings = Settings::default()
        .size(Size::new(1920.0, 1080.0))
        .transparent(true)
        .exit_on_close(true);

    cosmic::app::run::<app::App>(settings, ())?;

    Ok(())
}
