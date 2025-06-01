//! Coordinate Systems - Transformations between grid, world, and screen space
//! 
//! This file provides functions to convert between different coordinate systems:
//! - Grid space: Integer tile positions
//! - World space: Bevy world coordinates
//! - Isometric space: 2D isometric projection
//! 
//! # Isometric Projection
//! Uses a 2:1 width:height ratio for diamond-shaped tiles
//! X axis goes right-down, Y axis goes left-down

use bevy::prelude::*;

/// Convert grid coordinates to world position
/// 
/// # Arguments
/// * `x` - Grid X coordinate (West to East)
/// * `y` - Grid Y coordinate (North to South) 
/// * `z` - Grid Z coordinate (elevation)
/// * `tile_size` - Size of each tile in world units
pub fn grid_to_world(x: i32, y: i32, z: i32, tile_size: f32) -> Vec3 {
    // Isometric transformation
    let world_x = (x - y) as f32 * tile_size * 0.5;
    let world_y = (x + y) as f32 * tile_size * 0.25;
    let world_z = z as f32 * tile_size * 0.5;
    
    Vec3::new(world_x, -world_y, world_z)
}

/// Convert world position to grid coordinates
/// 
/// # Arguments
/// * `world_pos` - Position in world space
/// * `tile_size` - Size of each tile in world units
/// 
/// Returns the nearest grid position
pub fn world_to_grid(world_pos: Vec3, tile_size: f32) -> (i32, i32, i32) {
    // Inverse isometric transformation
    let x = world_pos.x / (tile_size * 0.5) - world_pos.y / (tile_size * 0.25);
    let y = -world_pos.x / (tile_size * 0.5) - world_pos.y / (tile_size * 0.25);
    let z = world_pos.z / (tile_size * 0.5);
    
    (
        x.round() as i32,
        y.round() as i32,
        z.round() as i32,
    )
}

/// Convert grid coordinates to isometric screen position
/// 
/// This is similar to grid_to_world but returns 2D coordinates
/// suitable for UI positioning
pub fn grid_to_isometric(x: i32, y: i32, tile_size: f32) -> Vec2 {
    let screen_x = (x - y) as f32 * tile_size * 0.5;
    let screen_y = (x + y) as f32 * tile_size * 0.25;
    
    Vec2::new(screen_x, screen_y)
}

/// Calculate the center of the grid in world space
pub fn grid_center_world(width: i32, height: i32, tile_size: f32) -> Vec3 {
    grid_to_world(width / 2, height / 2, 0, tile_size)
}