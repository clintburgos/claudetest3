use bevy::prelude::*;

fn main() {
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
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Create a simple square mesh
    let mesh = meshes.add(Rectangle::new(100.0, 100.0));
    
    // Create a red material
    let material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    
    // Spawn the mesh with all required components
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
    
    // Spawn another square to test
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
    
    println!("Spawned red square at (0, 0, 0)");
}