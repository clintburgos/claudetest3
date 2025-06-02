//! Debug isometric culling issue

use bevy::prelude::*;
use claudetest3::game::{GameState, GameStatePlugin};
use claudetest3::ui::{
    panels::UIPanelsPlugin,
    world::{
        camera::{CameraState, IsometricCamera},
        grid::{coordinates::{grid_to_world, world_to_grid}, GridConfig},
        tiles::{Tile, TilePosition, ViewCullingConfig},
        WorldPlugin,
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Debug Isometric Culling".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((GameStatePlugin, WorldPlugin, UIPanelsPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (zoom_control, debug_culling).run_if(in_state(GameState::Playing)),
        )
        .run();
}

fn setup(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Playing);

    // Small map for easier debugging
    commands.insert_resource(GridConfig {
        width: 50,
        height: 50,
        tile_size: 64.0,
    });

    commands.insert_resource(ViewCullingConfig {
        buffer_tiles: 2,
        tiles_per_frame: 5000,
        enabled: true,
    });
}

fn zoom_control(
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    if keyboard.just_pressed(KeyCode::Digit1) {
        state.zoom = 0.2;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.2");
    }
    if keyboard.just_pressed(KeyCode::Digit2) {
        state.zoom = 0.3;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.3");
    }
    if keyboard.just_pressed(KeyCode::Digit3) {
        state.zoom = 0.4;
        transform.scale = Vec3::splat(state.zoom);
        info!("Set zoom to 0.4");
    }
}

fn debug_culling(
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    tile_query: Query<&TilePosition, With<Tile>>,
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

    let Ok((cam_transform, cam_state)) = camera_query.single() else { return; };
    let Ok(window) = windows.single() else { return; };

    // Calculate what the culling system sees
    let visible_width = window.width() / cam_state.zoom;
    let visible_height = window.height() / cam_state.zoom;
    
    let cam_pos = cam_transform.translation;
    let left = cam_pos.x - visible_width * 0.5;
    let right = cam_pos.x + visible_width * 0.5;
    let bottom = cam_pos.y - visible_height * 0.5;
    let top = cam_pos.y + visible_height * 0.5;

    // Check actual spawned tiles
    let mut min_spawned_x = 50;
    let mut max_spawned_x = 0;
    let mut min_spawned_y = 50;
    let mut max_spawned_y = 0;
    let mut edge_tiles = vec![];

    for pos in tile_query.iter() {
        min_spawned_x = min_spawned_x.min(pos.x);
        max_spawned_x = max_spawned_x.max(pos.x);
        min_spawned_y = min_spawned_y.min(pos.y);
        max_spawned_y = max_spawned_y.max(pos.y);

        // Check if it's an edge tile
        if pos.x == 0 || pos.x == 49 || pos.y == 0 || pos.y == 49 {
            edge_tiles.push((pos.x, pos.y));
        }
    }

    // Check map corners in world space
    let corners = [
        (0, 0, "Bottom-Left"),
        (49, 0, "Bottom-Right"),
        (0, 49, "Top-Left"),
        (49, 49, "Top-Right"),
    ];

    info!("\n=== Culling Debug at zoom {:.3} ===", cam_state.zoom);
    info!("Camera at ({:.0}, {:.0})", cam_pos.x, cam_pos.y);
    info!("Visible world: ({:.0},{:.0}) to ({:.0},{:.0})", left, bottom, right, top);
    info!("Spawned tiles: X({}-{}), Y({}-{})", min_spawned_x, max_spawned_x, min_spawned_y, max_spawned_y);
    info!("Edge tiles visible: {}", edge_tiles.len());

    // Check if corners are within visible world bounds
    for (x, y, name) in corners {
        let world_pos = grid_to_world(x, y, 0, grid_config.tile_size);
        let in_view = world_pos.x >= left && world_pos.x <= right && 
                     world_pos.y >= bottom && world_pos.y <= top;
        
        // Check if actually spawned
        let is_spawned = edge_tiles.iter().any(|(tx, ty)| *tx == x && *ty == y);
        
        if in_view && !is_spawned {
            warn!("{} corner SHOULD be visible but ISN'T spawned!", name);
        } else if !in_view && is_spawned {
            warn!("{} corner SHOULDN'T be visible but IS spawned!", name);
        }
    }

    // Calculate what grid bounds the culling thinks it needs
    let test_points = [
        Vec3::new(left, bottom, 0.0),
        Vec3::new(right, top, 0.0),
        Vec3::new(left, top, 0.0),
        Vec3::new(right, bottom, 0.0),
        Vec3::new(cam_pos.x, bottom, 0.0),
        Vec3::new(cam_pos.x, top, 0.0),
        Vec3::new(left, cam_pos.y, 0.0),
        Vec3::new(right, cam_pos.y, 0.0),
    ];

    let mut calc_min_x = i32::MAX;
    let mut calc_max_x = i32::MIN;
    let mut calc_min_y = i32::MAX;
    let mut calc_max_y = i32::MIN;

    for point in &test_points {
        let (gx, gy, _) = world_to_grid(*point, grid_config.tile_size);
        calc_min_x = calc_min_x.min(gx);
        calc_max_x = calc_max_x.max(gx);
        calc_min_y = calc_min_y.min(gy);
        calc_max_y = calc_max_y.max(gy);
    }

    info!("Calculated grid bounds: X({}-{}), Y({}-{})", calc_min_x, calc_max_x, calc_min_y, calc_max_y);
    
    if calc_min_x < 0 || calc_max_x >= 50 || calc_min_y < 0 || calc_max_y >= 50 {
        info!("⚠️  Calculated bounds exceed map size!");
    }
}