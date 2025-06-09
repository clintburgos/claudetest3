//! Test program to verify culling bounds calculation
//! 
//! This program tests the isometric culling algorithm to ensure it correctly
//! identifies all visible tiles.

use bevy::prelude::*;
use claudetest3::ui::world::{
    grid::coordinates::*,
    tiles::isometric_culling::calculate_isometric_visible_tiles,
};

fn main() {
    // Test parameters
    let camera_pos = Vec3::new(0.0, -3200.0, 1000.0);
    let window_size = Vec2::new(1280.0, 720.0);
    let camera_zoom = 1.0;
    let tile_size = 64.0;
    let grid_width = 200;
    let grid_height = 200;
    let buffer = 10;
    
    println!("=== Culling Bounds Test ===");
    println!("Camera position: {:?}", camera_pos);
    println!("Window size: {}x{}", window_size.x, window_size.y);
    println!("Camera zoom: {}", camera_zoom);
    println!("Tile size: {}", tile_size);
    println!("Grid size: {}x{}", grid_width, grid_height);
    println!("Buffer: {}", buffer);
    println!();
    
    // Calculate camera grid position
    let (cam_x, cam_y, _) = world_to_grid(camera_pos, tile_size);
    println!("Camera grid position: ({}, {})", cam_x, cam_y);
    println!();
    
    // Calculate visible bounds
    let (min_x, min_y, max_x, max_y) = calculate_isometric_visible_tiles(
        camera_pos,
        window_size,
        camera_zoom,
        tile_size,
        grid_width,
        grid_height,
        buffer,
    );
    
    println!("Visible bounds: ({}, {}) to ({}, {})", min_x, min_y, max_x, max_y);
    println!("Visible area: {}x{} tiles", max_x - min_x + 1, max_y - min_y + 1);
    println!("Total visible tiles: {}", (max_x - min_x + 1) * (max_y - min_y + 1));
    println!();
    
    // Calculate the screen bounds in world space
    let half_width = window_size.x / camera_zoom * 0.5;
    let half_height = window_size.y / camera_zoom * 0.5;
    
    println!("Screen bounds in world space:");
    println!("  Left: {:.1}", camera_pos.x - half_width);
    println!("  Right: {:.1}", camera_pos.x + half_width);
    println!("  Top: {:.1}", camera_pos.y + half_height);
    println!("  Bottom: {:.1}", camera_pos.y - half_height);
    println!();
    
    // Test the four corners of the screen
    let corners = [
        ("Bottom-left", Vec3::new(camera_pos.x - half_width, camera_pos.y - half_height, 0.0)),
        ("Bottom-right", Vec3::new(camera_pos.x + half_width, camera_pos.y - half_height, 0.0)),
        ("Top-right", Vec3::new(camera_pos.x + half_width, camera_pos.y + half_height, 0.0)),
        ("Top-left", Vec3::new(camera_pos.x - half_width, camera_pos.y + half_height, 0.0)),
    ];
    
    println!("Screen corners in grid space:");
    for (name, corner) in &corners {
        let (gx, gy, _) = world_to_grid(*corner, tile_size);
        println!("  {}: world({:.1}, {:.1}) -> grid({}, {})", name, corner.x, corner.y, gx, gy);
    }
    println!();
    
    // Now let's check some specific tiles that should be visible
    println!("Checking tiles near camera center:");
    for dy in -2..=2 {
        for dx in -2..=2 {
            let tile_x = cam_x + dx;
            let tile_y = cam_y + dy;
            let world_pos = grid_to_world(tile_x, tile_y, 0, tile_size);
            let in_bounds = tile_x >= min_x && tile_x <= max_x && tile_y >= min_y && tile_y <= max_y;
            
            println!("  Tile ({}, {}): world({:.1}, {:.1}) - {}", 
                tile_x, tile_y, world_pos.x, world_pos.y,
                if in_bounds { "VISIBLE" } else { "CULLED" }
            );
        }
    }
    println!();
    
    // Check if the culling algorithm might be too aggressive
    println!("Analyzing culling buffer:");
    let base_buffer = 5;
    let zoom_buffer = if camera_zoom < 1.0 {
        let zoom_factor = 1.0 / camera_zoom;
        (base_buffer as f32 * zoom_factor.sqrt() * 2.0).ceil() as i32
    } else {
        base_buffer
    };
    let diagonal_buffer = (zoom_buffer as f32 * 1.5).ceil() as i32;
    
    println!("  Base buffer: {}", base_buffer);
    println!("  Zoom buffer: {}", zoom_buffer);
    println!("  Diagonal buffer: {}", diagonal_buffer);
    println!("  Total buffer applied: {}", zoom_buffer + diagonal_buffer);
}