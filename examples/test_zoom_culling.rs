//! Test zoom culling - simple test to verify culling behavior
//!
//! Controls:
//! - Q/E or mouse wheel: Zoom in/out
//! - WASD: Move camera
//! - Space: Toggle culling

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        tiles::{SpawnedTiles, Tile, ViewCullingConfig},
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zoom Culling Test".to_string(),
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
                toggle_culling.run_if(in_state(GameState::Playing)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);

    // Small grid for easier testing
    commands.insert_resource(GridConfig {
        width: 50,
        height: 50,
        tile_size: 64.0,
    });

    // Aggressive culling for testing
    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 2,
        tiles_per_frame: 100,
        enabled: true,
    });
}

fn monitor_system(
    spawned_tiles: Res<SpawnedTiles>,
    tile_query: Query<&Tile>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    mut last_report: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_report < 0.5 {
        return;
    }
    *last_report = now;

    if let Ok((transform, state)) = camera_query.single() {
        let tile_count = tile_query.iter().count();
        info!(
            "Zoom: {:.2} (scale: {:.2}), Pos: ({:.0}, {:.0}), Spawned: {}, Entities: {}",
            state.zoom,
            transform.scale.x,
            transform.translation.x,
            transform.translation.y,
            spawned_tiles.count(),
            tile_count
        );
    }
}

fn toggle_culling(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut culling_config: ResMut<ViewCullingConfig>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        culling_config.enabled = !culling_config.enabled;
        info!(
            "Culling: {}",
            if culling_config.enabled { "ON" } else { "OFF" }
        );
    }
}
