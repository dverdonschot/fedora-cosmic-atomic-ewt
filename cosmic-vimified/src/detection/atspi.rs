use super::filters;
use super::types::{DetectedElement, ElementBounds};
use anyhow::{Context, Result};
use atspi::CoordType;
use atspi_connection::AccessibilityConnection;
use atspi_proxies::accessible::{AccessibleProxy, ObjectRefExt};
use atspi_proxies::component::ComponentProxy;
use std::pin::Pin;
use std::future::Future;

/// AT-SPI detector for finding clickable UI elements across all applications.
///
/// This detector connects to the accessibility bus and traverses the AT-SPI tree
/// to find all clickable elements (buttons, links, etc.) on the screen.
pub struct AtSpiDetector {
    /// Connection to the AT-SPI accessibility bus
    connection: AccessibilityConnection,
}

impl AtSpiDetector {
    /// Creates a new AT-SPI detector and establishes connection to the accessibility bus.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Session accessibility cannot be enabled
    /// - Connection to the AT-SPI bus fails
    pub async fn new() -> Result<Self> {
        atspi::connection::set_session_accessibility(true)
            .await
            .context("Failed to enable session accessibility")?;

        let connection = AccessibilityConnection::new()
            .await
            .context("Failed to connect to AT-SPI")?;

        tracing::info!("Connected to AT-SPI accessibility bus");
        Ok(Self { connection })
    }

    /// Detects all clickable elements across all accessible applications.
    ///
    /// Traverses the entire AT-SPI accessibility tree, identifying and filtering
    /// clickable elements based on their role, state, and size.
    ///
    /// # Returns
    ///
    /// A vector of detected elements with their screen positions and metadata.
    ///
    /// # Errors
    ///
    /// Returns an error if the AT-SPI registry or child applications cannot be accessed.
    pub async fn detect_all_elements(&self) -> Result<Vec<DetectedElement>> {
        let mut elements = Vec::new();

        let registry = self
            .connection
            .root_accessible_on_registry()
            .await
            .context("Failed to get AT-SPI registry")?;

        let child_count = registry
            .child_count()
            .await
            .context("Failed to get child count")?;

        tracing::debug!("Found {} accessible applications", child_count);

        match registry.get_children().await {
            Ok(children) => {
                for child_obj in children {
                    if child_obj.is_null() {
                        continue;
                    }

                    match child_obj
                        .into_accessible_proxy(self.connection.connection())
                        .await
                    {
                        Ok(child_proxy) => {
                            self.traverse_tree(&child_proxy, &mut elements).await;
                        }
                        Err(e) => {
                            tracing::warn!("Failed to create proxy: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to get children: {}", e);
            }
        }

        tracing::info!("Detected {} clickable elements", elements.len());
        Ok(elements)
    }

    fn traverse_tree<'a>(
        &'a self,
        accessible: &'a AccessibleProxy<'_>,
        elements: &'a mut Vec<DetectedElement>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
        if let Err(e) = self.process_accessible(accessible, elements).await {
            tracing::trace!("Error processing accessible: {}", e);
        }

        match accessible.get_children().await {
            Ok(children) => {
                for child_obj in children {
                    if child_obj.is_null() {
                        continue;
                    }

                    match child_obj
                        .into_accessible_proxy(self.connection.connection())
                        .await
                    {
                        Ok(child_proxy) => {
                            self.traverse_tree(&child_proxy, elements).await;
                        }
                        Err(e) => {
                            tracing::trace!("Failed to create child proxy: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                tracing::trace!("Failed to get children: {}", e);
            }
        }
        })
    }

    async fn process_accessible(
        &self,
        accessible: &AccessibleProxy<'_>,
        elements: &mut Vec<DetectedElement>,
    ) -> Result<()> {
        let role = accessible.get_role().await?;
        let states = accessible.get_state().await?;

        // Get bounds first to check size before creating expensive proxies
        let component = ComponentProxy::builder(accessible.inner().connection())
            .destination(accessible.inner().destination())?
            .path(accessible.inner().path())?
            .build()
            .await?;

        let extents = component.get_extents(CoordType::Screen).await?;

        let bounds = ElementBounds {
            x: extents.0,
            y: extents.1,
            width: extents.2,
            height: extents.3,
        };

        // Basic bounds validity check
        if !bounds.is_valid() {
            return Ok(());
        }

        // Apply comprehensive filtering with role, states, and bounds
        if !filters::should_process_element(role, &states, &bounds) {
            return Ok(());
        }

        let name = accessible.name().await.unwrap_or_default();
        let description = accessible.description().await.unwrap_or_default();

        let app_name = self.get_app_name(accessible).await.unwrap_or_default();

        let element = DetectedElement::new(bounds, role, name, app_name, description);

        tracing::debug!("Found clickable element: {} [{}]", element.name, element.role);
        elements.push(element);

        Ok(())
    }

    async fn get_app_name(&self, accessible: &AccessibleProxy<'_>) -> Result<String> {
        let mut current_obj = accessible.parent().await?;

        for _ in 0..10 {
            if current_obj.is_null() {
                break;
            }

            let current = current_obj
                .into_accessible_proxy(self.connection.connection())
                .await?;

            if let Ok(role) = current.get_role().await {
                if role == atspi::Role::Application {
                    return Ok(current.name().await.unwrap_or_default());
                }
            }

            current_obj = current.parent().await?;
        }

        Ok(String::from("Unknown"))
    }
}
