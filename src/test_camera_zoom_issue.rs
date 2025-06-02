use bevy::prelude::*;

/// This test demonstrates the camera zoom issue where:
/// 1. Zooming out makes map edges recede
/// 2. Zooming in makes the map disappear
///
/// The root cause is that camera scale and zoom have an inverted relationship
/// in the view culling calculation.
pub fn test_camera_zoom_issue() {
    println!("\n=== CAMERA ZOOM ISSUE DEMONSTRATION ===\n");

    // Test parameters
    let window_width = 1280.0;
    let window_height = 720.0;
    let tile_size = 32.0;
    let map_size = 100; // 100x100 tiles
    let map_width_pixels = map_size as f32 * tile_size;
    let map_height_pixels = map_size as f32 * tile_size;

    println!("Setup:");
    println!("  Window: {}x{}", window_width, window_height);
    println!(
        "  Map: {}x{} tiles ({:.0}x{:.0} pixels)",
        map_size, map_size, map_width_pixels, map_height_pixels
    );
    println!("  Tile size: {}x{} pixels", tile_size, tile_size);

    println!("\n=== CURRENT BEHAVIOR (INVERTED) ===");

    // Test different zoom levels with current inverted calculation
    let zoom_levels = vec![
        (0.25, "Very zoomed in"),
        (0.5, "Zoomed in"),
        (1.0, "Default"),
        (2.0, "Zoomed out"),
        (4.0, "Very zoomed out"),
    ];

    for (zoom, description) in &zoom_levels {
        println!("\n{} (zoom = {}):", description, zoom);

        // Current calculation (INVERTED - dividing by scale)
        let visible_width = window_width / zoom;
        let visible_height = window_height / zoom;

        println!(
            "  Visible area: {:.0}x{:.0} pixels",
            visible_width, visible_height
        );
        println!(
            "  Visible tiles: {:.1}x{:.1}",
            visible_width / tile_size,
            visible_height / tile_size
        );

        // Check if entire map fits in view
        if visible_width >= map_width_pixels && visible_height >= map_height_pixels {
            println!("  ⚠️  ENTIRE MAP FITS IN VIEW - edges will recede!");
            let empty_space_x = visible_width - map_width_pixels;
            let empty_space_y = visible_height - map_height_pixels;
            println!(
                "  Empty space: {:.0}x{:.0} pixels",
                empty_space_x, empty_space_y
            );
        }

        // Check if view is smaller than a single tile
        if visible_width < tile_size || visible_height < tile_size {
            println!("  ❌ VIEW SMALLER THAN TILE SIZE - tiles may disappear!");
        }
    }

    println!("\n=== EXPECTED BEHAVIOR (CORRECT) ===");

    for (zoom, description) in &zoom_levels {
        println!("\n{} (zoom = {}):", description, zoom);

        // Correct calculation - multiplying by scale
        let visible_width = window_width * zoom;
        let visible_height = window_height * zoom;

        println!(
            "  Visible area: {:.0}x{:.0} pixels",
            visible_width, visible_height
        );
        println!(
            "  Visible tiles: {:.1}x{:.1}",
            visible_width / tile_size,
            visible_height / tile_size
        );

        // In correct behavior, zooming in should show fewer tiles
        if *zoom < 1.0 {
            println!("  ✓ Zoomed in - showing fewer tiles (closer view)");
        } else if *zoom > 1.0 {
            println!("  ✓ Zoomed out - showing more tiles (wider view)");
        }
    }

    println!("\n=== ANALYSIS ===");
    println!("\nThe issue is in calculate_visible_bounds():");
    println!("  CURRENT: visible_width = window.width() / camera_scale");
    println!("  CORRECT: visible_width = window.width() * camera_scale");
    println!("\nThis inversion causes:");
    println!("1. When zooming IN (scale > 1), visible area DECREASES (should INCREASE)");
    println!("2. When zooming OUT (scale < 1), visible area INCREASES (should DECREASE)");
    println!("\nEffects:");
    println!("- Zooming out too far makes the entire map fit, causing edges to recede");
    println!("- Zooming in too far makes visible area tiny, causing tiles to disappear");

    println!("\n=== SOLUTION ===");
    println!("Fix the calculation in calculate_visible_bounds to multiply instead of divide.");
    println!("This will make zoom behave intuitively:");
    println!("- Zoom in = see less of the world but in more detail");
    println!("- Zoom out = see more of the world but in less detail");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_camera_zoom_test() {
        test_camera_zoom_issue();
    }
}
