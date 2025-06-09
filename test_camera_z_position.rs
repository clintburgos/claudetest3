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
    // Test different camera Z positions
    
    // Camera 1: Default Z (should be around 1000 for 2D cameras in Bevy)
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(-400.0, 0.0, 1000.0), // Note: explicitly set to 1000
    ));
    
    // Camera 2: Z = 0 (might not work)
    commands.spawn((
        Camera2d::default(),
        Camera {
            is_active: false, // Disable so it doesn't conflict
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    // Camera 3: Default transform (let Bevy handle it)
    commands.spawn(Camera2d::default());
    
    // Spawn test meshes
    let colors = [
        (Color::srgb(1.0, 0.0, 0.0), -200.0), // Red
        (Color::srgb(0.0, 1.0, 0.0), 0.0),    // Green
        (Color::srgb(0.0, 0.0, 1.0), 200.0),  // Blue
    ];
    
    for (color, x) in colors.iter() {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
            MeshMaterial2d(materials.add(*color)),
            Transform::from_xyz(*x, 0.0, 0.0),
        ));
    }
    
    println!("Testing camera Z positions:");
    println!("- Camera with Z=1000 (like game)");
    println!("- Default Camera2d spawn");
    println!("- Three colored squares: Red, Green, Blue");
    println!("\nIf squares are visible, camera Z position is OK");
}