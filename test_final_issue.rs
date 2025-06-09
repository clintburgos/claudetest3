use bevy::prelude::*;
use bevy::render::{mesh::*, render_asset::RenderAssetUsages};

// Marker component
#[derive(Component)]
struct Tile;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Final Issue Test".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_systems(Startup, setup)
        .add_systems(Update, report_once)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera EXACTLY as game sets it up
    let camera = commands.spawn((
        Camera2d {
            ..default()
        },
        Transform::from_xyz(0.0, -3200.0, 1000.0),
    )).id();
    
    // Create tile mesh EXACTLY as game does
    let tile_mesh = create_tile_diamond_mesh(64.0, 32.0);
    let mesh_handle = meshes.add(tile_mesh);
    
    // Spawn tiles EXACTLY as game does (from view_culling.rs spawn_tile)
    let world_pos = Vec3::new(0.0, -3200.0, 0.0); // At camera focus
    let biome_color = Color::srgb(0.56, 0.93, 0.56); // Plain biome
    
    let tile = commands
        .spawn((
            Tile,
            Mesh2d(mesh_handle.clone()),
            MeshMaterial2d(materials.add(ColorMaterial::from(biome_color))),
            Transform::from_translation(world_pos),
        ))
        .id();
        
    println!("Setup complete:");
    println!("  Camera: {:?} at (0, -3200, 1000)", camera);
    println!("  Tile: {:?} at (0, -3200, 0)", tile);
    println!("\nThis exactly reproduces how the game spawns tiles.");
    println!("If no green diamond is visible, the issue is confirmed.");
}

// EXACT copy from game
fn create_tile_diamond_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top
        [half_width, 0.0, 0.0],   // Right
        [0.0, -half_height, 0.0], // Bottom
        [-half_width, 0.0, 0.0],  // Left
    ];

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    let uvs: Vec<[f32; 2]> = vec![
        [0.5, 0.0], // Top
        [1.0, 0.5], // Right
        [0.5, 1.0], // Bottom
        [0.0, 0.5], // Left
    ];

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

fn report_once(
    mut done: Local<bool>,
    tiles: Query<Entity, With<Tile>>,
    meshes: Query<Entity, With<Mesh2d>>,
    cameras: Query<Entity, With<Camera2d>>,
) {
    if *done { return; }
    *done = true;
    
    println!("\n=== Final Report ===");
    println!("Cameras: {}", cameras.iter().count());
    println!("Tiles: {}", tiles.iter().count());
    println!("Mesh2d entities: {}", meshes.iter().count());
}