//! Camera Components - Data structures for camera state
//!
//! This file defines components for the isometric camera system.
//! The camera uses orthographic projection suitable for 2D isometric views.
//!
//! # Design Notes
//! - CameraState tracks zoom and movement velocity
//! - Zoom is stored as a scale factor (1.0 = default)
//! - Velocity enables smooth camera movement

use crate::constants::camera::*;
use bevy::prelude::*;

/// Marker component for the isometric camera
#[derive(Component)]
pub struct IsometricCamera;

/// Camera state including zoom and movement
#[derive(Component)]
pub struct CameraState {
    /// Current zoom level (1.0 = default, >1 = zoomed in, <1 = zoomed out)
    pub zoom: f32,
    /// Minimum zoom level
    pub min_zoom: f32,
    /// Maximum zoom level
    pub max_zoom: f32,
    /// Camera movement velocity for smooth motion
    pub velocity: Vec2,
    /// Movement speed multiplier
    pub move_speed: f32,
    /// Zoom speed multiplier
    pub zoom_speed: f32,
    /// Deceleration factor for smooth stopping
    pub friction: f32,
}

impl Default for CameraState {
    fn default() -> Self {
        Self {
            zoom: DEFAULT_ZOOM,
            min_zoom: MIN_ZOOM,
            max_zoom: MAX_ZOOM,
            velocity: Vec2::ZERO,
            move_speed: DEFAULT_MOVE_SPEED,
            zoom_speed: DEFAULT_ZOOM_SPEED,
            friction: DEFAULT_FRICTION,
        }
    }
}

impl CameraState {
    /// Apply zoom within limits
    pub fn apply_zoom(&mut self, delta: f32) {
        self.zoom = (self.zoom + delta).clamp(self.min_zoom, self.max_zoom);
    }

    /// Update velocity with friction
    pub fn update_velocity(&mut self, delta_time: f32) {
        self.velocity *= self.friction.powf(delta_time * FRICTION_FPS_BASE);

        // Stop if velocity is very small
        if self.velocity.length_squared() < VELOCITY_STOP_THRESHOLD {
            self.velocity = Vec2::ZERO;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_state_default() {
        let state = CameraState::default();
        assert_eq!(state.zoom, DEFAULT_ZOOM);
        assert_eq!(state.min_zoom, MIN_ZOOM);
        assert_eq!(state.max_zoom, MAX_ZOOM);
        assert_eq!(state.velocity, Vec2::ZERO);
        assert_eq!(state.move_speed, DEFAULT_MOVE_SPEED);
        assert_eq!(state.zoom_speed, DEFAULT_ZOOM_SPEED);
        assert_eq!(state.friction, DEFAULT_FRICTION);
    }

    #[test]
    fn test_apply_zoom_within_limits() {
        let mut state = CameraState::default();

        // Test zoom in
        state.apply_zoom(0.5);
        assert_eq!(state.zoom, 1.5);

        // Test zoom out
        state.apply_zoom(-0.5);
        assert_eq!(state.zoom, 1.0);
    }

    #[test]
    fn test_apply_zoom_clamps_to_max() {
        let mut state = CameraState::default();

        // Try to zoom beyond max
        state.apply_zoom(10.0);
        assert_eq!(state.zoom, state.max_zoom);
    }

    #[test]
    fn test_apply_zoom_clamps_to_min() {
        let mut state = CameraState::default();

        // Try to zoom below min
        state.apply_zoom(-10.0);
        assert_eq!(state.zoom, state.min_zoom);
    }

    #[test]
    fn test_update_velocity_applies_friction() {
        let mut state = CameraState::default();
        state.velocity = Vec2::new(100.0, 100.0);

        let initial_speed = state.velocity.length();
        state.update_velocity(1.0 / 60.0); // One frame at 60 FPS

        let new_speed = state.velocity.length();
        assert!(new_speed < initial_speed);
        assert!(new_speed > 0.0);
    }

    #[test]
    fn test_update_velocity_zeros_small_values() {
        let mut state = CameraState::default();
        state.velocity = Vec2::new(0.001, 0.001);

        state.update_velocity(1.0 / 60.0);

        assert_eq!(state.velocity, Vec2::ZERO);
    }

    #[test]
    fn test_update_velocity_with_different_delta_times() {
        let mut state1 = CameraState::default();
        let mut state2 = CameraState::default();

        state1.velocity = Vec2::new(100.0, 100.0);
        state2.velocity = Vec2::new(100.0, 100.0);

        // Update with different delta times
        state1.update_velocity(1.0 / 60.0); // 60 FPS
        state2.update_velocity(1.0 / 30.0); // 30 FPS

        // Both should have reduced velocity
        assert!(state1.velocity.length() < 100.0 * std::f32::consts::SQRT_2);
        assert!(state2.velocity.length() < 100.0 * std::f32::consts::SQRT_2);

        // Longer frame time = more friction applied
        assert!(state2.velocity.length() < state1.velocity.length());
    }

    #[test]
    fn test_isometric_camera_marker_component() {
        // Test that IsometricCamera can be used as a component
        // This is a compile-time test more than runtime
        let _camera = IsometricCamera;
    }

    #[test]
    fn test_camera_state_custom_values() {
        let state = CameraState {
            zoom: 1.5,
            min_zoom: 0.25,
            max_zoom: 4.0,
            velocity: Vec2::new(50.0, -50.0),
            move_speed: 300.0,
            zoom_speed: 0.2,
            friction: 0.85,
        };

        assert_eq!(state.zoom, 1.5);
        assert_eq!(state.min_zoom, 0.25);
        assert_eq!(state.max_zoom, 4.0);
        assert_eq!(state.velocity, Vec2::new(50.0, -50.0));
        assert_eq!(state.move_speed, 300.0);
        assert_eq!(state.zoom_speed, 0.2);
        assert_eq!(state.friction, 0.85);
    }

    #[test]
    fn test_apply_zoom_edge_cases() {
        let mut state = CameraState::default();

        // Test zero delta
        let original_zoom = state.zoom;
        state.apply_zoom(0.0);
        assert_eq!(state.zoom, original_zoom);

        // Test very small delta
        state.apply_zoom(0.0001);
        assert!((state.zoom - 1.0001).abs() < f32::EPSILON);

        // Test negative delta from max zoom
        state.zoom = state.max_zoom;
        state.apply_zoom(-0.1);
        assert_eq!(state.zoom, 1.9);
    }

    #[test]
    fn test_velocity_friction_over_multiple_frames() {
        let mut state = CameraState::default();
        state.velocity = Vec2::new(1000.0, 0.0);

        // Simulate multiple frames
        for _ in 0..60 {
            state.update_velocity(1.0 / 60.0);
        }

        // After 1 second, velocity should be significantly reduced
        assert!(state.velocity.x < 100.0);
    }
}
