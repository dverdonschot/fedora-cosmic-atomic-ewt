// Commands for controlling the overlay from daemon

#[derive(Debug, Clone)]
pub enum DaemonCommand {
    /// Show the overlay
    Show,
    /// Hide the overlay
    Hide,
    /// Toggle the overlay visibility
    Toggle,
    /// Exit the application
    Exit,
}
