// Commands for controlling the overlay from daemon

/// Internal commands for controlling the overlay application.
///
/// These commands are sent from the D-Bus daemon interface to the overlay
/// application via a message channel.
#[derive(Debug, Clone)]
pub enum DaemonCommand {
    /// Show the overlay and detect clickable elements
    Show,
    /// Hide the overlay without exiting the daemon
    Hide,
    /// Toggle the overlay visibility
    Toggle,
    /// Exit the entire application
    Exit,
}
