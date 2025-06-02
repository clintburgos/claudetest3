//! Test zoom edge visibility - Specifically test that edge tiles remain visible when zoomed out
//!
//! This test focuses on the issue where tiles at the edges disappear when zooming out.

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        grid::GridConfig,
        tiles::{Tile, TilePosition, ViewCullingConfig},
        WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Zoom Edge Visibility".to_string(),
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
                test_zoom_levels,
                check_edge_tiles,
            ).run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);

    // Use default 200x200 map
    commands.insert_resource(GridConfig {
        width: 200,
        height: 200,
        tile_size: 64.0,
    });

    // Standard culling config with more buffer
    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 10, // Increased buffer
        tiles_per_frame: 100,
        enabled: true,
    });
}

#[derive(Resource)]
struct TestState {
    test_phase: usize,
    timer: Timer,
}

fn test_zoom_levels(
    mut commands: Commands,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    mut test_state: Local<Option<TestState>>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    // Initialize test state
    if test_state.is_none() {
        *test_state = Some(TestState {
            test_phase: 0,
            timer: Timer::from_seconds(3.0, TimerMode::Repeating),
        });
    }

    let state = test_state.as_mut().unwrap();
    state.timer.tick(time.delta());

    let Ok((mut transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };

    // Manual control
    if keyboard.just_pressed(KeyCode::Digit1) {
        camera_state.zoom = camera_state.min_zoom;
        info!("MANUAL: Set to minimum zoom: {:.4}", camera_state.zoom);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        camera_state.zoom = 1.0;
        info!("MANUAL: Set to default zoom: 1.0");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        camera_state.zoom = camera_state.max_zoom;
        info!("MANUAL: Set to maximum zoom: {:.4}", camera_state.zoom);
    }

    // Automated test phases
    if state.timer.just_finished() {
        match state.test_phase {
            0 => {
                // Phase 1: Test minimum zoom
                camera_state.zoom = camera_state.min_zoom;
                transform.scale = Vec3::splat(camera_state.zoom);
                info!("\n=== PHASE 1: Testing minimum zoom ({:.4}) ===", camera_state.zoom);
            }
            1 => {
                // Phase 2: Test slightly more than minimum
                camera_state.zoom = camera_state.min_zoom * 1.5;
                transform.scale = Vec3::splat(camera_state.zoom);
                info!("\n=== PHASE 2: Testing 1.5x minimum zoom ({:.4}) ===", camera_state.zoom);
            }
            2 => {
                // Phase 3: Test default zoom
                camera_state.zoom = 1.0;
                transform.scale = Vec3::splat(camera_state.zoom);
                info!("\n=== PHASE 3: Testing default zoom (1.0) ===");
            }
            _ => {
                // Reset
                state.test_phase = 0;
                return;
            }
        }
        state.test_phase += 1;
    }
}

fn check_edge_tiles(
    tile_query: Query<(&TilePosition, &Transform), With<Tile>>,
    mut last_check: Local<f32>,
    time: Res<Time>,
    grid_config: Res<GridConfig>,
) {
    let now = time.elapsed_secs();
    if now - *last_check < 1.0 {
        return;
    }
    *last_check = now;

    // Check for edge tiles
    let mut corner_tiles = vec![
        (0, 0, "Bottom-Left"),
        (grid_config.width - 1, 0, "Bottom-Right"),
        (0, grid_config.height - 1, "Top-Left"),
        (grid_config.width - 1, grid_config.height - 1, "Top-Right"),
    ];

    let mut edge_counts = (0, 0, 0, 0); // left, right, bottom, top

    for (tile_pos, _) in tile_query.iter() {
        // Check corners
        corner_tiles.retain(|(x, y, name)| {
            if tile_pos.x == *x && tile_pos.y == *y {
                info!("✓ {} corner tile IS VISIBLE at ({}, {})", name, x, y);
                false // Remove from list as it's found
            } else {
                true // Keep in list
            }
        });

        // Count edge tiles
        if tile_pos.x == 0 {
            edge_counts.0 += 1;
        }
        if tile_pos.x == grid_config.width - 1 {
            edge_counts.1 += 1;
        }
        if tile_pos.y == 0 {
            edge_counts.2 += 1;
        }
        if tile_pos.y == grid_config.height - 1 {
            edge_counts.3 += 1;
        }
    }

    // Report missing corners
    for (x, y, name) in &corner_tiles {
        warn!("✗ {} corner tile MISSING at ({}, {})", name, x, y);
    }

    // Report edge counts
    info!(
        "Edge tile counts - Left: {}, Right: {}, Bottom: {}, Top: {}",
        edge_counts.0, edge_counts.1, edge_counts.2, edge_counts.3
    );

    if corner_tiles.is_empty() {
        info!("✓ All corner tiles are visible!");
    } else {
        warn!("⚠ {} corner tiles are missing!", corner_tiles.len());
    }
}