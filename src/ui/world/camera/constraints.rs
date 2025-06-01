//! Camera Constraints - Boundary enforcement for camera movement
//!
//! This file contains systems and functions that keep the camera
//! within valid bounds, ensuring the map remains visible.
//!
//! # Constraints
//! - Camera cannot move beyond map boundaries
//! - Maintains padding to keep edges visible
//! - Adjusts bounds based on zoom level

use super::components::{CameraState, IsometricCamera};
use crate::ui::world::grid::{coordinates::grid_center_world, GridConfig};
use bevy::prelude::*;

/// Apply constraints to keep camera within map bounds
pub fn apply_camera_constraints_system(
    mut camera_query: Query<(&mut Transform, &CameraState), With<IsometricCamera>>,
    grid_config: Res<GridConfig>,
    windows: Query<&Window>,
) {
    let Ok((mut transform, state)) = camera_query.single_mut() else {
        return;
    };
    let Ok(window) = windows.single() else { return };

    // Calculate visible area based on zoom
    let visible_width = window.width() / state.zoom;
    let visible_height = window.height() / state.zoom;

    // Calculate map bounds in world space
    let (min_bounds, max_bounds) = calculate_bounds(&grid_config, visible_width, visible_height);

    // Clamp camera position
    transform.translation.x = transform.translation.x.clamp(min_bounds.x, max_bounds.x);
    transform.translation.y = transform.translation.y.clamp(min_bounds.y, max_bounds.y);
}

/// Calculate camera bounds based on map size and visible area
fn calculate_bounds(
    grid_config: &GridConfig,
    visible_width: f32,
    visible_height: f32,
) -> (Vec2, Vec2) {
    // Get world bounds of the map
    let map_width = grid_config.width as f32 * grid_config.tile_size;
    let map_height = grid_config.height as f32 * grid_config.tile_size * 0.5; // Isometric ratio

    // Center of the map
    let center = grid_center_world(grid_config.width, grid_config.height, grid_config.tile_size);

    // Add padding
    let padding = grid_config.tile_size * 2.0;

    // Calculate min/max bounds
    let half_visible = Vec2::new(visible_width * 0.5, visible_height * 0.5);

    let min_x = -map_width * 0.5 + half_visible.x - padding;
    let max_x = map_width * 0.5 - half_visible.x + padding;
    let min_y = -map_height - half_visible.y - padding;
    let max_y = map_height - half_visible.y + padding;

    // If map is smaller than viewport, center it
    let min_x = if max_x < min_x { center.x } else { min_x };
    let max_x = if max_x < min_x { center.x } else { max_x };
    let min_y = if max_y < min_y { center.y } else { min_y };
    let max_y = if max_y < min_y { center.y } else { max_y };

    (Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
}
