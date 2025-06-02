//! Calculate the scale needed to view the entire map

use claudetest3::ui::world::grid::coordinates::grid_to_world;

fn main() {
    let grid_width = 200;
    let grid_height = 200;
    let tile_size = 64.0;
    let window_width = 1280.0;
    let window_height = 720.0;

    // Get world bounds of the entire map
    let bottom_left = grid_to_world(0, 0, 0, tile_size);
    let top_right = grid_to_world(grid_width - 1, grid_height - 1, 0, tile_size);
    let bottom_right = grid_to_world(grid_width - 1, 0, 0, tile_size);
    let top_left = grid_to_world(0, grid_height - 1, 0, tile_size);

    println!("World bounds:");
    println!(
        "  Bottom-left (0,0): ({:.1}, {:.1})",
        bottom_left.x, bottom_left.y
    );
    println!(
        "  Top-right ({},{})

: ({:.1}, {:.1})",
        grid_width - 1,
        grid_height - 1,
        top_right.x,
        top_right.y
    );
    println!(
        "  Bottom-right: ({:.1}, {:.1})",
        bottom_right.x, bottom_right.y
    );
    println!("  Top-left: ({:.1}, {:.1})", top_left.x, top_left.y);

    // Calculate the world dimensions needed
    let world_min_x = bottom_left
        .x
        .min(top_right.x)
        .min(bottom_right.x)
        .min(top_left.x);
    let world_max_x = bottom_left
        .x
        .max(top_right.x)
        .max(bottom_right.x)
        .max(top_left.x);
    let world_min_y = bottom_left
        .y
        .min(top_right.y)
        .min(bottom_right.y)
        .min(top_left.y);
    let world_max_y = bottom_left
        .y
        .max(top_right.y)
        .max(bottom_right.y)
        .max(top_left.y);

    let world_width = world_max_x - world_min_x;
    let world_height = world_max_y - world_min_y;

    println!("\nWorld dimensions:");
    println!("  Width: {:.1}", world_width);
    println!("  Height: {:.1}", world_height);

    // Calculate scale needed
    let scale_for_width = window_width / world_width;
    let scale_for_height = window_height / world_height;
    let scale_needed = scale_for_width.min(scale_for_height);

    println!("\nScale calculation:");
    println!("  Scale for width: {:.3}", scale_for_width);
    println!("  Scale for height: {:.3}", scale_for_height);
    println!("  Scale needed: {:.3}", scale_needed);

    // Test if we can see everything at min zoom
    let min_zoom = 0.5;
    let visible_at_min = window_width / min_zoom;
    println!("\nAt min zoom ({}):", min_zoom);
    println!("  Visible width: {:.1}", visible_at_min);
    println!(
        "  Can see entire map: {}",
        if visible_at_min >= world_width {
            "YES"
        } else {
            "NO"
        }
    );
}
