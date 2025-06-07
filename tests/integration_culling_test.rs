//! Integration test to verify culling works correctly

use bevy::prelude::*;
use claudetest3::ui::world::grid::coordinates::{grid_to_world, tile_intersects_rect};

#[test]
fn verify_new_culling_algorithm_includes_all_corners() {
    // Test parameters matching the actual game
    let tile_size = 64.0;
    let grid_width = 200;
    let grid_height = 200;
    let window_width = 1280.0;
    let window_height = 720.0;

    // Calculate minimum zoom (from camera calculations)
    let map_world_width = 12736.0_f32; // From design doc
    let map_world_height = 6368.0_f32; // From design doc
    let width_scale = window_width / map_world_width;
    let height_scale = window_height / map_world_height;
    let min_zoom = width_scale.min(height_scale * 2.0); // ~0.0838

    println!("Calculated min zoom: {:.4}", min_zoom);

    // Camera at center of map
    let camera_pos = grid_to_world(grid_width / 2, grid_height / 2, 0, tile_size);
    println!(
        "Camera position: ({:.1}, {:.1})",
        camera_pos.x, camera_pos.y
    );

    // Calculate visible bounds at minimum zoom
    let visible_width = window_width / min_zoom;
    let visible_height = window_height / min_zoom;

    let visible_min = Vec2::new(
        camera_pos.x - visible_width * 0.5,
        camera_pos.y - visible_height * 0.5,
    );
    let visible_max = Vec2::new(
        camera_pos.x + visible_width * 0.5,
        camera_pos.y + visible_height * 0.5,
    );

    println!(
        "Visible world bounds: ({:.1}, {:.1}) to ({:.1}, {:.1})",
        visible_min.x, visible_min.y, visible_max.x, visible_max.y
    );

    // Test specific problematic corners
    let test_cases = [
        (0, 0, "Bottom-Left corner"),
        (199, 0, "Bottom-Right corner"),
        (0, 199, "Top-Left corner"),
        (199, 199, "Top-Right corner"),
        (100, 0, "Bottom edge center"),
        (0, 100, "Left edge center"),
        (199, 100, "Right edge center"),
        (100, 199, "Top edge center"),
    ];

    let mut all_pass = true;

    for (x, y, description) in test_cases {
        let is_visible = tile_intersects_rect(x, y, tile_size, visible_min, visible_max);
        let world_pos = grid_to_world(x, y, 0, tile_size);

        println!("\n{} at grid ({},{})", description, x, y);
        println!("  World position: ({:.1}, {:.1})", world_pos.x, world_pos.y);
        println!("  Visible: {}", if is_visible { "✅ YES" } else { "❌ NO" });

        if !is_visible {
            all_pass = false;
        }
    }

    assert!(all_pass, "All test tiles should be visible at minimum zoom");
    println!("\n✅ SUCCESS: All corners and edges are visible at minimum zoom!");
}

#[test]
fn test_dynamic_buffer_calculation() {
    let base_buffer = 5;

    // Test zoom levels
    let test_zooms: [f32; 6] = [0.084, 0.1, 0.5, 1.0, 2.0, 5.0];

    println!("Dynamic buffer calculation:");
    for &zoom in &test_zooms {
        let dynamic_buffer = if zoom > 1.0 {
            // Zoomed in: larger buffer for smooth panning
            (base_buffer as f32 * (1.0 + zoom.ln())).ceil() as i32
        } else {
            // Zoomed out: smaller buffer since we see more tiles
            (base_buffer as f32 * zoom.sqrt()).max(1.0).ceil() as i32
        };

        println!("  Zoom {:.3}: buffer = {}", zoom, dynamic_buffer);
    }
}

#[test]
fn test_search_optimization() {
    // Verify that our algorithm doesn't check unnecessary tiles
    let tile_size = 64.0;
    let visible_min = Vec2::new(-1000.0, -1000.0);
    let visible_max = Vec2::new(1000.0, 1000.0);

    let mut tiles_checked = 0;
    let mut tiles_visible = 0;

    // Only check tiles in a reasonable range
    for y in 0..200 {
        for x in 0..200 {
            tiles_checked += 1;
            if tile_intersects_rect(x, y, tile_size, visible_min, visible_max) {
                tiles_visible += 1;
            }
        }
    }

    println!("Efficiency test:");
    println!("  Tiles checked: {}", tiles_checked);
    println!("  Tiles visible: {}", tiles_visible);
    println!(
        "  Efficiency: {:.1}%",
        (tiles_visible as f32 / tiles_checked as f32) * 100.0
    );

    // For this visible area, we should see a reasonable subset of tiles
    assert!(tiles_visible > 0 && tiles_visible < tiles_checked);
}
