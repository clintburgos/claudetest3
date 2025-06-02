//! Debug isometric coordinates to understand the issue

use bevy::prelude::*;
use claudetest3::ui::world::grid::coordinates::{grid_to_world, world_to_grid};

fn main() {
    let tile_size = 64.0;
    let grid_size = 200;
    
    println!("=== Isometric Coordinate Debug ===");
    println!("Grid size: {}x{}, Tile size: {}", grid_size, grid_size, tile_size);
    
    // Test key positions
    let positions = vec![
        (0, 0, "Origin (Bottom-Left)"),
        (199, 0, "Bottom-Right"),
        (0, 199, "Top-Left"),
        (199, 199, "Top-Right"),
        (100, 100, "Center"),
    ];
    
    println!("\nGrid -> World conversions:");
    for (x, y, name) in &positions {
        let world = grid_to_world(*x, *y, 0, tile_size);
        println!("{:20} ({:3}, {:3}) -> ({:7.1}, {:7.1})", 
                 name, x, y, world.x, world.y);
    }
    
    // Calculate world bounds
    let corners = vec![
        grid_to_world(0, 0, 0, tile_size),
        grid_to_world(199, 0, 0, tile_size),
        grid_to_world(0, 199, 0, tile_size),
        grid_to_world(199, 199, 0, tile_size),
    ];
    
    let min_x = corners.iter().map(|v| v.x).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_x = corners.iter().map(|v| v.x).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let min_y = corners.iter().map(|v| v.y).min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    let max_y = corners.iter().map(|v| v.y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
    
    println!("\nWorld bounds:");
    println!("X: {:.1} to {:.1} (width: {:.1})", min_x, max_x, max_x - min_x);
    println!("Y: {:.1} to {:.1} (height: {:.1})", min_y, max_y, max_y - min_y);
    
    // Test camera visibility at different scales
    let window_width = 1280.0;
    let window_height = 720.0;
    let camera_pos = Vec3::new(0.0, -3200.0, 0.0);
    
    println!("\nCamera at ({:.1}, {:.1}):", camera_pos.x, camera_pos.y);
    for scale in [0.084, 0.1, 0.5, 1.0, 2.0] {
        let visible_width = window_width / scale;
        let visible_height = window_height / scale;
        
        let left = camera_pos.x - visible_width * 0.5;
        let right = camera_pos.x + visible_width * 0.5;
        let bottom = camera_pos.y - visible_height * 0.5;
        let top = camera_pos.y + visible_height * 0.5;
        
        // Check which corners are visible
        let mut visible_corners = vec![];
        for (i, corner) in corners.iter().enumerate() {
            if corner.x >= left && corner.x <= right && corner.y >= bottom && corner.y <= top {
                visible_corners.push(i);
            }
        }
        
        // Convert visible bounds to grid
        let (min_grid_x, min_grid_y, _) = world_to_grid(Vec3::new(left, bottom, 0.0), tile_size);
        let (max_grid_x, max_grid_y, _) = world_to_grid(Vec3::new(right, top, 0.0), tile_size);
        
        println!("\nScale: {:.3}", scale);
        println!("  Visible world: ({:.0}, {:.0}) to ({:.0}, {:.0})", left, bottom, right, top);
        println!("  Visible size: {:.0} x {:.0}", visible_width, visible_height);
        println!("  Grid range estimate: ({}, {}) to ({}, {})", min_grid_x, min_grid_y, max_grid_x, max_grid_y);
        println!("  Corners visible: {} of 4", visible_corners.len());
        
        if scale == 0.084 {
            println!("\n  At minimum zoom (0.084):");
            println!("  Need visible area >= {:.0} x {:.0} to see entire map", max_x - min_x, max_y - min_y);
            println!("  Current visible area: {:.0} x {:.0}", visible_width, visible_height);
            if visible_width < (max_x - min_x) || visible_height < (max_y - min_y) {
                println!("  ⚠️  NOT ENOUGH - Can't see entire map!");
            }
        }
    }
}