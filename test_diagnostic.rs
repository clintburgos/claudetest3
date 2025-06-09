use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Diagnostic Test".to_string(),
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
        // Skip menu, go directly to playing
        .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
            next_state.set(game::GameState::Playing);
        })
        .add_systems(Update, diagnostic_system.run_if(run_once()))
        .run();
}

fn diagnostic_system(
    world: &World,
    tile_meshes: Option<Res<ui::world::tiles::TileMeshes>>,
    camera_query: Query<(Entity, &Transform), With<ui::world::camera::IsometricCamera>>,
    tile_query: Query<Entity, With<ui::world::tiles::Tile>>,
    mesh_query: Query<Entity, With<Mesh2d>>,
) {
    println!("\n=== DIAGNOSTIC REPORT ===");
    
    // Check TileMeshes resource
    if tile_meshes.is_some() {
        println!("✓ TileMeshes resource exists");
    } else {
        println!("✗ TileMeshes resource MISSING!");
    }
    
    // Check camera
    let camera_count = camera_query.iter().count();
    println!("Camera count: {}", camera_count);
    for (entity, transform) in camera_query.iter() {
        println!("  Camera {:?} at ({:.1}, {:.1}, {:.1})", 
            entity, transform.translation.x, transform.translation.y, transform.translation.z);
    }
    
    // Check tiles
    let tile_count = tile_query.iter().count();
    println!("Tile entity count: {}", tile_count);
    
    // Check mesh2d entities
    let mesh_count = mesh_query.iter().count();
    println!("Mesh2d entity count: {}", mesh_count);
    
    // Check for ColorMaterial assets
    if let Some(materials) = world.get_resource::<Assets<ColorMaterial>>() {
        println!("ColorMaterial asset count: {}", materials.len());
    }
    
    // Check for Mesh assets
    if let Some(meshes) = world.get_resource::<Assets<Mesh>>() {
        println!("Mesh asset count: {}", meshes.len());
    }
    
    println!("========================\n");
}