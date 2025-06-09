//! Debug program to visualize tile culling issues

use bevy::prelude::*;
use claudetest3::{game, logging, ui};
use claudetest3::ui::world::tiles::{Tile, TilePosition};
use claudetest3::ui::world::grid::coordinates::{grid_to_world, world_to_grid};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Debug Visible Tiles".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_plugins((
            game::GameStatePlugin,
            logging::LoggingPlugin,
            ui::world::WorldPlugin,
            ui::panels::UIPanelsPlugin,
        ))
        .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
            next_state.set(game::GameState::Playing);
        })
        .add_systems(Update, debug_visible_tiles.run_if(in_state(game::GameState::Playing)))
        .run();
}

fn debug_visible_tiles(
    camera_query: Query<(&Transform, &ui::world::camera::CameraState), With<ui::world::camera::components::IsometricCamera>>,
    windows: Query<&Window>,
    grid_config: Res<ui::world::grid::GridConfig>,
    tile_query: Query<&TilePosition, With<Tile>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut last_debug: Local<f32>,
    time: Res<Time>,
) {
    // Only debug every second or when D is pressed
    *last_debug += time.delta_secs();
    if *last_debug < 1.0 && !keyboard.just_pressed(KeyCode::KeyD) {
        return;
    }
    *last_debug = 0.0;

    let Ok((camera_transform, camera_state)) = camera_query.get_single() else { return; };
    let Ok(window) = windows.get_single() else { return; };
    
    let camera_pos = camera_transform.translation;
    let zoom = camera_state.zoom;
    
    // Calculate what should be visible
    let visible_width = window.width() / zoom;
    let visible_height = window.height() / zoom;
    
    // Screen bounds in world space
    let screen_min = Vec2::new(
        camera_pos.x - visible_width * 0.5,
        camera_pos.y - visible_height * 0.5
    );
    let screen_max = Vec2::new(
        camera_pos.x + visible_width * 0.5,
        camera_pos.y + visible_height * 0.5
    );
    
    // Convert screen corners to grid coordinates
    let corners = [
        Vec3::new(screen_min.x, screen_min.y, 0.0),
        Vec3::new(screen_max.x, screen_min.y, 0.0),
        Vec3::new(screen_max.x, screen_max.y, 0.0),
        Vec3::new(screen_min.x, screen_max.y, 0.0),
    ];
    
    info!("=== Debug Visible Tiles ===");
    info!("Camera: pos=({:.0}, {:.0}), zoom={:.3}", camera_pos.x, camera_pos.y, zoom);
    info!("Screen bounds: ({:.0}, {:.0}) to ({:.0}, {:.0})", 
        screen_min.x, screen_min.y, screen_max.x, screen_max.y);
    
    // Find expected tile bounds
    let mut expected_min_x = i32::MAX;
    let mut expected_max_x = i32::MIN;
    let mut expected_min_y = i32::MAX;
    let mut expected_max_y = i32::MIN;
    
    for corner in &corners {
        let (gx, gy, _) = world_to_grid(*corner, grid_config.tile_size);
        expected_min_x = expected_min_x.min(gx);
        expected_max_x = expected_max_x.max(gx);
        expected_min_y = expected_min_y.min(gy);
        expected_max_y = expected_max_y.max(gy);
    }
    
    info!("Expected grid bounds (from corners): ({}, {}) to ({}, {})",
        expected_min_x, expected_min_y, expected_max_x, expected_max_y);
    
    // Count actual rendered tiles
    let mut actual_min_x = i32::MAX;
    let mut actual_max_x = i32::MIN;
    let mut actual_min_y = i32::MAX;
    let mut actual_max_y = i32::MIN;
    let mut tile_count = 0;
    
    for tile_pos in tile_query.iter() {
        tile_count += 1;
        actual_min_x = actual_min_x.min(tile_pos.x);
        actual_max_x = actual_max_x.max(tile_pos.x);
        actual_min_y = actual_min_y.min(tile_pos.y);
        actual_max_y = actual_max_y.max(tile_pos.y);
    }
    
    info!("Actual rendered tiles: {} tiles", tile_count);
    info!("Actual grid bounds: ({}, {}) to ({}, {})",
        actual_min_x, actual_min_y, actual_max_x, actual_max_y);
    
    // Check for missing tiles at edges
    if actual_min_x > expected_min_x || actual_max_x < expected_max_x ||
       actual_min_y > expected_min_y || actual_max_y < expected_max_y {
        warn!("MISSING EDGE TILES!");
        warn!("Missing on left: {} tiles", expected_min_x.saturating_sub(actual_min_x));
        warn!("Missing on right: {} tiles", expected_max_x.saturating_sub(actual_max_x));
        warn!("Missing on top: {} tiles", expected_min_y.saturating_sub(actual_min_y));
        warn!("Missing on bottom: {} tiles", expected_max_y.saturating_sub(actual_max_y));
    }
    
    // Check specific edge tiles
    let edge_tiles = [
        (expected_min_x, (expected_min_y + expected_max_y) / 2, "Left edge"),
        (expected_max_x, (expected_min_y + expected_max_y) / 2, "Right edge"),
        ((expected_min_x + expected_max_x) / 2, expected_min_y, "Top edge"),
        ((expected_min_x + expected_max_x) / 2, expected_max_y, "Bottom edge"),
    ];
    
    for (x, y, label) in edge_tiles {
        let world_pos = grid_to_world(x, y, 0, grid_config.tile_size);
        let on_screen = world_pos.x >= screen_min.x && world_pos.x <= screen_max.x &&
                       world_pos.y >= screen_min.y && world_pos.y <= screen_max.y;
        info!("{} tile ({}, {}) world pos: ({:.0}, {:.0}) - on screen: {}", 
            label, x, y, world_pos.x, world_pos.y, on_screen);
    }
    
    info!("Press D to debug again");
}