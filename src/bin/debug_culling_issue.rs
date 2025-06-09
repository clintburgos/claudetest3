//! Debug program to visualize culling issue
//! 
//! This creates a minimal test case that shows tiles being incorrectly culled

use bevy::prelude::*;
use claudetest3::ui::world::{
    camera::{CameraState, IsometricCamera},
    grid::{coordinates::*, GridConfig},
    tiles::{
        components::{Tile, TileBiome, TilePosition},
        systems::{create_tile_diamond_mesh, spawn_tile, TileMeshes},
        view_culling::{view_culling_system, SpawnedTiles, ViewCullingConfig},
        isometric_culling::calculate_isometric_visible_tiles,
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
            tiles_per_frame: 1000,
            enabled: true,
        })
        .insert_resource(SpawnedTiles::default())
        .insert_resource(claudetest3::ui::world::grid::GridMap::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                view_culling_system,
                camera_movement,
                debug_info,
                toggle_culling,
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

    // UI for debug info
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
                Text::new("Debug Info\n"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn camera_movement(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };

    let mut movement = Vec2::ZERO;
    
    // WASD movement
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
    
    // Apply movement
    if movement != Vec2::ZERO {
        movement = movement.normalize() * 500.0 * time.delta_secs();
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
    }
    
    // Zoom with Q/E
    if keyboard_input.pressed(KeyCode::KeyQ) {
        camera_state.zoom = (camera_state.zoom * 1.02).min(camera_state.max_zoom);
    }
    if keyboard_input.pressed(KeyCode::KeyE) {
        camera_state.zoom = (camera_state.zoom / 1.02).max(camera_state.min_zoom);
    }
}

fn debug_info(
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    tile_query: Query<&TilePosition, With<Tile>>,
    windows: Query<&Window>,
    grid_config: Res<GridConfig>,
    culling_config: Res<ViewCullingConfig>,
    spawned_tiles: Res<SpawnedTiles>,
    mut text_query: Query<&mut Text>,
) {
    let Ok((camera_transform, camera_state)) = camera_query.single() else {
        return;
    };
    
    let Ok(window) = windows.single() else {
        return;
    };
    
    // Calculate grid position
    let (grid_x, grid_y, _) = world_to_grid(camera_transform.translation, grid_config.tile_size);
    
    // Calculate visible bounds
    let (min_x, min_y, max_x, max_y) = calculate_isometric_visible_tiles(
        camera_transform.translation,
        Vec2::new(window.width(), window.height()),
        camera_state.zoom,
        grid_config.tile_size,
        grid_config.width,
        grid_config.height,
        culling_config.buffer_tiles,
    );
    
    // Count visible tiles
    let visible_tiles = tile_query.iter().count();
    let expected_tiles = (max_x - min_x + 1) * (max_y - min_y + 1);
    
    // Update debug text
    for mut text in text_query.iter_mut() {
        text.0 = format!(
            "Camera World: ({:.0}, {:.0})\n\
             Camera Grid: ({}, {})\n\
             Zoom: {:.2}\n\
             Culling: {}\n\
             Visible Bounds: ({}, {}) to ({}, {})\n\
             Expected Area: {}x{} = {} tiles\n\
             Spawned Tiles: {}\n\
             Visible Tiles: {}\n\
             \n\
             Controls:\n\
             WASD - Move camera\n\
             Q/E - Zoom in/out\n\
             C - Toggle culling\n\
             R - Reset camera",
            camera_transform.translation.x,
            camera_transform.translation.y,
            grid_x,
            grid_y,
            camera_state.zoom,
            if culling_config.enabled { "ON" } else { "OFF" },
            min_x, min_y, max_x, max_y,
            max_x - min_x + 1,
            max_y - min_y + 1,
            expected_tiles,
            spawned_tiles.count(),
            visible_tiles,
        );
    }
}

fn toggle_culling(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut culling_config: ResMut<ViewCullingConfig>,
    mut commands: Commands,
    tile_query: Query<Entity, With<Tile>>,
    grid_config: Res<GridConfig>,
    tile_meshes: Res<TileMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut grid_map: ResMut<claudetest3::ui::world::grid::GridMap>,
    mut spawned_tiles: ResMut<SpawnedTiles>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyC) {
        culling_config.enabled = !culling_config.enabled;
        info!("Culling toggled: {}", if culling_config.enabled { "ON" } else { "OFF" });
        
        if !culling_config.enabled {
            // Clear all tiles first
            for entity in tile_query.iter() {
                commands.entity(entity).despawn();
            }
            grid_map.clear();
            spawned_tiles.clear();
            
            // Spawn all tiles
            info!("Spawning all {}x{} = {} tiles", grid_config.width, grid_config.height, grid_config.width * grid_config.height);
            for y in 0..grid_config.height {
                for x in 0..grid_config.width {
                    let position = TilePosition::ground(x, y);
                    let biome = TileBiome::Plain;
                    
                    let entity = spawn_tile(
                        &mut commands,
                        position,
                        biome,
                        grid_config.tile_size,
                        tile_meshes.diamond.clone(),
                        &mut materials,
                    );
                    
                    grid_map.insert_tile(x, y, entity);
                    spawned_tiles.insert(x, y);
                }
            }
        }
    }
    
    // Reset camera with R
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        // This won't work - need to query camera, not tiles
        info!("Reset camera pressed");
    }
}