use super::TestScenario;
use crate::ui::world::camera::components::{CameraState, IsometricCamera};
use bevy::prelude::*;

pub fn handle_test_camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    grid_config: Res<crate::ui::world::grid::GridConfig>,
) {
    if let Ok((mut transform, mut camera_state)) = camera_query.single_mut() {
        // F5 - Reset camera to center
        if keyboard.just_pressed(KeyCode::F5) {
            let center = crate::ui::world::grid::coordinates::grid_center_world(
                grid_config.width,
                grid_config.height,
                grid_config.tile_size,
            );
            transform.translation.x = center.x;
            transform.translation.y = center.y;
            camera_state.zoom = 1.0;
            info!("Camera reset to map center at default zoom");
        }

        // F6 - Minimum zoom (zoomed out to see entire map)
        if keyboard.just_pressed(KeyCode::F6) {
            camera_state.zoom = camera_state.min_zoom;
            info!(
                "Camera set to minimum zoom (zoom: {})",
                camera_state.min_zoom
            );
        }

        // F7 - Maximum zoom (zoomed in close)
        if keyboard.just_pressed(KeyCode::F7) {
            camera_state.zoom = camera_state.max_zoom;
            info!(
                "Camera set to maximum zoom (zoom: {})",
                camera_state.max_zoom
            );
        }
    }
}

pub fn apply_test_scenario(
    current_scenario: Res<super::CurrentTestScenario>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
    grid_config: Res<crate::ui::world::grid::GridConfig>,
) {
    if !current_scenario.is_changed() {
        return;
    }

    if let Ok((mut transform, mut camera_state)) = camera_query.single_mut() {
        // Calculate the actual center of the map
        let center = crate::ui::world::grid::coordinates::grid_center_world(
            grid_config.width,
            grid_config.height,
            grid_config.tile_size,
        );

        match current_scenario.0 {
            TestScenario::Normal => {
                // Don't override the camera position in Normal mode
                // Let it stay where the camera setup placed it (map center)
                camera_state.zoom = 1.0;
            }
            TestScenario::MaxZoom => {
                camera_state.zoom = camera_state.max_zoom;
            }
            TestScenario::MinZoom => {
                camera_state.zoom = camera_state.min_zoom;
            }
            TestScenario::EdgeOfMap => {
                // Move to top-right corner of map
                transform.translation.x = 2000.0;
                transform.translation.y = 2000.0;
            }
            TestScenario::CenterOfMap => {
                transform.translation.x = center.x;
                transform.translation.y = center.y;
            }
        }

        info!("Applied test scenario: {:?}", current_scenario.0);
    }
}

pub fn cycle_test_scenarios(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut current_scenario: ResMut<super::CurrentTestScenario>,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        use TestScenario::*;
        current_scenario.0 = match current_scenario.0 {
            Normal => MaxZoom,
            MaxZoom => MinZoom,
            MinZoom => EdgeOfMap,
            EdgeOfMap => CenterOfMap,
            CenterOfMap => Normal,
        };

        info!("Switched to test scenario: {:?}", current_scenario.0);
    }
}
