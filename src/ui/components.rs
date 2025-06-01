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

#[derive(Clone, Copy, Debug)]
pub enum ButtonAction {
    Navigate(usize),
    OpenDialog,
    CloseDialog,
    Custom(u32),
}
