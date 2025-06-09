use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

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
    println!("=== Mesh Debug Test ===");
    
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Test 1: Bevy's built-in rectangle
    let rect_mesh = meshes.add(Rectangle::new(100.0, 100.0));
    let red_mat = materials.add(Color::srgb(1.0, 0.0, 0.0));
    
    commands.spawn((
        Mesh2d(rect_mesh),
        MeshMaterial2d(red_mat),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));
    println!("Test 1: Spawned Bevy rectangle at (-200, 0)");
    
    // Test 2: Custom diamond mesh (same as game)
    let diamond_mesh = meshes.add(create_diamond_mesh(100.0, 50.0));
    let green_mat = materials.add(Color::srgb(0.0, 1.0, 0.0));
    
    commands.spawn((
        Mesh2d(diamond_mesh),
        MeshMaterial2d(green_mat),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    println!("Test 2: Spawned custom diamond at (0, 0)");
    
    // Test 3: Custom triangle mesh
    let triangle_mesh = meshes.add(create_triangle_mesh());
    let blue_mat = materials.add(Color::srgb(0.0, 0.0, 1.0));
    
    commands.spawn((
        Mesh2d(triangle_mesh),
        MeshMaterial2d(blue_mat),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
    println!("Test 3: Spawned custom triangle at (200, 0)");
    
    println!("\nIf you see:");
    println!("- Red rectangle: Bevy built-in meshes work");
    println!("- Green diamond: Custom diamond mesh works");
    println!("- Blue triangle: Custom mesh creation works");
}

fn create_diamond_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top
        [half_width, 0.0, 0.0],   // Right
        [0.0, -half_height, 0.0], // Bottom
        [-half_width, 0.0, 0.0],  // Left
    ];
    
    println!("Diamond vertices: {:?}", vertices);

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];
    let uvs: Vec<[f32; 2]> = vec![
        [0.5, 0.0], [1.0, 0.5], [0.5, 1.0], [0.0, 0.5]
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];
    
    println!("Diamond indices: {:?}", indices);

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn create_triangle_mesh() -> Mesh {
    let vertices: Vec<[f32; 3]> = vec![
        [0.0, 50.0, 0.0],   // Top
        [50.0, -50.0, 0.0], // Right
        [-50.0, -50.0, 0.0], // Left
    ];
    
    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 3];
    let uvs: Vec<[f32; 2]> = vec![[0.5, 0.0], [1.0, 1.0], [0.0, 1.0]];
    let indices = vec![0, 1, 2];

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}