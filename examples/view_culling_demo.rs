//! View Culling Demo
//!
//! This example demonstrates the view culling system which only spawns tiles
//! that are visible to the camera, improving performance on large maps.
//!
//! Controls:
//! - WASD/Arrow keys: Move camera
//! - Q/E: Zoom in/out
//! - Mouse wheel: Zoom
//!
//! Watch the console output to see tiles being spawned and despawned
//! as you move the camera around.

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        tiles::{SpawnedTiles, ViewCullingConfig},
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "View Culling Demo".to_string(),
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
                display_stats.run_if(in_state(GameState::Playing)),
                toggle_culling.run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    // Start in playing state
    game_state.set(GameState::Playing);

    // Configure view culling
    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 5,      // Base buffer (will be adjusted dynamically)
        tiles_per_frame: 100, // Spawn up to 100 tiles per frame
        enabled: true,
    });

    // Use a large grid to test dynamic zoom limits
    commands.insert_resource(GridConfig {
        width: 200,
        height: 200,
        tile_size: 64.0,
    });
}

/// Display statistics about spawned tiles
fn display_stats(
    spawned_tiles: Res<SpawnedTiles>,
    culling_config: Res<ViewCullingConfig>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    mut last_count: Local<usize>,
    mut last_scale: Local<f32>,
) {
    let current_count = spawned_tiles.count();

    // Log when count changes or zoom changes significantly
    if let Ok((transform, camera_state)) = camera_query.single() {
        let scale_changed = (transform.scale.x - *last_scale).abs() > 0.01;

        if current_count != *last_count || scale_changed {
            info!(
                "Tiles: {} | Zoom: {:.2} (limits: {:.2}-{:.2}) | Pos: ({:.0}, {:.0}) | Culling: {}",
                current_count,
                transform.scale.x,
                camera_state.min_zoom,
                camera_state.max_zoom,
                transform.translation.x,
                transform.translation.y,
                if culling_config.enabled { "ON" } else { "OFF" }
            );

            *last_count = current_count;
            *last_scale = transform.scale.x;
        }
    }
}

/// Toggle view culling with the C key
fn toggle_culling(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut culling_config: ResMut<ViewCullingConfig>,
    mut pressed: Local<bool>,
) {
    if keyboard.pressed(KeyCode::KeyC) && !*pressed {
        culling_config.enabled = !culling_config.enabled;
        info!(
            "View culling: {}",
            if culling_config.enabled {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        *pressed = true;
    } else if !keyboard.pressed(KeyCode::KeyC) {
        *pressed = false;
    }
}
