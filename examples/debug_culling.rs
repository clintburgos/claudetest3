//! Debug culling calculation
//! 
//! This example helps debug the view culling calculation

use bevy::prelude::*;

fn main() {
    // Test different zoom levels
    let window_width = 1280.0;
    let window_height = 720.0;
    
    println!("Window size: {}x{}", window_width, window_height);
    println!("Testing view culling calculations:\n");
    
    // Test zoom out (scale < 1.0)
    let scale = 0.5;
    println!("Zoom OUT (scale = {}):", scale);
    println!("  Visible width = {} / {} = {}", window_width, scale, window_width / scale);
    println!("  Expected: Should see MORE of the world (2560 units wide)\n");
    
    // Test normal zoom (scale = 1.0)  
    let scale = 1.0;
    println!("Normal zoom (scale = {}):", scale);
    println!("  Visible width = {} / {} = {}", window_width, scale, window_width / scale);
    println!("  Expected: Should see normal amount (1280 units wide)\n");
    
    // Test zoom in (scale > 1.0)
    let scale = 2.0;
    println!("Zoom IN (scale = {}):", scale);
    println!("  Visible width = {} / {} = {}", window_width, scale, window_width / scale);
    println!("  Expected: Should see LESS of the world (640 units wide)\n");
    
    // The key insight:
    println!("ANALYSIS:");
    println!("- When camera scale is 0.5, objects appear at half size");
    println!("- This means we can fit TWICE as many world units in our view");
    println!("- So visible_width = window_width / scale is CORRECT");
    println!("\nThe issue must be elsewhere in the calculation!");
}