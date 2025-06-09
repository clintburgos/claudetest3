//! Debug zoom culling - start zoomed out

use bevy::prelude::*;
use claudetest3::{game, logging, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Debug Zoom Culling".to_string(),
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
        .add_systems(OnEnter(game::GameState::Playing), setup_zoomed_out)
        .add_systems(Update, debug_culling_info.run_if(in_state(game::GameState::Playing)))
        .run();
}

fn setup_zoomed_out(
    mut camera_query: Query<&mut ui::world::camera::CameraState, With<ui::world::camera::components::IsometricCamera>>,
) {
    if let Ok(mut camera_state) = camera_query.get_single_mut() {
        camera_state.zoom = 0.2; // Start zoomed out
        info!("Set initial zoom to 0.2");
    }
}

fn debug_culling_info(
    camera_query: Query<(&Transform, &ui::world::camera::CameraState), With<ui::world::camera::components::IsometricCamera>>,
    windows: Query<&Window>,
    grid_config: Res<ui::world::grid::GridConfig>,
    tile_query: Query<&ui::world::tiles::TilePosition, With<ui::world::tiles::Tile>>,
    mut logged: Local<bool>,
) {
    if *logged { return; }
    
    let Ok((camera_transform, camera_state)) = camera_query.get_single() else { return; };
    let Ok(window) = windows.get_single() else { return; };
    
    let camera_scale = camera_state.zoom.max(0.001);
    let visible_width = window.width() / camera_scale;
    let visible_height = window.height() / camera_scale;
    
    let cam_pos = camera_transform.translation;
    
    // Calculate screen bounds
    let screen_min = Vec2::new(
        cam_pos.x - visible_width * 0.5,
        cam_pos.y - visible_height * 0.5
    );
    let screen_max = Vec2::new(
        cam_pos.x + visible_width * 0.5,
        cam_pos.y + visible_height * 0.5
    );
    
    // Find edge tiles
    let mut leftmost = i32::MAX;
    let mut rightmost = i32::MIN;
    let mut topmost = i32::MAX;
    let mut bottommost = i32::MIN;
    
    for tile_pos in tile_query.iter() {
        leftmost = leftmost.min(tile_pos.x);
        rightmost = rightmost.max(tile_pos.x);
        topmost = topmost.min(tile_pos.y);
        bottommost = bottommost.max(tile_pos.y);
    }
    
    // Convert edge tiles to world coordinates
    let left_world = ui::world::grid::coordinates::grid_to_world(leftmost, 100, 0, grid_config.tile_size);
    let right_world = ui::world::grid::coordinates::grid_to_world(rightmost, 100, 0, grid_config.tile_size);
    let top_world = ui::world::grid::coordinates::grid_to_world(100, topmost, 0, grid_config.tile_size);
    let bottom_world = ui::world::grid::coordinates::grid_to_world(100, bottommost, 0, grid_config.tile_size);
    
    info!(
        "Zoom {:.3}: Screen bounds X: {:.0} to {:.0}, Y: {:.0} to {:.0}",
        camera_scale, screen_min.x, screen_max.x, screen_min.y, screen_max.y
    );
    info!(
        "Rendered tiles: X: {} to {}, Y: {} to {}",
        leftmost, rightmost, topmost, bottommost
    );
    info!(
        "Leftmost tile {} world X: {:.0}, Rightmost tile {} world X: {:.0}",
        leftmost, left_world.x, rightmost, right_world.x
    );
    info!(
        "Topmost tile {} world Y: {:.0}, Bottommost tile {} world Y: {:.0}",
        topmost, top_world.y, bottommost, bottom_world.y
    );
    
    *logged = true;
}