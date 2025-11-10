// Command-line interface for cosmic-vimified

use clap::{Parser, Subcommand};

/// Command-line interface for COSMIC Vimified.
///
/// Provides subcommands for controlling the hint overlay daemon.
#[derive(Parser, Debug)]
#[command(name = "cosmic-vimified")]
#[command(about = "Vimium-style keyboard navigation for COSMIC desktop", long_about = None)]
pub struct Cli {
    /// Optional subcommand (defaults to Daemon if not specified)
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start the daemon (default if no command specified)
    Daemon,

    /// Show the hint overlay (sends command to running daemon)
    Show,

    /// Hide the hint overlay (sends command to running daemon)
    Hide,

    /// Toggle the hint overlay (sends command to running daemon)
    Toggle,
}
