//! Simple test - just check if we can see all tiles when zoomed out

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        tiles::{Tile, TilePosition, ViewCullingConfig},
        GridConfig, WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simple Zoom Test".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(OnEnter(GameState::Playing), wait_and_zoom)
        .add_systems(
            Update,
            monitor_tiles.run_if(in_state(GameState::Playing)),
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

    // Disable per-frame limit for this test
    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 10,
        tiles_per_frame: 10000, // Very high to avoid spawn delays
        enabled: true,
    });
}

fn wait_and_zoom(
    mut commands: Commands,
) {
    // Set a timer to zoom out after tiles spawn
    commands.insert_resource(ZoomTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

#[derive(Resource)]
struct ZoomTimer(Timer);

fn monitor_tiles(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    tile_query: Query<&TilePosition, With<Tile>>,
    time: Res<Time>,
    mut timer: Option<ResMut<ZoomTimer>>,
    mut phase: Local<u32>,
) {
    if let Some(ref mut timer) = timer {
        timer.0.tick(time.delta());
        
        if timer.0.just_finished() {
            if let Ok((mut transform, mut state)) = camera_query.single_mut() {
                state.zoom = state.min_zoom;
                transform.scale = Vec3::splat(state.zoom);
                info!("\n=== ZOOMED OUT TO MINIMUM: {:.4} ===", state.zoom);
                *phase = 1;
            }
        }
    }

    // Every second, report tile coverage
    static mut LAST_REPORT: f32 = 0.0;
    let now = time.elapsed_secs();
    unsafe {
        if now - LAST_REPORT > 1.0 {
            LAST_REPORT = now;
            
            let total = tile_query.iter().count();
            let mut corners_found = 0;
            let corners = [(0, 0), (199, 0), (0, 199), (199, 199)];
            
            for pos in tile_query.iter() {
                for &(x, y) in &corners {
                    if pos.x == x && pos.y == y {
                        corners_found += 1;
                    }
                }
            }
            
            if let Ok((_, state)) = camera_query.single() {
                info!(
                    "Phase {}: Zoom={:.4}, Tiles={}, Corners={}/4", 
                    *phase, state.zoom, total, corners_found
                );
                
                if *phase == 1 && corners_found < 4 {
                    error!("FAIL: At minimum zoom, only {} corners visible!", corners_found);
                }
            }
        }
    }
}