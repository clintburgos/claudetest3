//! Isometric Culling - Proper culling for isometric projection
//!
//! The key insight is that in isometric projection, the visible area in grid space
//! is a rotated diamond, not a rectangle. We need to account for this when
//! determining which tiles are visible.

use bevy::prelude::*;
use crate::ui::world::grid::coordinates::{grid_to_world, world_to_grid};

/// Calculate which tiles are visible in an isometric view
/// 
/// In isometric projection, the screen rectangle maps to a rotated diamond in grid space.
/// We need to find all tiles whose centers fall within or near this diamond.
pub fn calculate_isometric_visible_tiles(
    camera_pos: Vec3,
    window_size: Vec2,
    camera_zoom: f32,
    tile_size: f32,
    grid_width: i32,
    grid_height: i32,
    buffer: i32,
) -> (i32, i32, i32, i32) {
    let scale = camera_zoom.max(0.001);
    
    // Calculate the visible world area
    let half_width = window_size.x / scale * 0.5;
    let half_height = window_size.y / scale * 0.5;
    
    // The four corners of the screen in world space
    let corners = [
        Vec3::new(camera_pos.x - half_width, camera_pos.y - half_height, 0.0), // Bottom-left
        Vec3::new(camera_pos.x + half_width, camera_pos.y - half_height, 0.0), // Bottom-right
        Vec3::new(camera_pos.x + half_width, camera_pos.y + half_height, 0.0), // Top-right
        Vec3::new(camera_pos.x - half_width, camera_pos.y + half_height, 0.0), // Top-left
    ];
    
    // Convert each corner to grid coordinates
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    
    for corner in &corners {
        let (gx, gy, _) = world_to_grid(*corner, tile_size);
        min_x = min_x.min(gx);
        max_x = max_x.max(gx);
        min_y = min_y.min(gy);
        max_y = max_y.max(gy);
    }
    
    // Add dynamic buffer based on zoom level
    let zoom_buffer = if scale < 1.0 {
        // When zoomed out, we need more buffer to ensure smooth edges
        let zoom_factor = 1.0 / scale;
        (buffer as f32 * zoom_factor.sqrt() * 2.0).ceil() as i32
    } else {
        buffer
    };
    
    // Apply buffer and clamp to grid bounds
    min_x = (min_x - zoom_buffer).max(0);
    max_x = (max_x + zoom_buffer).min(grid_width - 1);
    min_y = (min_y - zoom_buffer).max(0);
    max_y = (max_y + zoom_buffer).min(grid_height - 1);
    
    // For isometric view, we might need to expand further to catch edge tiles
    // This is because the screen rectangle becomes a diamond in grid space
    let diagonal_buffer = (zoom_buffer as f32 * 1.5).ceil() as i32;
    min_x = (min_x - diagonal_buffer).max(0);
    max_x = (max_x + diagonal_buffer).min(grid_width - 1);
    min_y = (min_y - diagonal_buffer).max(0);
    max_y = (max_y + diagonal_buffer).min(grid_height - 1);
    
    (min_x, min_y, max_x, max_y)
}

/// Check if a grid position is potentially visible given the camera view
pub fn is_tile_potentially_visible(
    tile_x: i32,
    tile_y: i32,
    camera_pos: Vec3,
    window_size: Vec2,
    camera_zoom: f32,
    tile_size: f32,
) -> bool {
    let tile_world = grid_to_world(tile_x, tile_y, 0, tile_size);
    let scale = camera_zoom.max(0.001);
    
    // Calculate the visible world area with margin
    let half_width = window_size.x / scale * 0.5;
    let half_height = window_size.y / scale * 0.5;
    
    // Add significant margin to ensure we don't cull edge tiles
    let margin_factor = if scale < 1.0 { 2.0 / scale } else { 1.5 };
    let margin_x = tile_size * margin_factor;
    let margin_y = tile_size * margin_factor * 0.5; // Less margin needed vertically
    
    // Check if tile is within extended bounds
    tile_world.x >= camera_pos.x - half_width - margin_x &&
    tile_world.x <= camera_pos.x + half_width + margin_x &&
    tile_world.y >= camera_pos.y - half_height - margin_y &&
    tile_world.y <= camera_pos.y + half_height + margin_y
}