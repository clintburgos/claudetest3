use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, check_rendering)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Setting up tile mesh test...");
    
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Create tile mesh using the exact same code as in the project
    let tile_mesh = create_tile_diamond_mesh(64.0, 32.0);
    let mesh_handle = meshes.add(tile_mesh);
    
    // Create materials for different biomes
    let red = materials.add(ColorMaterial::from(Color::srgb(0.8, 0.2, 0.2)));
    let green = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.8, 0.2)));
    let blue = materials.add(ColorMaterial::from(Color::srgb(0.2, 0.2, 0.8)));
    
    // Spawn multiple tiles
    for i in 0..5 {
        for j in 0..5 {
            let x = (i as f32 - 2.0) * 70.0;
            let y = (j as f32 - 2.0) * 40.0;
            
            let material = match (i + j) % 3 {
                0 => red.clone(),
                1 => green.clone(),
                _ => blue.clone(),
            };
            
            commands.spawn((
                bevy::render::mesh::Mesh2d(mesh_handle.clone()),
                bevy::render::mesh::MeshMaterial2d(material),
                Transform::from_xyz(x, y, 0.0),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));
        }
    }
    
    println!("Spawned 25 diamond tiles in a 5x5 grid");
    
    // Add UI text
    commands.spawn((
        Text::new("Diamond tiles should be visible"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

// Copied from the project's tile mesh creation
fn create_tile_diamond_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // Vertices for a diamond shape (clockwise from top)
    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top
        [half_width, 0.0, 0.0],   // Right
        [0.0, -half_height, 0.0], // Bottom
        [-half_width, 0.0, 0.0],  // Left
    ];

    // Normals (all facing forward for 2D)
    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    // UV coordinates
    let uvs: Vec<[f32; 2]> = vec![
        [0.5, 0.0], // Top
        [1.0, 0.5], // Right
        [0.5, 1.0], // Bottom
        [0.0, 0.5], // Left
    ];

    // Indices for two triangles
    let indices = vec![0, 1, 2, 0, 2, 3];

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn check_rendering(
    query: Query<&Transform, With<bevy::render::mesh::Mesh2d>>,
    mut frame: Local<u32>,
) {
    *frame += 1;
    if *frame % 60 == 0 {
        let count = query.iter().count();
        println!("Frame {}: {} mesh entities in scene", *frame, count);
    }
}