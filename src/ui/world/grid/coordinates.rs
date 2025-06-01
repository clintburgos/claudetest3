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
    // From grid_to_world:
    // world_x = (x - y) * tile_size * 0.5
    // world_y = -(x + y) * tile_size * 0.25
    // 
    // Solving for x and y:
    // world_x / (tile_size * 0.5) = x - y
    // -world_y / (tile_size * 0.25) = x + y
    // 
    // Adding: 2x = world_x / (tile_size * 0.5) + (-world_y / (tile_size * 0.25))
    // Subtracting: -2y = world_x / (tile_size * 0.5) - (-world_y / (tile_size * 0.25))
    
    let wx_scaled = world_pos.x / (tile_size * 0.5);
    let wy_scaled = -world_pos.y / (tile_size * 0.25);
    
    let x = (wx_scaled + wy_scaled) / 2.0;
    let y = (wy_scaled - wx_scaled) / 2.0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_to_world_basic() {
        let tile_size = 64.0;
        
        // Test origin
        let pos = grid_to_world(0, 0, 0, tile_size);
        assert_eq!(pos, Vec3::new(0.0, 0.0, 0.0));
        
        // Test basic positions
        let pos = grid_to_world(1, 0, 0, tile_size);
        assert_eq!(pos, Vec3::new(32.0, -16.0, 0.0));
        
        let pos = grid_to_world(0, 1, 0, tile_size);
        assert_eq!(pos, Vec3::new(-32.0, -16.0, 0.0));
        
        let pos = grid_to_world(1, 1, 0, tile_size);
        assert_eq!(pos, Vec3::new(0.0, -32.0, 0.0));
    }

    #[test]
    fn test_grid_to_world_with_elevation() {
        let tile_size = 64.0;
        
        // Test with elevation
        let pos = grid_to_world(2, 3, 1, tile_size);
        let expected_x = (2 - 3) as f32 * tile_size * 0.5;
        let expected_y = (2 + 3) as f32 * tile_size * 0.25;
        let expected_z = 1.0 * tile_size * 0.5;
        assert_eq!(pos, Vec3::new(expected_x, -expected_y, expected_z));
        
        // Negative elevation
        let pos = grid_to_world(0, 0, -2, tile_size);
        assert_eq!(pos, Vec3::new(0.0, 0.0, -64.0));
    }

    #[test]
    fn test_grid_to_world_negative_coordinates() {
        let tile_size = 32.0;
        
        let pos = grid_to_world(-1, -1, 0, tile_size);
        assert_eq!(pos, Vec3::new(0.0, 16.0, 0.0));
        
        let pos = grid_to_world(-2, 3, 0, tile_size);
        let expected_x = (-2 - 3) as f32 * tile_size * 0.5;
        let expected_y = (-2 + 3) as f32 * tile_size * 0.25;
        assert_eq!(pos, Vec3::new(expected_x, -expected_y, 0.0));
    }

    #[test]
    fn test_world_to_grid_basic() {
        let tile_size = 64.0;
        
        // Test origin
        let grid = world_to_grid(Vec3::new(0.0, 0.0, 0.0), tile_size);
        assert_eq!(grid, (0, 0, 0));
        
        // Test exact positions
        let grid = world_to_grid(Vec3::new(32.0, -16.0, 0.0), tile_size);
        assert_eq!(grid, (1, 0, 0));
        
        let grid = world_to_grid(Vec3::new(-32.0, -16.0, 0.0), tile_size);
        assert_eq!(grid, (0, 1, 0));
    }

    #[test]
    fn test_world_to_grid_rounding() {
        let tile_size = 64.0;
        
        // Test rounding with small offsets
        let grid = world_to_grid(Vec3::new(30.0, -15.0, 0.0), tile_size);
        assert_eq!(grid, (1, 0, 0)); // Should round to nearest
        
        let grid = world_to_grid(Vec3::new(34.0, -17.0, 0.0), tile_size);
        assert_eq!(grid, (1, 0, 0)); // Should still round to (1,0,0)
        
        // Test rounding boundary
        let grid = world_to_grid(Vec3::new(16.0, -8.0, 31.0), tile_size);
        assert_eq!(grid, (1, 0, 1)); // z should round down from 31/32=0.97
        
        let grid = world_to_grid(Vec3::new(16.0, -8.0, 33.0), tile_size);
        assert_eq!(grid, (1, 0, 1)); // z should round up from 33/32=1.03
    }

    #[test]
    fn test_world_to_grid_inverse() {
        let tile_size = 64.0;
        
        // Test that world_to_grid is inverse of grid_to_world for integer positions
        for x in -5..=5 {
            for y in -5..=5 {
                for z in -2..=2 {
                    let world_pos = grid_to_world(x, y, z, tile_size);
                    let grid_pos = world_to_grid(world_pos, tile_size);
                    assert_eq!(grid_pos, (x, y, z));
                }
            }
        }
    }

    #[test]
    fn test_grid_to_isometric() {
        let tile_size = 64.0;
        
        // Test origin
        let iso = grid_to_isometric(0, 0, tile_size);
        assert_eq!(iso, Vec2::new(0.0, 0.0));
        
        // Test basic positions
        let iso = grid_to_isometric(1, 0, tile_size);
        assert_eq!(iso, Vec2::new(32.0, 16.0));
        
        let iso = grid_to_isometric(0, 1, tile_size);
        assert_eq!(iso, Vec2::new(-32.0, 16.0));
        
        let iso = grid_to_isometric(1, 1, tile_size);
        assert_eq!(iso, Vec2::new(0.0, 32.0));
    }

    #[test]
    fn test_grid_to_isometric_large_values() {
        let tile_size = 32.0;
        
        let iso = grid_to_isometric(100, 50, tile_size);
        let expected_x = (100 - 50) as f32 * tile_size * 0.5;
        let expected_y = (100 + 50) as f32 * tile_size * 0.25;
        assert_eq!(iso, Vec2::new(expected_x, expected_y));
        
        // Negative values
        let iso = grid_to_isometric(-10, -20, tile_size);
        let expected_x = (-10 - (-20)) as f32 * tile_size * 0.5;
        let expected_y = (-10 + (-20)) as f32 * tile_size * 0.25;
        assert_eq!(iso, Vec2::new(expected_x, expected_y));
    }

    #[test]
    fn test_grid_center_world() {
        let tile_size = 64.0;
        
        // Even dimensions
        let center = grid_center_world(10, 10, tile_size);
        let expected = grid_to_world(5, 5, 0, tile_size);
        assert_eq!(center, expected);
        
        // Odd dimensions
        let center = grid_center_world(11, 11, tile_size);
        let expected = grid_to_world(5, 5, 0, tile_size); // 11/2 = 5
        assert_eq!(center, expected);
        
        // Different dimensions
        let center = grid_center_world(20, 10, tile_size);
        let expected = grid_to_world(10, 5, 0, tile_size);
        assert_eq!(center, expected);
    }

    #[test]
    fn test_coordinate_conversions_consistency() {
        let tile_size = 64.0;
        
        // Test that grid_to_isometric matches the x,y components of grid_to_world
        for x in -3..=3 {
            for y in -3..=3 {
                let world = grid_to_world(x, y, 0, tile_size);
                let iso = grid_to_isometric(x, y, tile_size);
                
                // The isometric projection should match world x,y (with y negated)
                assert_eq!(iso.x, world.x);
                assert_eq!(iso.y, -world.y);
            }
        }
    }

    #[test]
    fn test_different_tile_sizes() {
        // Test with various tile sizes
        let tile_sizes = [16.0, 32.0, 64.0, 128.0, 256.0];
        
        for &tile_size in &tile_sizes {
            // Test grid_to_world
            let pos = grid_to_world(1, 1, 1, tile_size);
            assert_eq!(pos.x, 0.0);
            assert_eq!(pos.y, -tile_size * 0.5);
            assert_eq!(pos.z, tile_size * 0.5);
            
            // Test grid_to_isometric
            let iso = grid_to_isometric(2, 0, tile_size);
            assert_eq!(iso.x, tile_size);
            assert_eq!(iso.y, tile_size * 0.5);
        }
    }

    #[test]
    fn test_floating_point_precision() {
        let tile_size = 1.0;
        
        // Test with very small tile size
        let pos = grid_to_world(1000, 1000, 0, tile_size);
        let back = world_to_grid(pos, tile_size);
        assert_eq!(back, (1000, 1000, 0));
        
        // Test with fractional tile size
        let tile_size = 33.33;
        let pos = grid_to_world(7, 13, 2, tile_size);
        let back = world_to_grid(pos, tile_size);
        assert_eq!(back, (7, 13, 2));
    }
}