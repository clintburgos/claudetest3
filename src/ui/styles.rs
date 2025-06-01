use bevy::prelude::*;

pub const HEADER_HEIGHT: f32 = 60.0;
pub const SIDEBAR_WIDTH: f32 = 200.0;
pub const BUTTON_HEIGHT: f32 = 40.0;
pub const PADDING: f32 = 10.0;
pub const BUTTON_GAP: f32 = 10.0;

pub struct UiColors;

impl UiColors {
    pub const BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.1);
    pub const HEADER_BG: Color = Color::srgb(0.15, 0.15, 0.15);
    pub const SIDEBAR_BG: Color = Color::srgb(0.2, 0.2, 0.2);
    pub const BUTTON_NORMAL: Color = Color::srgb(0.3, 0.3, 0.3);
    pub const BUTTON_HOVER: Color = Color::srgb(0.4, 0.4, 0.4);
    pub const BUTTON_PRESSED: Color = Color::srgb(0.5, 0.5, 0.5);
    pub const TEXT_PRIMARY: Color = Color::WHITE;
    pub const TEXT_SECONDARY: Color = Color::srgb(0.8, 0.8, 0.8);
}

pub fn default_button_style() -> Node {
    Node {
        width: Val::Percent(100.0),
        height: Val::Px(BUTTON_HEIGHT),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    }
}