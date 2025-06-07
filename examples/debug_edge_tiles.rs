//! Debug edge tile visibility issue

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        grid::{
            coordinates::{grid_to_world, world_to_grid},
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
                title: "Debug Edge Tiles".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (manual_zoom_control, debug_visibility).run_if(in_state(GameState::Playing)),
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
        buffer_tiles: 10,
        tiles_per_frame: 200,
        enabled: true,
    });
}

fn manual_zoom_control(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    // Manual zoom with finer control
    let zoom_speed = 0.02;
    if keyboard.pressed(KeyCode::KeyQ) {
        state.zoom = (state.zoom + zoom_speed * time.delta_secs()).min(state.max_zoom);
        transform.scale = Vec3::splat(state.zoom);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        state.zoom = (state.zoom - zoom_speed * time.delta_secs()).max(state.min_zoom);
        transform.scale = Vec3::splat(state.zoom);
    }

    // Direct zoom levels
    if keyboard.just_pressed(KeyCode::Digit1) {
        state.zoom = state.min_zoom;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set to MIN zoom: {:.4}", state.zoom);
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        state.zoom = 0.1;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.1");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        state.zoom = 0.5;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.5");
    }
}

fn debug_visibility(
    tile_query: Query<(&TilePosition, &GlobalTransform), With<Tile>>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    windows: Query<&Window>,
    grid_config: Res<GridConfig>,
    mut last_debug: Local<f32>,
    time: Res<Time>,
) {
    let now = time.elapsed_secs();
    if now - *last_debug < 1.0 {
        return;
    }
    *last_debug = now;

    let Ok((cam_transform, cam_state)) = camera_query.single() else {
        return;
    };
    let Ok(window) = windows.single() else {
        return;
    };

    // Key corner tiles to check
    let corners = [
        (0, 0, "Origin"),
        (199, 0, "Bottom-Right"),
        (0, 199, "Top-Left"),
        (199, 199, "Top-Right"),
    ];

    // Check which corners are spawned
    let mut found_corners = vec![];
    for (pos, _) in tile_query.iter() {
        for &(x, y, name) in &corners {
            if pos.x == x && pos.y == y {
                found_corners.push(name);
            }
        }
    }

    // Calculate what the view culling system sees
    let visible_width = window.width() / cam_state.zoom;
    let visible_height = window.height() / cam_state.zoom;

    let cam_x = cam_transform.translation.x;
    let cam_y = cam_transform.translation.y;

    let left = cam_x - visible_width * 0.5;
    let right = cam_x + visible_width * 0.5;
    let bottom = cam_y - visible_height * 0.5;
    let top = cam_y + visible_height * 0.5;

    // Convert world bounds to grid to see what tiles SHOULD be visible
    let (grid_min_x, grid_min_y, _) =
        world_to_grid(Vec3::new(left, bottom, 0.0), grid_config.tile_size);
    let (grid_max_x, grid_max_y, _) =
        world_to_grid(Vec3::new(right, top, 0.0), grid_config.tile_size);

    info!("\n=== Debug at zoom {:.4} ===", cam_state.zoom);
    info!("Camera pos: ({:.0}, {:.0})", cam_x, cam_y);
    info!(
        "Visible world area: ({:.0}, {:.0}) to ({:.0}, {:.0})",
        left, bottom, right, top
    );
    info!(
        "Grid conversion: ({}, {}) to ({}, {})",
        grid_min_x, grid_min_y, grid_max_x, grid_max_y
    );
    info!("Corners found: {:?}", found_corners);

    // Check if corner world positions are within visible bounds
    for &(x, y, name) in &corners {
        let world_pos = grid_to_world(x, y, 0, grid_config.tile_size);
        let in_bounds = world_pos.x >= left
            && world_pos.x <= right
            && world_pos.y >= bottom
            && world_pos.y <= top;
        info!(
            "{} at grid({},{}) -> world({:.0},{:.0}) - in view: {}",
            name, x, y, world_pos.x, world_pos.y, in_bounds
        );
    }

    info!("Total tiles spawned: {}", tile_query.iter().count());
}
