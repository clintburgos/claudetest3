//! Test minimum zoom directly

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        tiles::ViewCullingConfig,
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Minimum Zoom".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            set_min_zoom.run_if(in_state(GameState::Playing)),
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

fn set_min_zoom(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    mut done: Local<bool>,
) {
    if *done {
        return;
    }

    if let Ok((mut transform, mut state)) = camera_query.single_mut() {
        // Set to minimum zoom
        state.zoom = state.min_zoom;
        transform.scale = Vec3::splat(state.zoom);
        
        info!("Set camera to minimum zoom: {:.4}", state.zoom);
        info!("Camera transform scale: {:?}", transform.scale);
        info!("Camera position: {:?}", transform.translation);
        
        *done = true;
    }
}