//! Camera Module - Isometric camera controls and view management
//!
//! This module provides camera functionality for navigating the isometric world.
//! It handles input, movement, zoom, and boundary constraints.
//!
//! # Features
//! - Keyboard controls (WASD/Arrows for pan, Q/E for zoom)
//! - Mouse/trackpad support (scroll, pan, pinch)
//! - Smooth camera movement with momentum
//! - Boundary constraints to keep map visible
//! - Zoom limits for playability

use bevy::prelude::*;

pub mod components;
pub mod constraints;
pub mod controls;

pub use components::{CameraState, IsometricCamera};

/// Plugin that manages the isometric camera
pub struct IsometricCameraPlugin;

impl Plugin for IsometricCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera).add_systems(
            Update,
            (
                controls::keyboard_camera_system,
                controls::mouse_camera_system,
                controls::zoom_system,
                constraints::apply_camera_constraints_system,
            ),
        );
    }
}

/// Setup the isometric camera
fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        IsometricCamera,
        CameraState::default(),
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
}
