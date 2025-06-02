//! Debug the culling issue with zoom
//!
//! This example helps debug why the map edges recede when zooming out

use bevy::prelude::*;
use claudetest3::ui::world::grid::{
    coordinates::{grid_to_world, world_to_grid},
    GridConfig,
};

fn main() {
    let grid_config = GridConfig {
        width: 200,
        height: 200,
        tile_size: 64.0,
    };

    // Test with different zoom levels
    let window_width = 1280.0;
    let window_height = 720.0;

    println!(
        "Grid: {}x{}, tile_size: {}",
        grid_config.width, grid_config.height, grid_config.tile_size
    );
    println!("Window: {}x{}\n", window_width, window_height);

    // Test zoom out
    let scales = vec![0.3, 0.5, 1.0, 1.5, 2.0];

    for scale in scales {
        println!("=== Scale: {} ===", scale);

        // Camera at center of map
        let center = grid_to_world(
            grid_config.width / 2,
            grid_config.height / 2,
            0,
            grid_config.tile_size,
        );
        println!("Camera at center: ({:.1}, {:.1})", center.x, center.y);

        // Calculate visible world area
        let visible_width = window_width / scale;
        let visible_height = window_height / scale;
        println!(
            "Visible area: {:.1} x {:.1} world units",
            visible_width, visible_height
        );

        // Calculate world bounds
        let left = center.x - visible_width * 0.5;
        let right = center.x + visible_width * 0.5;
        let bottom = center.y - visible_height * 0.5;
        let top = center.y + visible_height * 0.5;

        // Convert to grid coordinates
        let (min_x_1, min_y_1, _) =
            world_to_grid(Vec3::new(left, bottom, 0.0), grid_config.tile_size);
        let (max_x_1, max_y_1, _) =
            world_to_grid(Vec3::new(right, top, 0.0), grid_config.tile_size);
        let (min_x_2, min_y_2, _) = world_to_grid(Vec3::new(left, top, 0.0), grid_config.tile_size);
        let (max_x_2, max_y_2, _) =
            world_to_grid(Vec3::new(right, bottom, 0.0), grid_config.tile_size);

        // Get bounds
        let min_x = min_x_1.min(min_x_2).min(max_x_1).min(max_x_2);
        let max_x = min_x_1.max(min_x_2).max(max_x_1).max(max_x_2);
        let min_y = min_y_1.min(min_y_2).min(max_y_1).min(max_y_2);
        let max_y = min_y_1.max(min_y_2).max(max_y_1).max(max_y_2);

        println!(
            "Raw bounds: ({}, {}) to ({}, {})",
            min_x, min_y, max_x, max_y
        );

        // Clamp
        let min_x = min_x.max(0);
        let max_x = max_x.min(grid_config.width - 1);
        let min_y = min_y.max(0);
        let max_y = max_y.min(grid_config.height - 1);

        println!(
            "Clamped bounds: ({}, {}) to ({}, {})",
            min_x, min_y, max_x, max_y
        );
        println!(
            "Visible tiles: {}x{} = {} tiles",
            max_x - min_x + 1,
            max_y - min_y + 1,
            (max_x - min_x + 1) * (max_y - min_y + 1)
        );

        // Check if entire map should be visible
        if min_x == 0
            && max_x == grid_config.width - 1
            && min_y == 0
            && max_y == grid_config.height - 1
        {
            println!("ENTIRE MAP VISIBLE");
        }

        println!();
    }
}
