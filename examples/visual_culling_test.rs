//! Visual test for culling - logs tile visibility at corners

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
                title: "Visual Culling Test".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (test_minimum_zoom, log_corner_visibility).run_if(in_state(GameState::Playing)),
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

    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 5,
        tiles_per_frame: 5000,
        enabled: true,
    });
}

fn test_minimum_zoom(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    mut test_started: Local<bool>,
) {
    if *test_started {
        return;
    }

    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    // Set to minimum zoom
    state.zoom = state.min_zoom;
    transform.scale = Vec3::splat(state.zoom);
    *test_started = true;

    info!("=== VISUAL CULLING TEST STARTED ===");
    info!("Set camera to minimum zoom: {:.4}", state.zoom);
    info!("Watch for corner visibility logs...");
}

fn log_corner_visibility(
    tile_query: Query<&TilePosition, With<Tile>>,
    mut last_log: Local<f32>,
    time: Res<Time>,
    camera_query: Query<&CameraState, With<IsometricCamera>>,
) {
    let now = time.elapsed_secs();
    if now - *last_log < 2.0 {
        return;
    }
    *last_log = now;

    let Ok(cam_state) = camera_query.single() else {
        return;
    };

    // Check if corner tiles are spawned
    let corners = [
        (0, 0, "Bottom-Left"),
        (199, 0, "Bottom-Right"),
        (0, 199, "Top-Left"),
        (199, 199, "Top-Right"),
    ];

    let mut corner_status = Vec::new();
    for (x, y, name) in corners {
        let is_spawned = tile_query.iter().any(|pos| pos.x == x && pos.y == y);
        corner_status.push((name, is_spawned));
    }

    // Count total tiles
    let total_tiles = tile_query.iter().count();

    info!(
        "\n=== Corner Visibility Report (zoom: {:.4}) ===",
        cam_state.zoom
    );
    for (name, visible) in &corner_status {
        info!(
            "{}: {}",
            name,
            if *visible {
                "✅ VISIBLE"
            } else {
                "❌ NOT VISIBLE"
            }
        );
    }
    info!("Total tiles spawned: {}/40000", total_tiles);

    // Check if all corners are visible
    let all_visible = corner_status.iter().all(|(_, visible)| *visible);
    if all_visible {
        info!("✅ SUCCESS: All corners are visible!");
    } else {
        error!("❌ FAILURE: Some corners are not visible at minimum zoom!");
    }
}
