//! Simple test to reproduce the zoom issue

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        tiles::{Tile, TilePosition},
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Zoom Issue".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (control_zoom, check_corners).run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);

    // 20x20 map for easier debugging
    commands.insert_resource(GridConfig {
        width: 20,
        height: 20,
        tile_size: 64.0,
    });
}

fn control_zoom(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut last_zoom: Local<f32>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Digit1) {
        state.zoom = state.min_zoom;
        transform.scale = Vec3::splat(state.zoom);
        info!("\n=== SET TO MIN ZOOM: {:.4} ===", state.zoom);
    }

    if keyboard.just_pressed(KeyCode::Digit2) {
        state.zoom = 1.0;
        transform.scale = Vec3::splat(state.zoom);
        info!("\n=== SET TO DEFAULT ZOOM: 1.0 ===");
    }

    if state.zoom != *last_zoom {
        *last_zoom = state.zoom;
        info!(
            "Current zoom: {:.4}, scale: {:?}",
            state.zoom, transform.scale
        );
    }
}

fn check_corners(
    tile_query: Query<&TilePosition, With<Tile>>,
    mut last_check: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_check < 0.5 {
        return;
    }
    *last_check = now;

    // Check for corner tiles in 20x20 grid
    let corners = [(0, 0), (19, 0), (0, 19), (19, 19)];
    let mut found = vec![];

    for pos in tile_query.iter() {
        for &(x, y) in &corners {
            if pos.x == x && pos.y == y {
                found.push((x, y));
            }
        }
    }

    if found.len() != 4 {
        warn!(
            "Only {} of 4 corners visible! Found: {:?}",
            found.len(),
            found
        );
    } else {
        info!("All 4 corners visible ✓");
    }
}
