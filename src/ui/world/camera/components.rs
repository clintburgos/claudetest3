//! Camera Components - Data structures for camera state
//!
//! This file defines components for the isometric camera system.
//! The camera uses orthographic projection suitable for 2D isometric views.
//!
//! # Design Notes
//! - CameraState tracks zoom and movement velocity
//! - Zoom is stored as a scale factor (1.0 = default)
//! - Velocity enables smooth camera movement

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
            zoom: 1.0,
            min_zoom: 0.5,
            max_zoom: 2.0,
            velocity: Vec2::ZERO,
            move_speed: 500.0,
            zoom_speed: 0.1,
            friction: 0.9,
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
        self.velocity *= self.friction.powf(delta_time * 60.0);

        // Stop if velocity is very small
        if self.velocity.length_squared() < 0.01 {
            self.velocity = Vec2::ZERO;
        }
    }
}
