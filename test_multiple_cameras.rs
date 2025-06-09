use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Multiple Cameras Test".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
    .add_plugins((
        game::GameStatePlugin,
        ui::world::WorldPlugin,
        ui::panels::UIPanelsPlugin,
    ))
    // Skip menu and go straight to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    // Check for multiple cameras
    .add_systems(Update, check_cameras.run_if(run_once()));

    app.run();
}

fn check_cameras(
    cameras: Query<(Entity, &Camera, &Transform, Option<&Name>)>,
    isometric_cameras: Query<Entity, With<ui::world::camera::IsometricCamera>>,
) {
    println!("\n=== CAMERA CHECK ===");
    
    let total_cameras = cameras.iter().count();
    println!("Total cameras: {}", total_cameras);
    
    if total_cameras > 1 {
        println!("⚠️  WARNING: Multiple cameras detected!");
    }
    
    for (entity, camera, transform, name) in cameras.iter() {
        let name_str = name.map(|n| n.as_str()).unwrap_or("unnamed");
        let is_isometric = isometric_cameras.iter().any(|e| e == entity);
        
        println!("\nCamera {:?} [{}]", entity, name_str);
        println!("  Position: {:?}", transform.translation);
        println!("  Is active: {}", camera.is_active);
        println!("  Is isometric camera: {}", is_isometric);
        println!("  Order: {}", camera.order);
    }
    
    println!("====================\n");
}

fn run_once() -> impl FnMut() -> bool {
    let mut has_run = false;
    move || {
        if has_run {
            false
        } else {
            has_run = true;
            true
        }
    }
}