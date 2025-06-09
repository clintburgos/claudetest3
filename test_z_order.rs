use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Z-Order Test".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.2, 0.2, 0.2)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d::default());
    
    // Create overlapping squares at different Z positions
    let colors = [
        (Color::srgb(1.0, 0.0, 0.0), 0.0),   // Red at z=0
        (Color::srgb(0.0, 1.0, 0.0), 1.0),   // Green at z=1
        (Color::srgb(0.0, 0.0, 1.0), -1.0),  // Blue at z=-1
        (Color::srgb(1.0, 1.0, 0.0), 0.5),   // Yellow at z=0.5
    ];
    
    for (i, (color, z)) in colors.iter().enumerate() {
        let offset = i as f32 * 30.0 - 45.0;
        
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(100.0, 100.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from(*color))),
            Transform::from_xyz(offset, offset, *z),
        ));
        
        println!("Spawned {} square at ({}, {}, {})", 
            match i {
                0 => "red",
                1 => "green",
                2 => "blue",
                3 => "yellow",
                _ => "unknown"
            },
            offset, offset, z
        );
    }
    
    println!("\nExpected front-to-back order: Green, Yellow, Red, Blue");
}