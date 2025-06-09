//! Debug spawning pattern to see which tiles are actually being spawned
//!
//! This creates a visual grid showing which tiles are spawned vs culled

use bevy::prelude::*;
use claudetest3::ui::world::{
    camera::{CameraState, IsometricCamera},
    grid::{coordinates::*, GridConfig, GridMap},
    tiles::{
        components::{Tile, TileBiome, TilePosition},
        systems::{create_tile_diamond_mesh, spawn_tile, TileMeshes},
        view_culling::{view_culling_system, SpawnedTiles, ViewCullingConfig},
    },
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .insert_resource(GridConfig {
            width: 200,
            height: 200,
            tile_size: 64.0,
        })
        .insert_resource(ViewCullingConfig {
            buffer_tiles: 5,
            tiles_per_frame: 100, // Reduced to see the spawning pattern
            enabled: true,
        })
        .insert_resource(SpawnedTiles::default())
        .insert_resource(GridMap::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                view_culling_system,
                debug_spawning_pattern,
                camera_movement,
            ).chain(),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Create tile meshes
    let tile_mesh = create_tile_diamond_mesh(grid_config.tile_size, grid_config.tile_size * 0.5);
    commands.insert_resource(TileMeshes {
        diamond: meshes.add(tile_mesh),
    });

    // Setup camera at center of map
    let center = grid_center_world(grid_config.width, grid_config.height, grid_config.tile_size);
    commands.spawn((
        Camera2d::default(),
        IsometricCamera,
        CameraState {
            zoom: 1.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            ..default()
        },
        Transform::from_xyz(center.x, center.y, 1000.0),
    ));

    // Debug overlay
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(""),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn debug_spawning_pattern(
    spawned_tiles: Res<SpawnedTiles>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    grid_config: Res<GridConfig>,
    mut text_query: Query<&mut Text>,
    time: Res<Time>,
) {
    static mut FRAME_COUNT: u32 = 0;
    static mut LAST_LOG_TIME: f32 = 0.0;
    
    unsafe {
        FRAME_COUNT += 1;
        
        // Log every second
        if time.elapsed_secs() - LAST_LOG_TIME > 1.0 {
            LAST_LOG_TIME = time.elapsed_secs();
            
            if let Ok((camera_transform, camera_state)) = camera_query.single() {
                let (cam_x, cam_y, _) = world_to_grid(camera_transform.translation, grid_config.tile_size);
                
                // Check pattern around camera
                let mut pattern = String::new();
                pattern.push_str("Spawn pattern (. = spawned, X = not spawned):\n");
                
                for dy in -10..=10 {
                    for dx in -10..=10 {
                        let tile_x = cam_x + dx;
                        let tile_y = cam_y + dy;
                        
                        if tile_x < 0 || tile_x >= grid_config.width || tile_y < 0 || tile_y >= grid_config.height {
                            pattern.push('#'); // Out of bounds
                        } else if spawned_tiles.contains(tile_x, tile_y) {
                            pattern.push('.');
                        } else {
                            pattern.push('X');
                        }
                    }
                    pattern.push('\n');
                }
                
                info!("Frame {}: Camera at grid ({}, {}), {} tiles spawned", 
                    FRAME_COUNT, cam_x, cam_y, spawned_tiles.count());
                info!("\n{}", pattern);
            }
        }
        
        // Update debug text
        for mut text in text_query.iter_mut() {
            text.0 = format!(
                "Frame: {}\nSpawned tiles: {}\nPress WASD to move camera",
                FRAME_COUNT,
                spawned_tiles.count()
            );
        }
    }
}

fn camera_movement(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, _camera_state)) = camera_query.single_mut() else {
        return;
    };

    let mut movement = Vec2::ZERO;
    
    if keyboard_input.pressed(KeyCode::KeyW) {
        movement.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        movement.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        movement.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        movement.x += 1.0;
    }
    
    if movement != Vec2::ZERO {
        movement = movement.normalize() * 200.0 * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
}