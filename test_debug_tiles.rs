use bevy::prelude::*;
use bevy::render::mesh::{Mesh2d, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, debug_entities)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("=== SETUP START ===");
    
    // Spawn camera
    let camera_id = commands.spawn(Camera2d).id();
    println!("Spawned Camera2d with entity {:?}", camera_id);
    
    // Create a simple diamond mesh
    let mesh = create_diamond_mesh();
    let mesh_handle = meshes.add(mesh);
    println!("Created mesh with handle {:?}", mesh_handle);
    
    // Create a red material
    let material = materials.add(ColorMaterial::from(Color::srgb(1.0, 0.0, 0.0)));
    println!("Created material with handle {:?}", material);
    
    // Spawn the mesh with all required components
    let entity = commands.spawn((
        Mesh2d(mesh_handle.clone()),
        MeshMaterial2d(material.clone()),
        Transform::from_xyz(0.0, 0.0, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    )).id();
    
    println!("Spawned mesh entity {:?} at (0, 0, 0)", entity);
    println!("=== SETUP END ===");
}

fn create_diamond_mesh() -> Mesh {
    let half_width = 100.0;
    let half_height = 50.0;

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

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );
    
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(bevy::render::mesh::Indices::U32(indices));
    
    println!("Created diamond mesh with 4 vertices and 2 triangles");
    mesh
}

fn debug_entities(
    query: Query<(Entity, &Transform, Option<&Mesh2d>)>,
    mut frame_count: Local<u32>,
) {
    *frame_count += 1;
    
    // Only print every 60 frames (approximately once per second at 60 FPS)
    if *frame_count % 60 == 0 {
        println!("\n=== Frame {} Debug ===", *frame_count);
        let mut entity_count = 0;
        let mut mesh_count = 0;
        
        for (entity, transform, mesh) in query.iter() {
            entity_count += 1;
            if mesh.is_some() {
                mesh_count += 1;
                println!("Mesh entity {:?} at position: {:?}", entity, transform.translation);
            }
        }
        
        println!("Total entities: {}, Mesh entities: {}", entity_count, mesh_count);
    }
}