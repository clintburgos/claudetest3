use bevy::prelude::*;

fn main() {
    println!("Starting basic 2D test...");
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Setup called!");
    
    // Spawn camera
    commands.spawn(Camera2d);
    println!("Camera spawned");
    
    // Use built-in Rectangle primitive
    let mesh = meshes.add(Rectangle::new(200.0, 100.0));
    let material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    
    // Spawn a simple colored rectangle
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    println!("Red rectangle spawned at origin");
    
    // Also try spawning a sprite for comparison
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
    
    println!("Green sprite spawned at (200, 0)");
}