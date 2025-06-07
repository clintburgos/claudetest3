//! Test the new culling algorithm specifically for corner visibility

use bevy::prelude::*;
use claudetest3::ui::world::grid::coordinates::{grid_to_world, tile_intersects_rect};

#[test]
fn test_culling_at_minimum_zoom() {
    // Test parameters for 200x200 map
    let grid_width = 200;
    let grid_height = 200;
    let tile_size = 64.0;

    // Window size: 1280x720
    let window_width = 1280.0;
    let window_height = 720.0;

    // Minimum zoom for this configuration (from the design doc)
    let min_zoom = 0.0838;

    // Camera at center of map
    let map_center = grid_to_world(grid_width / 2, grid_height / 2, 0, tile_size);

    // Calculate visible world bounds at minimum zoom
    let visible_width = window_width / min_zoom;
    let visible_height = window_height / min_zoom;

    let visible_min = Vec2::new(
        map_center.x - visible_width * 0.5,
        map_center.y - visible_height * 0.5,
    );
    let visible_max = Vec2::new(
        map_center.x + visible_width * 0.5,
        map_center.y + visible_height * 0.5,
    );

    println!("Test Configuration:");
    println!(
        "  Map: {}x{} tiles, tile_size={}",
        grid_width, grid_height, tile_size
    );
    println!("  Window: {}x{}", window_width, window_height);
    println!("  Min zoom: {:.4}", min_zoom);
    println!("  Camera at: ({:.1}, {:.1})", map_center.x, map_center.y);
    println!(
        "  Visible world: ({:.1}, {:.1}) to ({:.1}, {:.1})",
        visible_min.x, visible_min.y, visible_max.x, visible_max.y
    );

    // Test all four corners
    let corners = [
        (0, 0, "Bottom-Left"),
        (199, 0, "Bottom-Right"),
        (0, 199, "Top-Left"),
        (199, 199, "Top-Right"),
    ];

    let mut all_visible = true;

    for (x, y, name) in corners {
        let world_pos = grid_to_world(x, y, 0, tile_size);
        let is_visible = tile_intersects_rect(x, y, tile_size, visible_min, visible_max);

        println!("\n{} corner ({},{})", name, x, y);
        println!("  World position: ({:.1}, {:.1})", world_pos.x, world_pos.y);
        println!("  Visible: {}", is_visible);

        if !is_visible {
            all_visible = false;
            eprintln!("  ❌ FAILED: Corner should be visible at minimum zoom!");
        }
    }

    assert!(all_visible, "All corners should be visible at minimum zoom");
    println!("\n✅ All corners are visible at minimum zoom!");
}

#[test]
fn test_map_bounds_calculation() {
    let tile_size = 64.0;

    // Test map corners world positions
    let corners = [(0, 0), (199, 0), (0, 199), (199, 199)];

    println!("Map corner world positions:");
    for (x, y) in corners {
        let pos = grid_to_world(x, y, 0, tile_size);
        println!("  Grid ({},{}) -> World ({:.1}, {:.1})", x, y, pos.x, pos.y);
    }

    // Calculate overall map bounds
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;

    for (x, y) in corners {
        let pos = grid_to_world(x, y, 0, tile_size);
        // Account for tile size
        min_x = min_x.min(pos.x - tile_size * 0.5);
        max_x = max_x.max(pos.x + tile_size * 0.5);
        min_y = min_y.min(pos.y - tile_size * 0.25);
        max_y = max_y.max(pos.y + tile_size * 0.25);
    }

    println!("\nMap world bounds:");
    println!(
        "  X: {:.1} to {:.1} (width: {:.1})",
        min_x,
        max_x,
        max_x - min_x
    );
    println!(
        "  Y: {:.1} to {:.1} (height: {:.1})",
        min_y,
        max_y,
        max_y - min_y
    );

    // Verify these match the expected values from the design doc
    // The actual values are slightly different due to including tile bounds
    assert!(
        (max_x - min_x - 12800.0).abs() < 1.0,
        "Map width should be ~12800"
    );
    assert!(
        (max_y - min_y - 6400.0).abs() < 1.0,
        "Map height should be ~6400"
    );
}

#[test]
fn test_tile_visibility_at_various_zooms() {
    let tile_size = 64.0;
    let window_width = 1280.0;
    let window_height = 720.0;

    // Test at various zoom levels
    let zoom_levels = [0.084, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];

    // Camera at center of map
    let camera_pos = grid_to_world(100, 100, 0, tile_size);

    println!("Testing tile visibility at various zoom levels:");
    println!("Camera at: ({:.1}, {:.1})", camera_pos.x, camera_pos.y);

    for &zoom in &zoom_levels {
        let visible_width = window_width / zoom;
        let visible_height = window_height / zoom;

        let visible_min = Vec2::new(
            camera_pos.x - visible_width * 0.5,
            camera_pos.y - visible_height * 0.5,
        );
        let visible_max = Vec2::new(
            camera_pos.x + visible_width * 0.5,
            camera_pos.y + visible_height * 0.5,
        );

        // Count visible tiles
        let mut visible_count = 0;
        for y in 0..200 {
            for x in 0..200 {
                if tile_intersects_rect(x, y, tile_size, visible_min, visible_max) {
                    visible_count += 1;
                }
            }
        }

        println!("\nZoom {:.3}: {} tiles visible", zoom, visible_count);

        if zoom <= 0.084 {
            assert_eq!(
                visible_count, 40000,
                "All tiles should be visible at minimum zoom"
            );
        }
    }
}
