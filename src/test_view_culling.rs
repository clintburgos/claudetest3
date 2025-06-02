use bevy::prelude::*;

pub fn test_view_culling_debug(_commands: Commands) {
    // Test different camera scales and viewport sizes
    let test_cases = vec![
        // (camera_scale, viewport_width, viewport_height, description)
        (Vec2::new(1.0, 1.0), 1280.0, 720.0, "Default scale"),
        (Vec2::new(0.5, 0.5), 1280.0, 720.0, "Zoomed in (0.5x)"),
        (Vec2::new(2.0, 2.0), 1280.0, 720.0, "Zoomed out (2x)"),
        (Vec2::new(4.0, 4.0), 1280.0, 720.0, "Very zoomed out (4x)"),
        (
            Vec2::new(0.25, 0.25),
            1280.0,
            720.0,
            "Very zoomed in (0.25x)",
        ),
    ];

    println!("\n=== VIEW CULLING DEBUG TEST ===\n");

    for (scale, width, height, description) in test_cases {
        println!(
            "Test case: {} - Scale: {:?}, Viewport: {}x{}",
            description, scale, width, height
        );

        // Test with camera at origin
        let camera_pos = Vec2::ZERO;
        println!("  Camera at origin (0, 0):");
        debug_visible_bounds(camera_pos, scale, width, height);

        // Test with camera offset
        let camera_pos = Vec2::new(500.0, 300.0);
        println!("  Camera at (500, 300):");
        debug_visible_bounds(camera_pos, scale, width, height);

        println!();
    }

    // Test edge cases
    println!("=== EDGE CASES ===\n");

    // Camera at map boundary
    let map_size = 100.0 * 32.0; // Assuming 100x100 tiles, 32 pixels each
    let camera_pos = Vec2::new(map_size / 2.0, map_size / 2.0);
    println!("Camera at map edge ({}, {}):", camera_pos.x, camera_pos.y);
    debug_visible_bounds(camera_pos, Vec2::new(2.0, 2.0), 1280.0, 720.0);

    println!("\n=== ANALYSIS ===");
    println!("When camera scale increases (zooming out):");
    println!("- The visible world area increases");
    println!("- More of the world fits in the viewport");
    println!("- If the entire map fits within the viewport, edges appear to recede");
    println!("\nWhen camera scale decreases (zooming in):");
    println!("- The visible world area decreases");
    println!("- Less of the world fits in the viewport");
    println!("- Tiles may appear larger than the viewport, causing disappearance");
}

fn debug_visible_bounds(
    camera_pos: Vec2,
    camera_scale: Vec2,
    viewport_width: f32,
    viewport_height: f32,
) {
    // Step 1: Calculate half dimensions of the visible area in world space
    let half_width = (viewport_width / 2.0) * camera_scale.x;
    let half_height = (viewport_height / 2.0) * camera_scale.y;

    println!("    Step 1 - Half dimensions in world space:");
    println!(
        "      half_width = (viewport_width / 2) * scale.x = ({} / 2) * {} = {}",
        viewport_width, camera_scale.x, half_width
    );
    println!(
        "      half_height = (viewport_height / 2) * scale.y = ({} / 2) * {} = {}",
        viewport_height, camera_scale.y, half_height
    );

    // Step 2: Calculate bounds
    let min_x = camera_pos.x - half_width;
    let max_x = camera_pos.x + half_width;
    let min_y = camera_pos.y - half_height;
    let max_y = camera_pos.y + half_height;

    println!("    Step 2 - World bounds:");
    println!(
        "      min_x = camera.x - half_width = {} - {} = {}",
        camera_pos.x, half_width, min_x
    );
    println!(
        "      max_x = camera.x + half_width = {} + {} = {}",
        camera_pos.x, half_width, max_x
    );
    println!(
        "      min_y = camera.y - half_height = {} - {} = {}",
        camera_pos.y, half_height, min_y
    );
    println!(
        "      max_y = camera.y + half_height = {} + {} = {}",
        camera_pos.y, half_height, max_y
    );

    // Step 3: Calculate visible area
    let visible_width = max_x - min_x;
    let visible_height = max_y - min_y;
    let visible_area = visible_width * visible_height;

    println!("    Step 3 - Visible area:");
    println!("      width = {} - {} = {}", max_x, min_x, visible_width);
    println!("      height = {} - {} = {}", max_y, min_y, visible_height);
    println!(
        "      area = {} * {} = {}",
        visible_width, visible_height, visible_area
    );

    // Step 4: Calculate tile range (assuming 32x32 tiles)
    let tile_size = 32.0;
    let min_tile_x = (min_x / tile_size).floor() as i32;
    let max_tile_x = (max_x / tile_size).ceil() as i32;
    let min_tile_y = (min_y / tile_size).floor() as i32;
    let max_tile_y = (max_y / tile_size).ceil() as i32;

    println!("    Step 4 - Tile range (32x32 tiles):");
    println!(
        "      X: {} to {} ({} tiles)",
        min_tile_x,
        max_tile_x,
        max_tile_x - min_tile_x
    );
    println!(
        "      Y: {} to {} ({} tiles)",
        min_tile_y,
        max_tile_y,
        max_tile_y - min_tile_y
    );
    println!(
        "      Total tiles in view: {}",
        (max_tile_x - min_tile_x) * (max_tile_y - min_tile_y)
    );
}

// Plugin to run the test
pub struct ViewCullingDebugPlugin;

impl Plugin for ViewCullingDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, test_view_culling_debug);
    }
}
