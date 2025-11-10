use atspi::Role;
use crate::detection::types::ElementBounds;

/// Primary clickable roles - these are definitely interactive
const PRIMARY_CLICKABLE_ROLES: &[Role] = &[
    Role::Button,
    Role::ToggleButton,
    Role::RadioButton,
    Role::CheckBox,
    Role::Link,
    Role::MenuItem,
];

/// Secondary clickable roles - may be interactive depending on context
const SECONDARY_CLICKABLE_ROLES: &[Role] = &[
    Role::MenuBar,
    Role::Menu,
    Role::PageTab,
    Role::ComboBox,
    Role::ListItem,  // Only if has actions
];

/// Roles that are usually NOT clickable (decorative/informational)
const NON_CLICKABLE_ROLES: &[Role] = &[
    Role::Icon,           // Usually decorative
    Role::Text,           // Plain text labels
    Role::Label,          // Static labels
    Role::Heading,        // Section headers
    Role::Paragraph,      // Text paragraphs
    Role::StatusBar,      // Status displays
    Role::ToolTip,        // Tooltips
    Role::Canvas,         // Drawing areas
    Role::Panel,          // Container panels
    Role::ScrollBar,      // Scrollbars (handled by system)
    Role::Separator,      // Visual separators
];

/// Checks if the role is a primary clickable element.
///
/// Primary clickable roles are elements that are always intended to be interactive,
/// such as buttons, links, and checkboxes.
pub fn is_primary_clickable_role(role: Role) -> bool {
    PRIMARY_CLICKABLE_ROLES.contains(&role)
}

/// Checks if the role is a secondary clickable element.
///
/// Secondary clickable roles may be interactive depending on their state,
/// such as menu items or list items that have action handlers.
pub fn is_secondary_clickable_role(role: Role) -> bool {
    SECONDARY_CLICKABLE_ROLES.contains(&role)
}

/// Checks if the role is typically non-clickable.
///
/// These are usually decorative or informational elements like labels,
/// icons, or static text that should not have hints.
pub fn is_non_clickable_role(role: Role) -> bool {
    NON_CLICKABLE_ROLES.contains(&role)
}

/// Checks if the element is visible on screen.
///
/// An element must have both Visible and Showing states to be considered visible.
pub fn is_visible_state(states: &atspi::StateSet) -> bool {
    states.contains(atspi::State::Visible) && states.contains(atspi::State::Showing)
}

/// Checks if the element is enabled and can be interacted with.
///
/// An element is considered enabled if it has either the Enabled or Sensitive state.
pub fn is_enabled_state(states: &atspi::StateSet) -> bool {
    states.contains(atspi::State::Enabled) || states.contains(atspi::State::Sensitive)
}

/// Checks if the element has any clickable-related states.
///
/// Returns `true` if the element is focusable, selectable, or checkable,
/// indicating it can accept user interaction.
pub fn has_clickable_state(states: &atspi::StateSet) -> bool {
    states.contains(atspi::State::Focusable)
        || states.contains(atspi::State::Selectable)
        || states.contains(atspi::State::Checkable)
}

/// Checks if the element size is reasonable for clicking.
///
/// Filters out tiny decorative elements (< 10x10 pixels) and huge containers
/// (> 2000 pixels in any dimension) that are unlikely to be clickable targets.
pub fn is_reasonable_size(bounds: &ElementBounds) -> bool {
    const MIN_SIZE: i32 = 10;   // Minimum 10x10 pixels
    const MAX_SIZE: i32 = 2000; // Maximum 2000 pixels in any dimension

    bounds.width >= MIN_SIZE
        && bounds.height >= MIN_SIZE
        && bounds.width <= MAX_SIZE
        && bounds.height <= MAX_SIZE
}

/// Determines if an element should be processed and shown as a hint target.
///
/// This is the main filtering function that combines role, state, and size checks
/// to decide if an element is worth showing a hint for.
///
/// # Arguments
///
/// * `role` - The AT-SPI role of the element
/// * `states` - The AT-SPI state set of the element
/// * `bounds` - The element's bounding box
///
/// # Returns
///
/// `true` if the element should have a hint displayed, `false` otherwise.
pub fn should_process_element(role: Role, states: &atspi::StateSet, bounds: &ElementBounds) -> bool {
    // Always exclude known non-clickable roles
    if is_non_clickable_role(role) {
        return false;
    }

    // Check basic requirements
    if !is_visible_state(states) || !is_enabled_state(states) {
        return false;
    }

    // Check size is reasonable
    if !is_reasonable_size(bounds) {
        return false;
    }

    // Primary clickable roles are always accepted
    if is_primary_clickable_role(role) {
        return true;
    }

    // Secondary roles need additional verification
    if is_secondary_clickable_role(role) {
        // Must have clickable states
        return has_clickable_state(states);
    }

    // Unknown roles - only accept if they have clickable states
    has_clickable_state(states)
}
