//! Test camera fixes - Comprehensive test for zoom and movement issues
//!
//! This test verifies:
//! 1. Can zoom out to see entire map
//! 2. Can zoom in until ~4 tiles cover the screen
//! 3. Map doesn't disappear when zooming
//!
//! Controls:
//! - Q/E or mouse wheel: Zoom in/out
//! - WASD: Move camera
//! - Space: Reset camera to center
//! - Tab: Cycle through test positions

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        grid::coordinates::grid_center_world,
        tiles::{SpawnedTiles, Tile, TilePosition, ViewCullingConfig},
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Camera Fixes Test".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                monitor_system.run_if(in_state(GameState::Playing)),
                test_controls.run_if(in_state(GameState::Playing)),
                count_visible_tiles.run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);

    // Use a 200x200 map to match the reported issue
    commands.insert_resource(GridConfig {
        width: 200,
        height: 200,
        tile_size: 64.0,
    });

    // Standard culling config
    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 5,
        tiles_per_frame: 50,
        enabled: true,
    });

    // UI help text
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Auto,
                height: Val::Auto,
                left: Val::Px(10.0),
                top: Val::Px(10.0),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(
                    "Camera Test Controls:\n\
                Q/E or Mouse Wheel: Zoom in/out\n\
                WASD: Move camera\n\
                Space: Reset to center\n\
                Tab: Cycle test positions\n\
                1: Min zoom (see entire map)\n\
                2: Max zoom (~4 tiles)",
                ),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn monitor_system(
    spawned_tiles: Res<SpawnedTiles>,
    tile_query: Query<&Tile>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    windows: Query<&Window>,
    grid_config: Res<GridConfig>,
    mut last_report: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_report < 0.25 {
        return;
    }
    *last_report = now;

    if let Ok((transform, state)) = camera_query.single() {
        let tile_count = tile_query.iter().count();
        let Ok(window) = windows.single() else {
            return;
        };

        // Calculate visible world area
        let visible_width = window.width() * state.zoom;
        let visible_height = window.height() * state.zoom;

        // Calculate approximate tiles visible
        let tiles_x = visible_width / grid_config.tile_size;
        let tiles_y = visible_height / grid_config.tile_size;

        info!(
            "Camera: zoom={:.3} [{:.3}-{:.3}], pos=({:.0},{:.0}), visible_area={:.0}x{:.0} (~{:.1}x{:.1} tiles), spawned={}, entities={}",
            state.zoom, state.min_zoom, state.max_zoom,
            transform.translation.x, transform.translation.y,
            visible_width, visible_height,
            tiles_x, tiles_y,
            spawned_tiles.count(),
            tile_count
        );
    }
}

fn test_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    grid_config: Res<GridConfig>,
    mut test_position: Local<usize>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    // Reset to center
    if keyboard.just_pressed(KeyCode::Space) {
        let center =
            grid_center_world(grid_config.width, grid_config.height, grid_config.tile_size);
        transform.translation.x = center.x;
        transform.translation.y = center.y;
        state.zoom = 1.0;
        info!("Camera reset to center");
    }

    // Jump to min zoom
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.zoom = state.min_zoom;
        info!("Set to minimum zoom: {:.3}", state.zoom);
    }

    // Jump to max zoom
    if keyboard.just_pressed(KeyCode::Digit2) {
        state.zoom = state.max_zoom;
        info!("Set to maximum zoom: {:.3}", state.zoom);
    }

    // Cycle through test positions
    if keyboard.just_pressed(KeyCode::Tab) {
        let positions = vec![
            (
                "Center",
                grid_center_world(grid_config.width, grid_config.height, grid_config.tile_size),
            ),
            (
                "Top-Left Corner",
                Vec3::new(
                    0.0,
                    (grid_config.height as f32) * grid_config.tile_size * 0.5,
                    1000.0,
                ),
            ),
            (
                "Bottom-Right Corner",
                Vec3::new(
                    (grid_config.width as f32) * grid_config.tile_size,
                    0.0,
                    1000.0,
                ),
            ),
            (
                "Map Edge",
                Vec3::new(
                    (grid_config.width as f32) * grid_config.tile_size * 0.5,
                    0.0,
                    1000.0,
                ),
            ),
        ];

        *test_position = (*test_position + 1) % positions.len();
        let (name, pos) = &positions[*test_position];
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
        info!("Moved to: {}", name);
    }
}

fn count_visible_tiles(
    tile_query: Query<(&TilePosition, &Transform), With<Tile>>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    windows: Query<&Window>,
    mut last_count: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_count < 2.0 {
        return;
    }
    *last_count = now;

    let Ok((cam_transform, cam_state)) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    // Calculate visible bounds
    let visible_width = window.width() * cam_state.zoom;
    let visible_height = window.height() * cam_state.zoom;

    let left = cam_transform.translation.x - visible_width * 0.5;
    let right = cam_transform.translation.x + visible_width * 0.5;
    let bottom = cam_transform.translation.y - visible_height * 0.5;
    let top = cam_transform.translation.y + visible_height * 0.5;

    // Count tiles within view
    let mut visible_count = 0;
    let mut edge_tiles = Vec::new();

    for (tile_pos, tile_transform) in tile_query.iter() {
        let x = tile_transform.translation.x;
        let y = tile_transform.translation.y;

        if x >= left && x <= right && y >= bottom && y <= top {
            visible_count += 1;

            // Track edge tiles
            if tile_pos.x == 0 || tile_pos.x == 199 || tile_pos.y == 0 || tile_pos.y == 199 {
                edge_tiles.push((tile_pos.x, tile_pos.y));
            }
        }
    }

    if !edge_tiles.is_empty() {
        info!(
            "Visible tiles: {} total, {} edge tiles at positions: {:?}",
            visible_count,
            edge_tiles.len(),
            edge_tiles.iter().take(5).collect::<Vec<_>>()
        );
    }
}
