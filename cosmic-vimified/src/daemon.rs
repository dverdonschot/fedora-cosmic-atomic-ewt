// D-Bus daemon interface for cosmic-vimified
// Allows controlling the overlay from external commands

use crate::commands::DaemonCommand;
use anyhow::Result;
use tokio::sync::mpsc;
use zbus::{Connection, interface};

/// D-Bus interface for controlling the cosmic-vimified overlay
pub struct VimifiedDaemon {
    command_tx: mpsc::UnboundedSender<DaemonCommand>,
}

#[interface(name = "com.cosmic.Vimified1")]
impl VimifiedDaemon {
    /// Show the hint overlay
    async fn show(&self) -> zbus::fdo::Result<()> {
        tracing::info!("D-Bus: Received show command");
        self.command_tx.send(DaemonCommand::Show)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Failed to send command: {}", e)))?;
        Ok(())
    }

    /// Hide the hint overlay
    async fn hide(&self) -> zbus::fdo::Result<()> {
        tracing::info!("D-Bus: Received hide command");
        self.command_tx.send(DaemonCommand::Hide)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Failed to send command: {}", e)))?;
        Ok(())
    }

    /// Toggle the hint overlay
    async fn toggle(&self) -> zbus::fdo::Result<()> {
        tracing::info!("D-Bus: Received toggle command");
        self.command_tx.send(DaemonCommand::Toggle)
            .map_err(|e| zbus::fdo::Error::Failed(format!("Failed to send command: {}", e)))?;
        Ok(())
    }
}

/// Start the D-Bus service
pub async fn start_dbus_service(command_tx: mpsc::UnboundedSender<DaemonCommand>) -> Result<Connection> {
    let connection = Connection::session().await?;

    let daemon = VimifiedDaemon { command_tx };

    connection
        .object_server()
        .at("/com/cosmic/Vimified", daemon)
        .await?;

    connection
        .request_name("com.cosmic.Vimified")
        .await?;

    tracing::info!("D-Bus service started: com.cosmic.Vimified");
    Ok(connection)
}

/// Check if another instance is already running
pub async fn is_already_running() -> bool {
    match Connection::session().await {
        Ok(conn) => {
            match conn.call_method(
                Some("com.cosmic.Vimified"),
                "/com/cosmic/Vimified",
                Some("com.cosmic.Vimified1"),
                "Toggle",
                &(),
            ).await {
                Ok(_) => {
                    tracing::info!("Another instance is already running");
                    true
                }
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}

/// Send show command to running instance
pub async fn send_show_command() -> Result<()> {
    let connection = Connection::session().await?;

    connection.call_method(
        Some("com.cosmic.Vimified"),
        "/com/cosmic/Vimified",
        Some("com.cosmic.Vimified1"),
        "Show",
        &(),
    ).await?;

    tracing::info!("Sent show command to running instance");
    Ok(())
}

/// Run the daemon - listen for D-Bus commands and spawn overlay on demand
pub async fn run_daemon() -> Result<()> {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let connection = Connection::session().await?;
    
    // Channel for D-Bus commands
    let (command_tx, mut command_rx) = mpsc::unbounded_channel::<DaemonCommand>();
    
    // Start D-Bus service
    let daemon = VimifiedDaemon { command_tx };
    connection
        .object_server()
        .at("/com/cosmic/Vimified", daemon)
        .await?;
    connection
        .request_name("com.cosmic.Vimified")
        .await?;

    tracing::info!("D-Bus service started, waiting for commands...");

    // Track if overlay is currently shown
    let overlay_active = Arc::new(Mutex::new(false));
    let overlay_thread: Arc<Mutex<Option<thread::JoinHandle<()>>>> = Arc::new(Mutex::new(None));

    // Listen for commands
    loop {
        if let Some(cmd) = command_rx.recv().await {
            tracing::info!("Received command: {:?}", cmd);
            
            match cmd {
                DaemonCommand::Show => {
                    let mut active = overlay_active.lock().unwrap();
                    if !*active {
                        tracing::info!("Spawning overlay window...");
                        *active = true;
                        
                        let overlay_active_clone = Arc::clone(&overlay_active);
                        let handle = thread::spawn(move || {
                            if let Err(e) = crate::layershell_app::run_once() {
                                tracing::error!("Overlay error: {}", e);
                            }
                            tracing::info!("Overlay window closed");
                            *overlay_active_clone.lock().unwrap() = false;
                        });
                        
                        *overlay_thread.lock().unwrap() = Some(handle);
                    } else {
                        tracing::info!("Overlay already active");
                    }
                }
                DaemonCommand::Hide | DaemonCommand::Toggle => {
                    // These will be handled by the overlay itself
                    // For now, just log
                    tracing::info!("Command will be handled by overlay: {:?}", cmd);
                }
                DaemonCommand::Exit => {
                    tracing::info!("Exiting daemon");
                    break;
                }
            }
        }
    }

    Ok(())
}
