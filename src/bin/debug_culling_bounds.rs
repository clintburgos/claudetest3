//! Debug program to visualize culling bounds

use bevy::prelude::*;
use claudetest3::{game, logging, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Debug Culling Bounds".to_string(),
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
        .add_systems(Update, debug_culling_info.run_if(in_state(game::GameState::Playing)))
        .run();
}

fn debug_culling_info(
    camera_query: Query<(&Transform, &ui::world::camera::CameraState), With<ui::world::camera::components::IsometricCamera>>,
    windows: Query<&Window>,
    grid_config: Res<ui::world::grid::GridConfig>,
    culling_config: Res<ui::world::tiles::ViewCullingConfig>,
) {
    let Ok((camera_transform, camera_state)) = camera_query.get_single() else { return; };
    let Ok(window) = windows.get_single() else { return; };
    
    let camera_scale = camera_state.zoom.max(0.001);
    let visible_width = window.width() / camera_scale;
    let visible_height = window.height() / camera_scale;
    
    let cam_pos = camera_transform.translation;
    
    // Calculate what the culling system sees
    let margin = grid_config.tile_size * 2.0;
    let visible_min = Vec2::new(
        cam_pos.x - visible_width * 0.5 - margin,
        cam_pos.y - visible_height * 0.5 - margin
    );
    let visible_max = Vec2::new(
        cam_pos.x + visible_width * 0.5 + margin,
        cam_pos.y + visible_height * 0.5 + margin
    );
    
    // Calculate actual screen bounds (what the user sees)
    let screen_min = Vec2::new(
        cam_pos.x - visible_width * 0.5,
        cam_pos.y - visible_height * 0.5
    );
    let screen_max = Vec2::new(
        cam_pos.x + visible_width * 0.5,
        cam_pos.y + visible_height * 0.5
    );
    
    info!(
        "Culling Debug: zoom={:.3}, cam=({:.0},{:.0}), screen_bounds=({:.0},{:.0})-({:.0},{:.0}), culling_bounds=({:.0},{:.0})-({:.0},{:.0}), margin={:.0}",
        camera_scale,
        cam_pos.x, cam_pos.y,
        screen_min.x, screen_min.y, screen_max.x, screen_max.y,
        visible_min.x, visible_min.y, visible_max.x, visible_max.y,
        margin
    );
}