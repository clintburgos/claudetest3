//! Culling Toggle System - Runtime control of view culling
//!
//! This module provides a system for toggling view culling on/off during gameplay.
//! Useful for debugging and performance testing.

use super::ViewCullingConfig;
use bevy::prelude::*;

/// Toggle culling on/off with the C key
pub fn toggle_culling_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut culling_config: ResMut<ViewCullingConfig>,
) {
    if keyboard.just_pressed(KeyCode::KeyC) {
        culling_config.enabled = !culling_config.enabled;
        info!(
            "View culling {} (press C to toggle)",
            if culling_config.enabled {
                "ENABLED"
            } else {
                "DISABLED - All tiles will be rendered!"
            }
        );
    }
}