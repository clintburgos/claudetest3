use bevy::prelude::*;
use bevy::core_pipeline::tonemapping::Tonemapping;

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
    // Test 1: Camera with default tonemapping
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(-300.0, 0.0, 0.0),
    ));
    
    // Test 2: Camera with disabled tonemapping
    commands.spawn((
        Camera2d::default(),
        Camera {
            is_active: false,
            ..default()
        },
        Tonemapping::None,
        Transform::from_xyz(300.0, 0.0, 0.0),
    ));
    
    // Spawn bright colored squares
    let positions = [
        (-300.0, Color::srgb(1.0, 0.0, 0.0)),
        (0.0, Color::srgb(0.0, 1.0, 0.0)),
        (300.0, Color::srgb(0.0, 0.0, 1.0)),
    ];
    
    for (x, color) in positions.iter() {
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
            MeshMaterial2d(materials.add(*color)),
            Transform::from_xyz(*x, 0.0, 0.0),
        ));
    }
    
    println!("Testing tonemapping settings");
    println!("You should see 3 colored squares");
}