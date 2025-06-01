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

use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseScrollUnit};
use super::components::{IsometricCamera, CameraState};

/// Handle keyboard input for camera movement
pub fn keyboard_camera_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut CameraState), With<IsometricCamera>>,
) {
    let Ok((mut transform, mut state)) = camera_query.single_mut() else { return };
    
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
    let Ok(mut state) = camera_query.single_mut() else { return };
    
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
    let Ok((mut transform, mut state)) = camera_query.single_mut() else { return };
    
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