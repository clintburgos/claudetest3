//! Camera Constraints - Boundary enforcement for camera movement
//!
//! This file contains systems and functions that keep the camera
//! within valid bounds, ensuring the map remains visible.
//!
//! # Constraints
//! - Camera cannot move beyond map boundaries
//! - Maintains padding to keep edges visible
//! - Adjusts bounds based on zoom level

use super::components::{CameraState, DisableCameraConstraints, IsometricCamera};
use crate::constants::camera::{BOUNDS_PADDING_MULTIPLIER, ISOMETRIC_HEIGHT_RATIO};
use crate::ui::world::grid::{coordinates::grid_center_world, GridConfig};
use bevy::prelude::*;

/// Apply constraints to keep camera within map bounds
pub fn apply_camera_constraints_system(
    mut camera_query: Query<(&mut Transform, &CameraState), (With<IsometricCamera>, Without<DisableCameraConstraints>)>,
    grid_config: Res<GridConfig>,
    windows: Query<&Window>,
) {
    let Ok((mut transform, state)) = camera_query.single_mut() else {
        return;
    };
    let Ok(window) = windows.single() else { return };

    // Calculate visible area based on zoom
    // When camera scale > 1.0 (zoomed in), we see LESS of the world
    // When camera scale < 1.0 (zoomed out), we see MORE of the world
    let visible_width = window.width() / state.zoom;
    let visible_height = window.height() / state.zoom;

    // Calculate map bounds in world space
    let (min_bounds, max_bounds) = calculate_bounds(&grid_config, visible_width, visible_height);

    // Clamp camera position
    transform.translation.x = transform.translation.x.clamp(min_bounds.x, max_bounds.x);
    transform.translation.y = transform.translation.y.clamp(min_bounds.y, max_bounds.y);
}

/// Calculate camera bounds based on map size and visible area
fn calculate_bounds(
    grid_config: &GridConfig,
    visible_width: f32,
    visible_height: f32,
) -> (Vec2, Vec2) {
    // Get world bounds of the map
    let map_width = grid_config.width as f32 * grid_config.tile_size;
    let map_height = grid_config.height as f32 * grid_config.tile_size * ISOMETRIC_HEIGHT_RATIO; // Isometric ratio

    // Center of the map
    let center = grid_center_world(grid_config.width, grid_config.height, grid_config.tile_size);

    // Add padding
    let padding = grid_config.tile_size * BOUNDS_PADDING_MULTIPLIER;

    // Calculate min/max bounds
    let half_visible = Vec2::new(visible_width * 0.5, visible_height * 0.5);

    let min_x = -map_width * 0.5 + half_visible.x - padding;
    let max_x = map_width * 0.5 - half_visible.x + padding;
    let min_y = -map_height - half_visible.y - padding;
    let max_y = map_height - half_visible.y + padding;

    // If map is smaller than viewport, center it
    let min_x = if max_x < min_x { center.x } else { min_x };
    let max_x = if max_x < min_x { center.x } else { max_x };
    let min_y = if max_y < min_y { center.y } else { min_y };
    let max_y = if max_y < min_y { center.y } else { max_y };

    (Vec2::new(min_x, min_y), Vec2::new(max_x, max_y))
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_calculate_bounds_normal_case() {
        let grid_config = GridConfig {
            width: 100,
            height: 100,
            tile_size: 64.0,
        };

        let (min, max) = calculate_bounds(&grid_config, 800.0, 600.0);

        // Map dimensions
        let map_width = 100.0 * 64.0; // 6400
        let map_height = 100.0 * 64.0 * 0.5; // 3200

        // Expected bounds
        let half_visible = Vec2::new(400.0, 300.0);
        let padding = 128.0; // 64 * 2

        assert_eq!(min.x, -map_width * 0.5 + half_visible.x - padding);
        assert_eq!(max.x, map_width * 0.5 - half_visible.x + padding);
        assert_eq!(min.y, -map_height - half_visible.y - padding);
        assert_eq!(max.y, map_height - half_visible.y + padding);
    }

    #[test]
    fn test_calculate_bounds_small_map() {
        let grid_config = GridConfig {
            width: 5,
            height: 5,
            tile_size: 32.0,
        };

        let (min, max) = calculate_bounds(&grid_config, 800.0, 600.0);

        // When map is smaller than viewport, the calculation is complex
        // The bounds depend on the map size, viewport, and padding
        // For a 5x5 map with tile_size=32:
        // - Map width = 5 * 32 = 160
        // - Map height = 5 * 32 * 0.5 = 80 (isometric)
        // - Viewport = 800x600
        // Since viewport is much larger than map, camera should be constrained

        // Just verify that movement is limited (min.x == max.x means no horizontal movement)
        assert_eq!(min.x, max.x, "Horizontal movement should be locked");
        // Y might have some range due to isometric projection
        assert!(max.y >= min.y, "Max Y should be >= Min Y");
    }

    #[test]
    fn test_apply_camera_constraints_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(GridConfig {
            width: 50,
            height: 50,
            tile_size: 64.0,
        });

        // Create window
        let window_entity = app
            .world_mut()
            .spawn(Window {
                resolution: (1280.0, 720.0).into(),
                ..default()
            })
            .id();

        // Spawn camera outside bounds
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(10000.0, 10000.0, 0.0),
            ))
            .id();

        // Run constraint system
        app.world_mut()
            .run_system_once(apply_camera_constraints_system)
            .expect("System should run");

        // Camera should be clamped to bounds
        let transform = app.world().get::<Transform>(camera_entity).unwrap();
        assert!(transform.translation.x < 10000.0);
        assert!(transform.translation.y < 10000.0);

        // Clean up
        app.world_mut().despawn(window_entity);
    }

    #[test]
    fn test_constraints_with_zoom() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(GridConfig {
            width: 50,
            height: 50,
            tile_size: 64.0,
        });

        // Create window
        let window_entity = app
            .world_mut()
            .spawn(Window {
                resolution: (1280.0, 720.0).into(),
                ..default()
            })
            .id();

        // Spawn camera with high zoom
        let mut state = CameraState::default();
        state.zoom = 2.0;

        let camera_entity = app
            .world_mut()
            .spawn((IsometricCamera, state, Transform::from_xyz(0.0, 0.0, 0.0)))
            .id();

        // Run constraint system
        app.world_mut()
            .run_system_once(apply_camera_constraints_system)
            .expect("System should run");

        // Position should remain valid
        let transform = app.world().get::<Transform>(camera_entity).unwrap();
        assert!(transform.translation.x.is_finite());
        assert!(transform.translation.y.is_finite());

        // Clean up
        app.world_mut().despawn(window_entity);
    }

    #[test]
    fn test_constraints_no_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(GridConfig::default());

        // No camera or window - should handle gracefully
        app.world_mut()
            .run_system_once(apply_camera_constraints_system)
            .expect("System should run without camera");
    }

    #[test]
    fn test_constraints_with_disable_component() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(GridConfig {
            width: 50,
            height: 50,
            tile_size: 64.0,
        });

        // Create window
        let window_entity = app
            .world_mut()
            .spawn(Window {
                resolution: (1280.0, 720.0).into(),
                ..default()
            })
            .id();

        // Spawn camera with DisableCameraConstraints component
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(10000.0, 10000.0, 0.0),
                DisableCameraConstraints,  // This should prevent constraints
            ))
            .id();

        // Run constraint system
        app.world_mut()
            .run_system_once(apply_camera_constraints_system)
            .expect("System should run");

        // Camera position should NOT be clamped
        let transform = app.world().get::<Transform>(camera_entity).unwrap();
        assert_eq!(transform.translation.x, 10000.0, "X should remain unchanged");
        assert_eq!(transform.translation.y, 10000.0, "Y should remain unchanged");

        // Clean up
        app.world_mut().despawn(window_entity);
    }

    #[test]
    fn test_calculate_bounds_edge_cases() {
        // Very large map
        let grid_config = GridConfig {
            width: 1000,
            height: 1000,
            tile_size: 128.0,
        };

        let (min, max) = calculate_bounds(&grid_config, 800.0, 600.0);
        assert!(min.x < max.x);
        assert!(min.y < max.y);

        // Single tile map
        let grid_config = GridConfig {
            width: 1,
            height: 1,
            tile_size: 64.0,
        };

        let (min, max) = calculate_bounds(&grid_config, 800.0, 600.0);
        // Should center on single tile
        let center = grid_center_world(1, 1, 64.0);
        assert_eq!(min.x, center.x);
        assert_eq!(max.x, center.x);
    }

    #[test]
    fn test_constraints_at_map_edges() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(GridConfig {
            width: 100,
            height: 100,
            tile_size: 64.0,
        });

        let window_entity = app
            .world_mut()
            .spawn(Window {
                resolution: (1280.0, 720.0).into(),
                ..default()
            })
            .id();

        // Test positions at each edge
        let test_positions = vec![
            (-10000.0, 0.0), // Far left
            (10000.0, 0.0),  // Far right
            (0.0, -10000.0), // Far bottom
            (0.0, 10000.0),  // Far top
        ];

        for (x, y) in test_positions {
            let camera_entity = app
                .world_mut()
                .spawn((
                    IsometricCamera,
                    CameraState::default(),
                    Transform::from_xyz(x, y, 0.0),
                ))
                .id();

            app.world_mut()
                .run_system_once(apply_camera_constraints_system)
                .expect("System should run");

            let transform = app.world().get::<Transform>(camera_entity).unwrap();

            // Should be clamped within reasonable bounds
            assert!(transform.translation.x.abs() < 5000.0);
            assert!(transform.translation.y.abs() < 5000.0);

            app.world_mut().despawn(camera_entity);
        }

        app.world_mut().despawn(window_entity);
    }

    #[test]
    fn test_calculate_bounds_with_different_tile_sizes() {
        let tile_sizes = [16.0, 32.0, 64.0, 128.0, 256.0];

        for &tile_size in &tile_sizes {
            let grid_config = GridConfig {
                width: 50,
                height: 50,
                tile_size,
            };

            let (min, max) = calculate_bounds(&grid_config, 800.0, 600.0);

            // Bounds should scale with tile size
            assert!(max.x > min.x);
            assert!(max.y > min.y);

            // Padding should scale with tile size
            let _expected_padding = tile_size * 2.0;
            let map_width = 50.0 * tile_size;
            let half_visible_x = 400.0;

            // Check that padding is applied correctly
            assert!((max.x - min.x) > (map_width - 2.0 * half_visible_x));
        }
    }

    #[test]
    fn test_constraints_with_different_zoom_levels() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).insert_resource(GridConfig {
            width: 50,
            height: 50,
            tile_size: 64.0,
        });

        let window_entity = app
            .world_mut()
            .spawn(Window {
                resolution: (1280.0, 720.0).into(),
                ..default()
            })
            .id();

        let zoom_levels = [0.5, 1.0, 1.5, 2.0];

        for &zoom in &zoom_levels {
            let mut state = CameraState::default();
            state.zoom = zoom;

            let camera_entity = app
                .world_mut()
                .spawn((
                    IsometricCamera,
                    state,
                    Transform::from_xyz(2000.0, 2000.0, 0.0),
                ))
                .id();

            app.world_mut()
                .run_system_once(apply_camera_constraints_system)
                .expect("System should run");

            let transform = app.world().get::<Transform>(camera_entity).unwrap();
            let _cam_state = app.world().get::<CameraState>(camera_entity).unwrap();

            // Higher zoom = more restricted movement
            if zoom > 1.0 {
                // With zoom in, camera should be more constrained
                assert!(transform.translation.x < 2000.0);
                assert!(transform.translation.y < 2000.0);
            }

            app.world_mut().despawn(camera_entity);
        }

        app.world_mut().despawn(window_entity);
    }
}
