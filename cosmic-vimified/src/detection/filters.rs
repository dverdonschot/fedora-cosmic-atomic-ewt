use atspi::Role;

const CLICKABLE_ROLES: &[Role] = &[
    Role::Button,
    Role::ToggleButton,
    Role::RadioButton,
    Role::CheckBox,
    Role::Link,
    Role::MenuItem,
    Role::MenuBar,
    Role::Menu,
    Role::ListItem,
    Role::Icon,
    Role::PageTab,
    Role::PageTabList,
    Role::ComboBox,
    Role::Entry,
    Role::Text,
];

pub fn is_clickable_role(role: Role) -> bool {
    CLICKABLE_ROLES.contains(&role)
}

pub fn is_visible_state(states: &atspi::StateSet) -> bool {
    states.contains(atspi::State::Visible) && states.contains(atspi::State::Showing)
}

pub fn is_enabled_state(states: &atspi::StateSet) -> bool {
    states.contains(atspi::State::Enabled) || states.contains(atspi::State::Sensitive)
}

pub fn should_process_element(role: Role, states: &atspi::StateSet) -> bool {
    is_clickable_role(role) && is_visible_state(states) && is_enabled_state(states)
}
