use bevy::prelude::*;

#[derive(Component)]
struct IsometricCamera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game Camera Exact Test".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_systems(Startup, setup)
        .add_systems(Update, debug_info)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera setup EXACTLY like the game
    let camera_entity = commands.spawn((
        Camera2d {
            ..default()
        },
        IsometricCamera,
        Transform::from_xyz(0.0, -3200.0, 1000.0),
    )).id();
    
    println!("Spawned camera {:?} at (0, -3200, 1000) - EXACT game setup", camera_entity);
    
    // Create diamond mesh EXACTLY like the game
    let mesh = create_tile_diamond_mesh(64.0, 32.0);
    let mesh_handle = meshes.add(mesh);
    
    // Spawn a grid of tiles around where the camera is looking
    let colors = [
        Color::srgb(0.2, 0.7, 0.2), // Green - Plains
        Color::srgb(0.1, 0.5, 0.1), // Dark Green - Forest  
        Color::srgb(0.0, 0.4, 0.8), // Blue - Water
        Color::srgb(0.9, 0.8, 0.4), // Sandy - Desert
    ];
    
    // Spawn tiles in a 10x10 grid centered at world (0, -3200)
    for y in -5..5 {
        for x in -5..5 {
            let grid_x = 100 + x;
            let grid_y = 100 + y;
            let world_pos = grid_to_world(grid_x, grid_y, 0, 64.0);
            let color = colors[((x + y).abs() % 4) as usize];
            
            let entity = commands.spawn((
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(materials.add(ColorMaterial::from(color))),
                Transform::from_translation(world_pos),
            )).id();
            
            if x == 0 && y == 0 {
                println!("Center tile at grid ({},{}) world ({:.1},{:.1},{:.1}) - entity {:?}", 
                    grid_x, grid_y, world_pos.x, world_pos.y, world_pos.z, entity);
            }
        }
    }
    
    println!("\nYou should see a 10x10 grid of colored diamond tiles");
}

fn debug_info(
    cameras: Query<(Entity, &Transform, &Camera2d), With<IsometricCamera>>,
    tiles: Query<Entity, With<Mesh2d>>,
    mut done: Local<bool>,
) {
    if *done { return; }
    *done = true;
    
    println!("\n=== Debug Info ===");
    for (entity, transform, _) in cameras.iter() {
        println!("Camera {:?} at {:?}", entity, transform.translation);
    }
    println!("Total tiles spawned: {}", tiles.iter().count());
}

// EXACT copy of game's mesh creation
fn create_tile_diamond_mesh(width: f32, height: f32) -> Mesh {
    use bevy::render::{mesh::*, render_asset::RenderAssetUsages};
    
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

// EXACT copy of game's coordinate conversion
fn grid_to_world(x: i32, y: i32, z: i32, tile_size: f32) -> Vec3 {
    let half_width = tile_size * 0.5;
    let half_height = tile_size * 0.25;

    let world_x = (x - y) as f32 * half_width;
    let world_y = -(x + y) as f32 * half_height;
    let world_z = z as f32 * tile_size * 0.5;

    Vec3::new(world_x, world_y, world_z)
}