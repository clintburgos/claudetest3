use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("=== 2D Mesh Rendering Test ===");
    
    // Spawn camera
    commands.spawn(Camera2d);
    println!("Camera spawned");
    
    // Test 1: Using Mesh2d components directly
    let mesh = meshes.add(Rectangle::new(100.0, 100.0));
    let material = materials.add(Color::srgb(1.0, 0.0, 0.0));
    
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));
    println!("Spawned red square at (-200, 0) using Mesh2d components");
    
    // Test 2: Using individual components (new way)
    let mesh2 = meshes.add(Circle::new(50.0));
    let material2 = materials.add(Color::srgb(0.0, 1.0, 0.0));
    
    commands.spawn((
        Mesh2d(mesh2),
        MeshMaterial2d(material2),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    println!("Spawned green circle at (0, 0) using individual components");
    
    // Test 3: Sprite for comparison
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
    println!("Spawned blue sprite at (200, 0) for comparison");
    
    println!("\nIf you see:");
    println!("- Red square on left: MaterialMesh2dBundle works");
    println!("- Green circle in center: Individual components work");
    println!("- Blue square on right: Sprite rendering works");
    println!("- Nothing: 2D rendering is broken");
}