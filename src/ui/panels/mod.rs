//! UI Panel System - Information displays and game controls
//!
//! This module provides various UI panels for displaying game information
//! and controls, including tile info, game controls, and menus.

pub mod components;
pub mod info_panel;
pub mod top_bar;

use bevy::prelude::*;

/// Plugin that adds all UI panels to the game
pub struct UIPanelsPlugin;

impl Plugin for UIPanelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((info_panel::InfoPanelPlugin, top_bar::TopBarPlugin));
    }
}
