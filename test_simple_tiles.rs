use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::asset::RenderAssetUsages;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, camera_controls)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Setting up simple tile test...");
    
    // Spawn camera at a reasonable height
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
    
    // Create diamond mesh (same as game uses)
    let mesh_handle = meshes.add(create_diamond_mesh(64.0, 32.0));
    
    // Spawn a grid of tiles
    let grid_size = 10;
    let tile_size = 64.0;
    
    for y in 0..grid_size {
        for x in 0..grid_size {
            let world_x = (x as f32 - grid_size as f32 / 2.0) * tile_size;
            let world_y = (y as f32 - grid_size as f32 / 2.0) * tile_size;
            
            // Pick a color based on position
            let color = match (x + y) % 3 {
                0 => Color::srgb(0.56, 0.93, 0.56), // Green
                1 => Color::srgb(0.27, 0.51, 0.71), // Blue
                _ => Color::srgb(0.94, 0.90, 0.55), // Yellow
            };
            
            let material = materials.add(ColorMaterial::from(color));
            
            // Spawn tile with all required components
            commands.spawn((
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(material),
                Transform::from_xyz(world_x, world_y, 0.0),
                GlobalTransform::default(),
                Visibility::default(),
                InheritedVisibility::default(),
                ViewVisibility::default(),
            ));
        }
    }
    
    println!("Spawned {} tiles in a {}x{} grid", grid_size * grid_size, grid_size, grid_size);
    println!("Controls: WASD to move, Q/E to zoom");
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

    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    let uvs: Vec<[f32; 2]> = vec![
        [0.5, 0.0], // Top
        [1.0, 0.5], // Right
        [0.5, 1.0], // Bottom
        [0.0, 0.5], // Left
    ];

    let indices = vec![0, 1, 2, 0, 2, 3];

    Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera2d>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = camera_query.get_single_mut() else {
        return;
    };
    
    let speed = 200.0 * time.delta_secs();
    let zoom_speed = 0.05;
    
    // Movement
    if keyboard.pressed(KeyCode::KeyW) {
        transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        transform.translation.y -= speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        transform.translation.x += speed;
    }
    
    // Zoom
    if keyboard.pressed(KeyCode::KeyQ) {
        transform.scale *= 1.0 + zoom_speed;
        println!("Zoom: {:.3}", transform.scale.x);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        transform.scale *= 1.0 - zoom_speed;
        println!("Zoom: {:.3}", transform.scale.x);
    }
}