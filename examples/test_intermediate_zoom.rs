//! Test intermediate zoom levels where edges might disappear

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
                title: "Test Intermediate Zoom".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (test_zoom_levels, check_edge_tiles).run_if(in_state(GameState::Playing)),
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
}

fn test_zoom_levels(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    // Test specific zoom levels
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.zoom = 0.084; // Min zoom
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.084 (min)");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        state.zoom = 0.15; // Threshold
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.15 (threshold)");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        state.zoom = 0.2; // Just above threshold
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.2");
    }
    if keyboard.just_pressed(KeyCode::Digit4) {
        state.zoom = 0.3;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.3");
    }
    if keyboard.just_pressed(KeyCode::Digit5) {
        state.zoom = 0.5;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.5");
    }
}

fn check_edge_tiles(
    tile_query: Query<&TilePosition, With<Tile>>,
    camera_query: Query<&CameraState, With<IsometricCamera>>,
    mut last_check: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_check < 1.0 {
        return;
    }
    *last_check = now;

    let Ok(state) = camera_query.single() else { return; };

    // Check edge tiles
    let mut edges = (false, false, false, false); // left, right, top, bottom
    let mut min_x = 200;
    let mut max_x = 0;
    let mut min_y = 200;
    let mut max_y = 0;

    for pos in tile_query.iter() {
        min_x = min_x.min(pos.x);
        max_x = max_x.max(pos.x);
        min_y = min_y.min(pos.y);
        max_y = max_y.max(pos.y);

        if pos.x == 0 { edges.0 = true; }
        if pos.x == 199 { edges.1 = true; }
        if pos.y == 0 { edges.2 = true; }
        if pos.y == 199 { edges.3 = true; }
    }

    info!(
        "Zoom {:.3}: Tile range X({}-{}), Y({}-{}). Edges visible: Left={}, Right={}, Bottom={}, Top={}",
        state.zoom, min_x, max_x, min_y, max_y,
        edges.0, edges.1, edges.2, edges.3
    );

    if !edges.0 || !edges.1 || !edges.2 || !edges.3 {
        warn!("⚠️  NOT ALL EDGES VISIBLE at zoom {:.3}!", state.zoom);
    }
}