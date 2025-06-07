use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};

mod debug_overlay;
mod camera_controls;
mod screenshot;

use debug_overlay::*;
use camera_controls::*;
use screenshot::*;

pub struct TestingPlugin;

impl Plugin for TestingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentTestScenario>()
            .add_systems(Startup, (setup_debug_overlay, setup_screenshot_indicator))
            .add_systems(Update, (
                handle_test_inputs,
                toggle_debug_overlay,
                update_debug_info,
                handle_test_camera_controls,
                cycle_test_scenarios,
                apply_test_scenario,
                update_screenshot_indicator,
            ));
    }
}


fn handle_test_inputs(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut screenshot_counter: Local<u32>,
) {
    // F12 - Take screenshot
    if keyboard.just_pressed(KeyCode::F12) {
        let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("screenshots/bevy_{}_{}.png", timestamp, *screenshot_counter);
        *screenshot_counter += 1;
        
        // Create screenshots directory if it doesn't exist
        std::fs::create_dir_all("screenshots").ok();
        
        // Take screenshot using Bevy 0.16 observer API
        commands
            .spawn(Screenshot::primary_window())
            .observe(save_to_disk(filename.clone()));
            
        info!("Taking screenshot: {}", filename);
    }
    
    // F1 - Show help
    if keyboard.just_pressed(KeyCode::F1) {
        info!("=== Test Controls ===");
        info!("F1  - Show this help");
        info!("F12 - Take screenshot");
        info!("F5  - Reset camera to center");
        info!("F6  - Zoom to minimum");
        info!("F7  - Zoom to maximum");
        info!("F8  - Toggle debug overlay");
        info!("F9  - Cycle through test scenarios");
        info!("===================");
    }
    
    // F5 - Reset camera
    if keyboard.just_pressed(KeyCode::F5) {
        info!("Resetting camera to center position");
        // This will be handled by camera system
    }
    
    // F6 - Min zoom
    if keyboard.just_pressed(KeyCode::F6) {
        info!("Setting camera to minimum zoom");
    }
    
    // F7 - Max zoom
    if keyboard.just_pressed(KeyCode::F7) {
        info!("Setting camera to maximum zoom");
    }
    
    // F8 - Toggle debug overlay
    if keyboard.just_pressed(KeyCode::F8) {
        info!("Toggling debug overlay");
    }
    
    // F9 - Cycle test scenarios
    if keyboard.just_pressed(KeyCode::F9) {
        info!("Cycling to next test scenario");
    }
}


// Test scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestScenario {
    Normal,
    MaxZoom,
    MinZoom,
    EdgeOfMap,
    CenterOfMap,
}

#[derive(Resource)]
pub struct CurrentTestScenario(pub TestScenario);

impl Default for CurrentTestScenario {
    fn default() -> Self {
        Self(TestScenario::Normal)
    }
}