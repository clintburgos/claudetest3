use bevy::{prelude::*, render::{mesh::*, render_asset::RenderAssetUsages}};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Mesh Winding Test".to_string(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .run();
}

fn create_diamond_clockwise(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // Vertices for a diamond shape (clockwise from top)
    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top (0)
        [half_width, 0.0, 0.0],   // Right (1)
        [0.0, -half_height, 0.0], // Bottom (2)
        [-half_width, 0.0, 0.0],  // Left (3)
    ];

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];
    let uvs: Vec<[f32; 2]> = vec![[0.5, 0.0], [1.0, 0.5], [0.5, 1.0], [0.0, 0.5]];

    // Current indices (clockwise)
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

fn create_diamond_counterclockwise(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // Same vertices
    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top (0)
        [half_width, 0.0, 0.0],   // Right (1)
        [0.0, -half_height, 0.0], // Bottom (2)
        [-half_width, 0.0, 0.0],  // Left (3)
    ];

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];
    let uvs: Vec<[f32; 2]> = vec![[0.5, 0.0], [1.0, 0.5], [0.5, 1.0], [0.0, 0.5]];

    // Counter-clockwise indices
    let indices = vec![0, 3, 2, 0, 2, 1];

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
    // Camera
    commands.spawn(Camera2d::default());
    
    // Test both winding orders
    let clockwise_mesh = meshes.add(create_diamond_clockwise(100.0, 50.0));
    let counterclockwise_mesh = meshes.add(create_diamond_counterclockwise(100.0, 50.0));
    
    // Left: Clockwise (current game implementation)
    commands.spawn((
        Mesh2d(clockwise_mesh),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(-150.0, 0.0, 0.0),
    ));
    
    // Right: Counter-clockwise
    commands.spawn((
        Mesh2d(counterclockwise_mesh),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(150.0, 0.0, 0.0),
    ));
    
    // Center: Bevy built-in rectangle for reference
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    println!("Left (Red): Clockwise winding");
    println!("Right (Green): Counter-clockwise winding");
    println!("Center (Blue): Bevy Rectangle reference");
    println!("\nIf only one diamond appears, winding order is the issue.");
}