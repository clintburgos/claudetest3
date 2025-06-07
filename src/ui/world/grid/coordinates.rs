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

    (x.round() as i32, y.round() as i32, z.round() as i32)
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

/// Convert screen position to grid coordinates
///
/// # Arguments
/// * `screen_pos` - Position in screen/world space (2D)
/// * `tile_size` - Size of each tile in world units
///
/// Returns the nearest grid position
pub fn screen_to_grid(screen_pos: Vec2, tile_size: f32) -> IVec2 {
    // Convert 2D screen position to 3D world position (z=0)
    let world_pos = Vec3::new(screen_pos.x, screen_pos.y, 0.0);
    let (x, y, _) = world_to_grid(world_pos, tile_size);
    IVec2::new(x, y)
}

/// Calculate the center of the grid in world space
pub fn grid_center_world(width: i32, height: i32, tile_size: f32) -> Vec3 {
    grid_to_world(width / 2, height / 2, 0, tile_size)
}

/// Get the world-space bounding box of a single tile
///
/// IMPORTANT: This calculates the diamond-shaped bounds of an isometric tile!
/// Each tile is a diamond (rhombus) in world space, NOT a square.
///
/// Example for tile (1,0) with tile_size=64:
/// - Center: world_x = (1-0)*64*0.5 = 32, world_y = -(1+0)*64*0.25 = -16
/// - Diamond vertices:
///   - Top: (32, -16 + 16) = (32, 0)
///   - Right: (32 + 32, -16) = (64, -16)
///   - Bottom: (32, -16 - 16) = (32, -32)
///   - Left: (32 - 32, -16) = (0, -16)
pub fn get_tile_world_bounds(x: i32, y: i32, tile_size: f32) -> (Vec3, Vec3) {
    let center = grid_to_world(x, y, 0, tile_size);

    // For a diamond tile in isometric view:
    // - Width is tile_size (distance from left vertex to right vertex)
    // - Height is tile_size * 0.5 (distance from top vertex to bottom vertex)
    let half_width = tile_size * 0.5;
    let half_height = tile_size * 0.25;

    // AABB (Axis-Aligned Bounding Box) that contains the diamond
    let min = Vec3::new(center.x - half_width, center.y - half_height, 0.0);
    let max = Vec3::new(center.x + half_width, center.y + half_height, 0.0);

    (min, max)
}

/// Check if a tile's world bounds intersect with a rectangle
///
/// This performs an AABB intersection test between:
/// - The tile's bounding box (which contains its diamond shape)
/// - The camera's visible rectangle
///
/// NOTE: This is conservative - it may return true for tiles whose
/// bounding box intersects but whose actual diamond doesn't.
/// This is fine for culling (better to render too much than too little).
pub fn tile_intersects_rect(
    tile_x: i32,
    tile_y: i32,
    tile_size: f32,
    rect_min: Vec2,
    rect_max: Vec2,
) -> bool {
    let (tile_min, tile_max) = get_tile_world_bounds(tile_x, tile_y, tile_size);

    // AABB intersection test: rectangles DON'T intersect if:
    // - tile is completely to the left of rect (tile_max.x < rect_min.x)
    // - tile is completely to the right of rect (tile_min.x > rect_max.x)
    // - tile is completely below rect (tile_max.y < rect_min.y)
    // - tile is completely above rect (tile_min.y > rect_max.y)
    //
    // They DO intersect if none of these are true (hence the NOT)
    !(tile_max.x < rect_min.x
        || tile_min.x > rect_max.x
        || tile_max.y < rect_min.y
        || tile_min.y > rect_max.y)
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

    #[test]
    fn test_screen_to_grid_basic() {
        let tile_size = 64.0;

        // Test origin
        let grid = screen_to_grid(Vec2::new(0.0, 0.0), tile_size);
        assert_eq!(grid, IVec2::new(0, 0));

        // Test basic positions
        let grid = screen_to_grid(Vec2::new(32.0, -16.0), tile_size);
        assert_eq!(grid, IVec2::new(1, 0));

        let grid = screen_to_grid(Vec2::new(-32.0, -16.0), tile_size);
        assert_eq!(grid, IVec2::new(0, 1));
    }

    #[test]
    fn test_screen_to_grid_consistency() {
        let tile_size = 64.0;

        // Test that screen_to_grid is consistent with grid_to_isometric
        for x in -5..=5 {
            for y in -5..=5 {
                let iso_pos = grid_to_isometric(x, y, tile_size);
                // Convert isometric Y to world Y (negate it)
                let screen_pos = Vec2::new(iso_pos.x, -iso_pos.y);
                let grid_pos = screen_to_grid(screen_pos, tile_size);
                assert_eq!(grid_pos, IVec2::new(x, y));
            }
        }
    }

    #[test]
    fn test_screen_to_grid_rounding() {
        let tile_size = 64.0;

        // Test rounding behavior
        let grid = screen_to_grid(Vec2::new(30.0, -15.0), tile_size);
        assert_eq!(grid, IVec2::new(1, 0));

        let grid = screen_to_grid(Vec2::new(34.0, -17.0), tile_size);
        assert_eq!(grid, IVec2::new(1, 0));

        // Test negative coordinates
        let grid = screen_to_grid(Vec2::new(-30.0, 15.0), tile_size);
        assert_eq!(grid, IVec2::new(-1, 0));
    }

    #[test]
    fn test_get_tile_world_bounds() {
        let tile_size = 64.0;

        // Test tile at origin
        let (min, max) = get_tile_world_bounds(0, 0, tile_size);
        assert_eq!(min, Vec3::new(-32.0, -16.0, 0.0));
        assert_eq!(max, Vec3::new(32.0, 16.0, 0.0));

        // Test tile at (1, 0)
        let (min, max) = get_tile_world_bounds(1, 0, tile_size);
        let center = grid_to_world(1, 0, 0, tile_size);
        assert_eq!(center, Vec3::new(32.0, -16.0, 0.0));
        assert_eq!(min, Vec3::new(0.0, -32.0, 0.0));
        assert_eq!(max, Vec3::new(64.0, 0.0, 0.0));

        // Test tile at (0, 1)
        let (min, max) = get_tile_world_bounds(0, 1, tile_size);
        let center = grid_to_world(0, 1, 0, tile_size);
        assert_eq!(center, Vec3::new(-32.0, -16.0, 0.0));
        assert_eq!(min, Vec3::new(-64.0, -32.0, 0.0));
        assert_eq!(max, Vec3::new(0.0, 0.0, 0.0));

        // Test with different tile size
        let tile_size = 100.0;
        let (min, max) = get_tile_world_bounds(2, 3, tile_size);
        let center = grid_to_world(2, 3, 0, tile_size);
        assert_eq!(min, Vec3::new(center.x - 50.0, center.y - 25.0, 0.0));
        assert_eq!(max, Vec3::new(center.x + 50.0, center.y + 25.0, 0.0));
    }

    #[test]
    fn test_tile_intersects_rect() {
        let tile_size = 64.0;

        // Test tile at origin with rectangle centered at origin
        let rect_min = Vec2::new(-50.0, -50.0);
        let rect_max = Vec2::new(50.0, 50.0);
        assert!(tile_intersects_rect(0, 0, tile_size, rect_min, rect_max));

        // Test tile completely outside rectangle
        let rect_min = Vec2::new(100.0, 100.0);
        let rect_max = Vec2::new(200.0, 200.0);
        assert!(!tile_intersects_rect(0, 0, tile_size, rect_min, rect_max));

        // Test tile partially overlapping rectangle
        let rect_min = Vec2::new(20.0, -20.0);
        let rect_max = Vec2::new(100.0, 100.0);
        assert!(tile_intersects_rect(1, 0, tile_size, rect_min, rect_max));

        // Test edge cases - rectangle touching tile bounds
        let (tile_min, tile_max) = get_tile_world_bounds(5, 5, tile_size);

        // Rectangle just outside right edge (not touching)
        let rect_min = Vec2::new(tile_max.x + 0.1, tile_min.y);
        let rect_max = Vec2::new(tile_max.x + 10.0, tile_max.y);
        assert!(!tile_intersects_rect(5, 5, tile_size, rect_min, rect_max));

        // Rectangle overlapping by 1 pixel
        let rect_min = Vec2::new(tile_max.x - 1.0, tile_min.y);
        let rect_max = Vec2::new(tile_max.x + 10.0, tile_max.y);
        assert!(tile_intersects_rect(5, 5, tile_size, rect_min, rect_max));
    }

    #[test]
    fn test_tile_intersects_rect_camera_view() {
        let tile_size = 64.0;

        // Simulate camera at center of map looking at origin
        // Camera sees area from (-640, -360) to (640, 360) - typical 1280x720 window
        let camera_min = Vec2::new(-640.0, -360.0);
        let camera_max = Vec2::new(640.0, 360.0);

        // Tiles near origin should be visible
        assert!(tile_intersects_rect(
            0, 0, tile_size, camera_min, camera_max
        ));
        assert!(tile_intersects_rect(
            1, 0, tile_size, camera_min, camera_max
        ));
        assert!(tile_intersects_rect(
            0, 1, tile_size, camera_min, camera_max
        ));
        assert!(tile_intersects_rect(
            -1, 0, tile_size, camera_min, camera_max
        ));
        assert!(tile_intersects_rect(
            0, -1, tile_size, camera_min, camera_max
        ));

        // Far away tiles should not be visible
        assert!(!tile_intersects_rect(
            50, 50, tile_size, camera_min, camera_max
        ));
        assert!(!tile_intersects_rect(
            -50, -50, tile_size, camera_min, camera_max
        ));
    }

    #[test]
    fn test_bounds_at_map_corners() {
        let tile_size = 64.0;

        // Test the four corners of a 200x200 map
        let corners = [(0, 0), (199, 0), (0, 199), (199, 199)];

        for (x, y) in corners {
            let (min, max) = get_tile_world_bounds(x, y, tile_size);
            let center = grid_to_world(x, y, 0, tile_size);

            // Verify bounds are centered correctly
            assert_eq!((min.x + max.x) / 2.0, center.x);
            assert_eq!((min.y + max.y) / 2.0, center.y);

            // Verify bounds have correct size
            assert_eq!(max.x - min.x, tile_size);
            assert_eq!(max.y - min.y, tile_size * 0.5);
        }
    }
}
