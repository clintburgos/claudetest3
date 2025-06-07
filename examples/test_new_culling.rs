//! Test the new isometric culling implementation

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        grid::{
            coordinates::{grid_to_world, tile_intersects_rect},
            GridConfig,
        },
        tiles::{Tile, TilePosition, ViewCullingConfig},
        WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test New Culling Implementation".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (automatic_zoom_test, verify_culling).run_if(in_state(GameState::Playing)),
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

    // Insert test state
    commands.insert_resource(TestState {
        current_zoom: 5.0,
        test_phase: TestPhase::ZoomingOut,
        time_since_change: 0.0,
        results: Vec::new(),
    });
}

#[derive(Resource)]
struct TestState {
    current_zoom: f32,
    test_phase: TestPhase,
    time_since_change: f32,
    results: Vec<TestResult>,
}

#[derive(Debug, Clone)]
enum TestPhase {
    ZoomingOut,
    TestingMinZoom,
    Complete,
}

#[derive(Debug, Clone)]
struct TestResult {
    zoom: f32,
    missing_corners: Vec<String>,
    visible_tiles: (i32, i32, i32, i32),
    camera_pos: Vec3,
}

fn automatic_zoom_test(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    mut test_state: ResMut<TestState>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    test_state.time_since_change += time.delta_secs();

    match test_state.test_phase {
        TestPhase::ZoomingOut => {
            // Change zoom every 2 seconds
            if test_state.time_since_change > 2.0 {
                test_state.current_zoom = (test_state.current_zoom - 0.5).max(state.min_zoom);
                state.zoom = test_state.current_zoom;
                transform.scale = Vec3::splat(state.zoom);
                test_state.time_since_change = 0.0;

                info!("Testing zoom level: {:.3}", test_state.current_zoom);

                if test_state.current_zoom <= state.min_zoom {
                    test_state.test_phase = TestPhase::TestingMinZoom;
                    info!("Reached minimum zoom: {:.3}", state.min_zoom);
                }
            }
        }
        TestPhase::TestingMinZoom => {
            // Stay at min zoom for 5 seconds
            if test_state.time_since_change > 5.0 {
                test_state.test_phase = TestPhase::Complete;

                // Print summary
                info!("\n=== CULLING TEST COMPLETE ===");
                for result in &test_state.results {
                    if !result.missing_corners.is_empty() {
                        error!(
                            "FAIL at zoom {:.3}: Missing corners: {:?}",
                            result.zoom, result.missing_corners
                        );
                    }
                }

                let failures = test_state
                    .results
                    .iter()
                    .filter(|r| !r.missing_corners.is_empty())
                    .count();

                if failures == 0 {
                    info!("✅ ALL TESTS PASSED! No tiles disappeared at any zoom level.");
                } else {
                    error!(
                        "❌ {} TESTS FAILED! Tiles disappeared at map edges.",
                        failures
                    );
                }
            }
        }
        TestPhase::Complete => {
            // Do nothing
        }
    }
}

fn verify_culling(
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    tile_query: Query<&TilePosition, With<Tile>>,
    windows: Query<&Window>,
    grid_config: Res<GridConfig>,
    mut test_state: ResMut<TestState>,
    mut last_check: Local<f32>,
    time: Res<Time>,
) {
    if matches!(test_state.test_phase, TestPhase::Complete) {
        return;
    }

    let now = time.elapsed_secs();
    if now - *last_check < 0.5 {
        return;
    }
    *last_check = now;

    let Ok((cam_transform, cam_state)) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    // Calculate visible bounds
    let visible_width = window.width() / cam_state.zoom;
    let visible_height = window.height() / cam_state.zoom;

    let cam_pos = cam_transform.translation;
    let visible_min = Vec2::new(
        cam_pos.x - visible_width * 0.5,
        cam_pos.y - visible_height * 0.5,
    );
    let visible_max = Vec2::new(
        cam_pos.x + visible_width * 0.5,
        cam_pos.y + visible_height * 0.5,
    );

    // Check map corners
    let corners = [
        (0, 0, "Bottom-Left"),
        (199, 0, "Bottom-Right"),
        (0, 199, "Top-Left"),
        (199, 199, "Top-Right"),
    ];

    let mut missing_corners = Vec::new();
    let mut visible_bounds = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

    // Count spawned tiles
    for pos in tile_query.iter() {
        visible_bounds.0 = visible_bounds.0.min(pos.x);
        visible_bounds.1 = visible_bounds.1.min(pos.y);
        visible_bounds.2 = visible_bounds.2.max(pos.x);
        visible_bounds.3 = visible_bounds.3.max(pos.y);
    }

    // Check each corner
    for (x, y, name) in corners {
        // Check if corner SHOULD be visible using our algorithm
        let should_be_visible =
            tile_intersects_rect(x, y, grid_config.tile_size, visible_min, visible_max);

        // Check if it's actually spawned
        let is_spawned = tile_query.iter().any(|pos| pos.x == x && pos.y == y);

        if should_be_visible && !is_spawned {
            missing_corners.push(format!("{} at ({},{})", name, x, y));
            warn!(
                "Corner {} SHOULD be visible but ISN'T spawned! (zoom: {:.3})",
                name, cam_state.zoom
            );
        }
    }

    // Record result
    if !missing_corners.is_empty() || cam_state.zoom < 0.2 {
        test_state.results.push(TestResult {
            zoom: cam_state.zoom,
            missing_corners: missing_corners.clone(),
            visible_tiles: visible_bounds,
            camera_pos: cam_pos,
        });
    }

    // Log current state
    info!(
        "Zoom: {:.3}, Visible tiles: ({}-{}, {}-{}), Missing: {}",
        cam_state.zoom,
        visible_bounds.0,
        visible_bounds.2,
        visible_bounds.1,
        visible_bounds.3,
        if missing_corners.is_empty() {
            "None".to_string()
        } else {
            format!("{:?}", missing_corners)
        }
    );
}
