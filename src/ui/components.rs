use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu;

#[derive(Component)]
pub struct Sidebar;

#[derive(Component)]
pub struct ContentPanel;

#[derive(Component)]
pub struct Header;

#[derive(Component)]
pub struct InteractiveButton {
    pub action: ButtonAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonAction {
    Navigate(usize),
    OpenDialog,
    CloseDialog,
    Custom(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_action_clone() {
        let action = ButtonAction::Navigate(5);
        let cloned = action.clone();
        assert_eq!(action, cloned);
    }

    #[test]
    fn test_button_action_copy() {
        let action = ButtonAction::Custom(42);
        let copied = action; // Copy trait
        assert_eq!(action, copied);
    }

    #[test]
    fn test_button_action_debug() {
        let action = ButtonAction::OpenDialog;
        let debug_str = format!("{:?}", action);
        assert_eq!(debug_str, "OpenDialog");
    }

    #[test]
    fn test_button_action_equality() {
        assert_eq!(ButtonAction::Navigate(1), ButtonAction::Navigate(1));
        assert_ne!(ButtonAction::Navigate(1), ButtonAction::Navigate(2));
        assert_ne!(ButtonAction::OpenDialog, ButtonAction::CloseDialog);
        assert_eq!(ButtonAction::Custom(10), ButtonAction::Custom(10));
    }

    #[test]
    fn test_interactive_button_component() {
        let button = InteractiveButton {
            action: ButtonAction::OpenDialog,
        };
        assert!(matches!(button.action, ButtonAction::OpenDialog));
    }
}
