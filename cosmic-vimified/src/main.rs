// COSMIC Vimified - Vimium-style keyboard navigation for COSMIC desktop
// Copyright (C) 2025 COSMIC Vimified Contributors
// Licensed under GPL-3.0-or-later

mod layershell_app;
mod detection;
mod hints;
mod overlay;
mod commands;
mod daemon;
mod cli;

use tracing_subscriber::EnvFilter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tracing::info!("COSMIC Vimified starting...");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));

    // Parse CLI arguments
    use clap::Parser;
    let cli = cli::Cli::parse();

    // Create tokio runtime for async operations
    let rt = tokio::runtime::Runtime::new()?;

    rt.block_on(async {
        match cli.command {
            Some(cli::Commands::Daemon) | None => {
                // Check if already running
                if daemon::is_already_running().await {
                    tracing::error!("Daemon is already running");
                    return Err("Daemon is already running".into());
                }

                // Start daemon mode - just listen for D-Bus commands
                tracing::info!("Starting in daemon mode (overlay will start on first Show command)");

                // Start D-Bus service and wait for commands
                daemon::run_daemon().await?;

                Ok(())
            }
            Some(cli::Commands::Show) => {
                tracing::info!("Sending show command to daemon");
                daemon::send_show_command().await?;
                Ok(())
            }
            Some(cli::Commands::Hide) => {
                tracing::info!("Sending hide command to daemon");
                send_daemon_command("Hide").await?;
                Ok(())
            }
            Some(cli::Commands::Toggle) => {
                tracing::info!("Sending toggle command to daemon");
                send_daemon_command("Toggle").await?;
                Ok(())
            }
        }
    })
}

/// Sends a command to the running COSMIC Vimified daemon via D-Bus.
///
/// # Arguments
///
/// * `command` - The D-Bus method name to invoke (e.g., "Hide", "Toggle")
///
/// # Errors
///
/// Returns an error if the D-Bus connection fails or the daemon is not running.
async fn send_daemon_command(command: &str) -> Result<(), Box<dyn std::error::Error>> {
    use zbus::Connection;

    let connection = Connection::session().await?;

    connection.call_method(
        Some("com.cosmic.Vimified"),
        "/com/cosmic/Vimified",
        Some("com.cosmic.Vimified1"),
        command,
        &(),
    ).await?;

    tracing::info!("Sent {} command to running instance", command);
    Ok(())
}
