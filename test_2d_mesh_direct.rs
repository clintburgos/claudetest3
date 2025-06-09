use bevy::prelude::*;
use bevy::render::mesh::{Mesh2d, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

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
    
    // Create a simple diamond mesh
    let mesh = meshes.add(create_diamond_mesh());
    
    // Create materials with different colors
    let red_material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    let green_material = materials.add(ColorMaterial::from(Color::srgb(0.0, 1.0, 0.0)));
    let blue_material = materials.add(ColorMaterial::from(Color::srgb(0.0, 0.0, 1.0)));
    
    // Spawn multiple diamonds to test rendering
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(red_material),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(green_material),
        Transform::from_xyz(100.0, 50.0, 0.0),
    ));
    
    commands.spawn((
        Mesh2d(mesh.clone()),
        MeshMaterial2d(blue_material),
        Transform::from_xyz(-100.0, -50.0, 0.0),
    ));
    
    println!("Spawned 3 diamond meshes: red at (0,0), green at (100,50), blue at (-100,-50)");
}

fn create_diamond_mesh() -> Mesh {
    let half_width = 50.0;
    let half_height = 25.0;

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
    .with_inserted_indices(bevy::render::mesh::Indices::U32(indices))
}