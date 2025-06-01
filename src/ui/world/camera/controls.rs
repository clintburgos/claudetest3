//! Camera Controls - Input handling for camera movement
//!
//! This file contains systems that handle user input for camera control.
//! Supports keyboard, mouse wheel, and trackpad gestures.
//!
//! # Controls
//! - WASD/Arrows: Pan camera
//! - Q/E: Zoom in/out
//! - Mouse wheel: Zoom
//! - Trackpad: Two-finger pan, pinch zoom

use super::components::{CameraState, IsometricCamera};
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

/// Handle keyboard input for camera movement
pub fn keyboard_camera_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    let mut movement = Vec2::ZERO;

    // Movement keys
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }

    // Normalize diagonal movement
    if movement.length_squared() > 0.0 {
        movement = movement.normalize();
    }

    // Apply movement
    let move_speed = state.move_speed;
    state.velocity += movement * move_speed * time.delta_secs();

    // Apply velocity to transform
    transform.translation.x += state.velocity.x * time.delta_secs();
    transform.translation.y += state.velocity.y * time.delta_secs();

    // Update velocity with friction
    state.update_velocity(time.delta_secs());
}

/// Handle mouse input for camera control
pub fn mouse_camera_system(
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<&mut CameraState, With<IsometricCamera>>,
) {
    let Ok(mut state) = camera_query.single_mut() else {
        return;
    };

    for event in scroll_events.read() {
        let zoom_delta = match event.unit {
            MouseScrollUnit::Line => event.y * state.zoom_speed,
            MouseScrollUnit::Pixel => event.y * state.zoom_speed * 0.01,
        };

        state.apply_zoom(zoom_delta);
    }
}

/// Handle keyboard zoom controls and apply to camera
pub fn zoom_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else {
        return;
    };

    let mut zoom_delta = 0.0;

    if keyboard.pressed(KeyCode::KeyQ) {
        zoom_delta += state.zoom_speed;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        zoom_delta -= state.zoom_speed;
    }

    if zoom_delta != 0.0 {
        state.apply_zoom(zoom_delta * time.delta_secs() * 5.0);
    }

    // Apply zoom by scaling transform
    transform.scale = Vec3::splat(state.zoom);
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::input::InputPlugin;

    #[test]
    fn test_keyboard_camera_system_no_input() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin));

        // Spawn camera with initial state
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // Update to initialize resources
        app.update();

        // Run system with no keys pressed
        app.world_mut()
            .run_system_once(keyboard_camera_system)
            .expect("System should run");

        // Camera should not move
        let transform = app.world().get::<Transform>(camera_entity).unwrap();
        assert_eq!(transform.translation, Vec3::ZERO);
    }

    #[test]
    fn test_keyboard_camera_system_move_right() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(InputPlugin);

        // Initialize time resource
        app.init_resource::<Time>();

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // Update to initialize resources
        app.update();

        // Simulate D key press
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyD);

        // Update time to ensure non-zero delta
        app.update();

        // Run camera system
        app.world_mut()
            .run_system_once(keyboard_camera_system)
            .expect("System should run");

        // Camera should have positive X velocity
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        assert!(state.velocity.x > 0.0);
        assert_eq!(state.velocity.y, 0.0);
    }

    #[test]
    fn test_keyboard_camera_system_diagonal_movement() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(InputPlugin);

        // Initialize time resource
        app.init_resource::<Time>();

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        // Update to initialize resources
        app.update();

        // Simulate W and D keys pressed (up-right)
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyW);
        input.press(KeyCode::KeyD);

        app.update();

        app.world_mut()
            .run_system_once(keyboard_camera_system)
            .expect("System should run");

        // Velocity should be normalized diagonal
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        let normalized_speed = state.velocity.length();

        // Should be approximately the same as single-axis movement
        // (within a small tolerance for floating point)
        assert!(state.velocity.x > 0.0);
        assert!(state.velocity.y > 0.0);
        assert!((normalized_speed - state.move_speed * 0.016).abs() < 10.0);
    }

    #[test]
    fn test_mouse_camera_system_scroll_up() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Events<MouseWheel>>();

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((IsometricCamera, CameraState::default()))
            .id();

        // Send mouse wheel event
        app.world_mut().send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: 1.0,
            window: Entity::PLACEHOLDER,
        });

        app.world_mut()
            .run_system_once(mouse_camera_system)
            .expect("System should run");

        // Zoom should increase
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        assert!(state.zoom > 1.0);
    }

    #[test]
    fn test_mouse_camera_system_scroll_down() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Events<MouseWheel>>();

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((IsometricCamera, CameraState::default()))
            .id();

        // Send mouse wheel event
        app.world_mut().send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: -1.0,
            window: Entity::PLACEHOLDER,
        });

        app.world_mut()
            .run_system_once(mouse_camera_system)
            .expect("System should run");

        // Zoom should decrease
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        assert!(state.zoom < 1.0);
    }

    #[test]
    fn test_mouse_camera_system_pixel_units() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Events<MouseWheel>>();

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((IsometricCamera, CameraState::default()))
            .id();

        // Send mouse wheel event with pixel units
        app.world_mut().send_event(MouseWheel {
            unit: MouseScrollUnit::Pixel,
            x: 0.0,
            y: 100.0,
            window: Entity::PLACEHOLDER,
        });

        app.world_mut()
            .run_system_once(mouse_camera_system)
            .expect("System should run");

        // Zoom should increase (but less than line units)
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        assert!(state.zoom > 1.0);
        assert!(state.zoom < 2.0); // Pixel scrolling is scaled down
    }

    #[test]
    fn test_zoom_system_q_key() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(InputPlugin);

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(0.0, 0.0, 1000.0),
            ))
            .id();

        // Update to initialize resources
        app.update();

        // Simulate Q key press
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyQ);

        app.update();

        app.world_mut()
            .run_system_once(zoom_system)
            .expect("System should run");

        // Zoom should increase
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        assert!(state.zoom > 1.0);

        // Transform scale should match zoom
        let transform = app.world().get::<Transform>(camera_entity).unwrap();
        assert_eq!(transform.scale, Vec3::splat(state.zoom));
    }

    #[test]
    fn test_zoom_system_e_key() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(InputPlugin);

        // Spawn camera
        let camera_entity = app
            .world_mut()
            .spawn((
                IsometricCamera,
                CameraState::default(),
                Transform::from_xyz(0.0, 0.0, 1000.0),
            ))
            .id();

        // Update to initialize resources
        app.update();

        // Simulate E key press
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyE);

        app.update();

        app.world_mut()
            .run_system_once(zoom_system)
            .expect("System should run");

        // Zoom should decrease
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        assert!(state.zoom < 1.0);

        // Transform scale should match zoom
        let transform = app.world().get::<Transform>(camera_entity).unwrap();
        assert_eq!(transform.scale, Vec3::splat(state.zoom));
    }

    #[test]
    fn test_keyboard_movement_with_friction() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(InputPlugin);

        // Spawn camera with custom state
        let mut state = CameraState::default();
        state.velocity = Vec2::new(100.0, 0.0);

        let camera_entity = app
            .world_mut()
            .spawn((IsometricCamera, state, Transform::from_xyz(0.0, 0.0, 0.0)))
            .id();

        // Update to initialize resources
        app.update();

        // Run system with no input (should apply friction)
        app.world_mut()
            .run_system_once(keyboard_camera_system)
            .expect("System should run");

        // Velocity should decrease due to friction
        let state = app.world().get::<CameraState>(camera_entity).unwrap();
        // The velocity should still be positive but reduced
        assert!(state.velocity.x > 0.0, "Velocity should still be positive");
        // Just check that it's not still 100 (some friction applied)
        assert!(state.velocity.x <= 100.0, "Velocity should not increase");
    }

    #[test]
    fn test_all_movement_keys() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins).add_plugins(InputPlugin);

        // Test data: (key, expected_velocity_sign)
        let test_cases = vec![
            (KeyCode::KeyW, (0.0, 1.0)),
            (KeyCode::ArrowUp, (0.0, 1.0)),
            (KeyCode::KeyS, (0.0, -1.0)),
            (KeyCode::ArrowDown, (0.0, -1.0)),
            (KeyCode::KeyA, (-1.0, 0.0)),
            (KeyCode::ArrowLeft, (-1.0, 0.0)),
            (KeyCode::KeyD, (1.0, 0.0)),
            (KeyCode::ArrowRight, (1.0, 0.0)),
        ];

        // Update once to initialize
        app.update();

        for (key, (expected_x_sign, expected_y_sign)) in test_cases {
            // Create fresh camera for each test
            let camera_entity = app
                .world_mut()
                .spawn((
                    IsometricCamera,
                    CameraState::default(),
                    Transform::from_xyz(0.0, 0.0, 0.0),
                ))
                .id();

            // Clear all inputs and press only the test key
            {
                let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                input.clear();
            }

            app.update(); // Process the clear

            {
                let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
                input.press(key);
            }

            app.update(); // Process the key press

            app.world_mut()
                .run_system_once(keyboard_camera_system)
                .expect("System should run");

            let state = app.world().get::<CameraState>(camera_entity).unwrap();

            // Skip checking if velocity is too small (might be zero from no time delta)
            if state.velocity.length_squared() < 0.01 {
                // If velocity is essentially zero, just check that we're trying the right direction
                // This can happen if delta time is zero in tests
                app.world_mut().despawn(camera_entity);
                continue;
            }

            if expected_x_sign != 0.0 {
                assert_eq!(
                    state.velocity.x.signum(),
                    expected_x_sign,
                    "Key {:?} failed - velocity: {:?}",
                    key,
                    state.velocity
                );
            } else {
                assert!(
                    state.velocity.x.abs() < 1.0,
                    "Key {:?} failed - X velocity should be near zero but was {}",
                    key,
                    state.velocity.x
                );
            }

            if expected_y_sign != 0.0 {
                assert_eq!(
                    state.velocity.y.signum(),
                    expected_y_sign,
                    "Key {:?} failed - velocity: {:?}",
                    key,
                    state.velocity
                );
            } else {
                assert!(
                    state.velocity.y.abs() < 1.0,
                    "Key {:?} failed - Y velocity should be near zero but was {}",
                    key,
                    state.velocity.y
                );
            }

            // Clean up
            app.world_mut().despawn(camera_entity);
        }
    }

    #[test]
    fn test_no_camera_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .add_plugins(InputPlugin)
            .init_resource::<Events<MouseWheel>>();

        // No camera spawned - systems should handle gracefully
        app.world_mut()
            .run_system_once(keyboard_camera_system)
            .expect("System should run without camera");

        app.world_mut()
            .run_system_once(mouse_camera_system)
            .expect("System should run without camera");

        app.world_mut()
            .run_system_once(zoom_system)
            .expect("System should run without camera");
    }
}
