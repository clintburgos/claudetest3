use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, debug_entities)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn camera exactly like the game does
    let camera = commands.spawn((
        Camera2d {
            ..default()
        },
        Transform::from_xyz(0.0, -3200.0, 1000.0),
    )).id();
    
    println!("Camera spawned: {:?}", camera);
    
    // Spawn a mesh at the camera's focus point
    let mesh_entity = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(200.0, 200.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, -3200.0, 0.0),
    )).id();
    
    println!("Mesh spawned at camera focus: {:?}", mesh_entity);
    
    // Also spawn one at origin to test
    let origin_mesh = commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).id();
    
    println!("Mesh spawned at origin: {:?}", origin_mesh);
}

fn debug_entities(
    mut done: Local<bool>,
    cameras: Query<(Entity, &Transform, &Camera2d)>,
    meshes: Query<(Entity, &Transform), With<Mesh2d>>,
) {
    if *done { return; }
    *done = true;
    
    println!("\n=== Entity Debug ===");
    for (entity, transform, _) in cameras.iter() {
        println!("Camera {:?} at {:?}", entity, transform.translation);
    }
    
    for (entity, transform) in meshes.iter() {
        println!("Mesh {:?} at {:?}", entity, transform.translation);
    }
    
    println!("\nExpected: Red square visible (at camera focus)");
    println!("Expected: Green square NOT visible (at origin, far from camera)");
}