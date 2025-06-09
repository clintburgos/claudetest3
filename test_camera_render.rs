use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Camera Render Test".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .add_systems(Update, log_entities.run_if(run_once()))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera at specific position (like in game)
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0.0, -3200.0, 1000.0),
    ));
    
    // Spawn a tile at the position the camera is looking at
    let tile_pos = Vec3::new(0.0, -3200.0, 0.0);
    
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(64.0, 64.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_translation(tile_pos),
    ));
    
    // Also spawn one at origin for reference
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(64.0, 64.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    println!("Camera at (0, -3200, 1000) looking at center");
    println!("Green square at (0, -3200, 0) - should be visible");
    println!("Red square at (0, 0, 0) - should be off screen");
}

fn log_entities(
    cameras: Query<(Entity, &Transform), With<Camera2d>>,
    meshes: Query<(Entity, &Transform), With<Mesh2d>>,
) {
    println!("\n=== Entity Report ===");
    for (entity, transform) in cameras.iter() {
        println!("Camera {:?} at {:?}", entity, transform.translation);
    }
    for (entity, transform) in meshes.iter() {
        println!("Mesh {:?} at {:?}", entity, transform.translation);
    }
}