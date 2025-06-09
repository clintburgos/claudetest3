use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Absolute Minimal Test".to_string(),
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
    // Camera at origin
    commands.spawn(Camera2d);
    
    // Red square using Bevy's Rectangle
    let mesh_handle = meshes.add(Rectangle::new(100.0, 100.0));
    let material_handle = materials.add(Color::srgb(1.0, 0.0, 0.0));
    
    let entity = commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material_handle.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
    )).id();
    
    println!("Created red square entity: {:?}", entity);
    println!("Mesh handle: {:?}", mesh_handle);
    println!("Material handle: {:?}", material_handle);
    println!("");
    println!("If you see a red square, 2D rendering works.");
    println!("If you see gray background only, 2D meshes are broken.");
}