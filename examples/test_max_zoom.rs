//! Test maximum zoom - should show ~4 tiles

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        tiles::{Tile, ViewCullingConfig},
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Maximum Zoom".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (set_max_zoom, count_visible_tiles).run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);

    commands.insert_resource(GridConfig {
        width: 200,
        height: 200,
        tile_size: 64.0,
    });

    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 5,
        tiles_per_frame: 200,
        enabled: true,
    });
}

fn set_max_zoom(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    if let Ok((mut transform, mut state)) = camera_query.single_mut() {
        // Set to maximum zoom
        state.zoom = state.max_zoom;
        transform.scale = Vec3::splat(state.zoom);

        info!("Set camera to maximum zoom: {:.4}", state.zoom);
        info!("Camera transform scale: {:?}", transform.scale);
        info!("Camera position: {:?}", transform.translation);

        *done = true;
    }
}

fn count_visible_tiles(
    tile_query: Query<Entity, With<Tile>>,
    windows: Query<&Window>,
    camera_query: Query<&CameraState, With<IsometricCamera>>,
    grid_config: Res<GridConfig>,
    mut last_count: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_count < 2.0 {
        return;
    }
    *last_count = now;

    let Ok(window) = windows.single() else {
        return;
    };
    let Ok(state) = camera_query.single() else {
        return;
    };

    let tile_count = tile_query.iter().count();

    // Calculate approximate tiles visible
    let visible_width = window.width() / state.zoom;
    let visible_height = window.height() / state.zoom;
    let tiles_x = visible_width / grid_config.tile_size;
    let tiles_y = visible_height / grid_config.tile_size;

    info!(
        "At max zoom {:.3}: {} tiles visible, window {}x{}, visible area {:.0}x{:.0}, approx {:.1}x{:.1} tiles",
        state.zoom, tile_count,
        window.width() as i32, window.height() as i32,
        visible_width, visible_height,
        tiles_x, tiles_y
    );

    // For isometric view, the effective tile coverage is different
    // A 2x2 grid of tiles forms a diamond that covers roughly the same area
    let effective_tiles = tiles_x * tiles_y;
    info!("Effective tile coverage: ~{:.1} tiles", effective_tiles);
}
