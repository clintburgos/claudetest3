use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);
    
    // Just create a simple colored square first
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.56, 0.93, 0.56))), // Plain biome color
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    println!("If you see a light green square, basic 2D rendering works.");
}