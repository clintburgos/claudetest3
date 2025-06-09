use bevy::{prelude::*, render::{mesh::*, render_asset::RenderAssetUsages}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Exact Tile Mesh Test".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

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

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera at origin
    commands.spawn(Camera2d);
    
    // Create several diamond tiles
    let tile_mesh = create_tile_diamond_mesh(64.0, 32.0);
    let mesh_handle = meshes.add(tile_mesh);
    
    // Spawn tiles in different colors
    let colors = [
        Color::srgb(0.0, 0.8, 0.0), // Green
        Color::srgb(1.0, 0.0, 0.0), // Red
        Color::srgb(0.0, 0.0, 1.0), // Blue
        Color::srgb(1.0, 1.0, 0.0), // Yellow
    ];
    
    for (i, color) in colors.iter().enumerate() {
        let x = (i as f32 - 1.5) * 80.0;
        let entity = commands.spawn((
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from(*color))),
            Transform::from_xyz(x, 0.0, 0.0),
        )).id();
        
        println!("Spawned diamond tile {} at x={}: entity={:?}", i, x, entity);
    }
    
    println!("\nYou should see 4 colored diamond tiles in a row.");
    println!("If you see only the gray background, the custom mesh is the issue.");
}