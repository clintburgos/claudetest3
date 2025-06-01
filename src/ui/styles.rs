use bevy::prelude::*;

pub const HEADER_HEIGHT: f32 = 60.0;
pub const SIDEBAR_WIDTH: f32 = 200.0;
pub const BUTTON_HEIGHT: f32 = 40.0;
pub const PADDING: f32 = 10.0;
pub const BUTTON_GAP: f32 = 10.0;

// Button colors
pub const BUTTON_COLOR: Color = Color::srgb(0.3, 0.3, 0.3);
pub const BUTTON_HOVER_COLOR: Color = Color::srgb(0.4, 0.4, 0.4);
pub const BUTTON_PRESSED_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(HEADER_HEIGHT, 60.0);
        assert_eq!(SIDEBAR_WIDTH, 200.0);
        assert_eq!(BUTTON_HEIGHT, 40.0);
        assert_eq!(PADDING, 10.0);
        assert_eq!(BUTTON_GAP, 10.0);
    }

    #[test]
    fn test_button_colors() {
        assert_eq!(BUTTON_COLOR, Color::srgb(0.3, 0.3, 0.3));
        assert_eq!(BUTTON_HOVER_COLOR, Color::srgb(0.4, 0.4, 0.4));
        assert_eq!(BUTTON_PRESSED_COLOR, Color::srgb(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_ui_colors() {
        assert_eq!(UiColors::BACKGROUND, Color::srgb(0.1, 0.1, 0.1));
        assert_eq!(UiColors::HEADER_BG, Color::srgb(0.15, 0.15, 0.15));
        assert_eq!(UiColors::SIDEBAR_BG, Color::srgb(0.2, 0.2, 0.2));
        assert_eq!(UiColors::BUTTON_NORMAL, Color::srgb(0.3, 0.3, 0.3));
        assert_eq!(UiColors::BUTTON_HOVER, Color::srgb(0.4, 0.4, 0.4));
        assert_eq!(UiColors::BUTTON_PRESSED, Color::srgb(0.5, 0.5, 0.5));
        assert_eq!(UiColors::TEXT_PRIMARY, Color::WHITE);
        assert_eq!(UiColors::TEXT_SECONDARY, Color::srgb(0.8, 0.8, 0.8));
    }

    #[test]
    fn test_default_button_style() {
        let style = default_button_style();
        assert_eq!(style.width, Val::Percent(100.0));
        assert_eq!(style.height, Val::Px(BUTTON_HEIGHT));
        assert_eq!(style.justify_content, JustifyContent::Center);
        assert_eq!(style.align_items, AlignItems::Center);
    }
}
