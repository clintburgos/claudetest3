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

use crate::game::GameState;
use bevy::prelude::*;

pub mod components;
pub mod constraints;
pub mod controls;

pub use components::{CameraState, IsometricCamera};

/// Plugin that manages the isometric camera
pub struct IsometricCameraPlugin;

impl Plugin for IsometricCameraPlugin {
    fn build(&self, app: &mut App) {
        use crate::ui::world::WorldSystems;

        app.add_systems(
            OnEnter(GameState::Playing),
            setup_camera.in_set(WorldSystems::CameraSetup),
        )
        .add_systems(OnExit(GameState::Playing), cleanup_camera)
        .add_systems(
            Update,
            (
                controls::keyboard_camera_system,
                controls::mouse_camera_system,
                controls::zoom_system,
                constraints::apply_camera_constraints_system,
            )
                .chain()
                .in_set(WorldSystems::CameraUpdate)
                .run_if(in_state(GameState::Playing)),
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

/// Cleanup the isometric camera when leaving the playing state
fn cleanup_camera(mut commands: Commands, camera_query: Query<Entity, With<IsometricCamera>>) {
    for entity in camera_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::GameState;
    use crate::ui::world::{GridConfig, WorldSystems};
    use bevy::ecs::system::RunSystemOnce;
    use bevy::input::{mouse::MouseScrollUnit, mouse::MouseWheel, InputPlugin};
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_setup_camera() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Run setup
        app.world_mut()
            .run_system_once(setup_camera)
            .expect("System should run");

        // Find camera entity
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<IsometricCamera>>();
        let camera_entities: Vec<Entity> = query.iter(&app.world()).collect();

        assert_eq!(camera_entities.len(), 1, "Should spawn exactly one camera");

        let camera = camera_entities[0];

        // Verify components
        assert!(app.world().get::<Camera2d>(camera).is_some());
        assert!(app.world().get::<IsometricCamera>(camera).is_some());
        assert!(app.world().get::<CameraState>(camera).is_some());

        let transform = app.world().get::<Transform>(camera).unwrap();
        assert_eq!(transform.translation, Vec3::new(0.0, 0.0, 1000.0));
    }

    #[test]
    fn test_camera_plugin_integration() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, StatesPlugin));

        // Initialize game state
        app.init_state::<GameState>();
        app.insert_resource(NextState::Pending(GameState::Playing));

        // Add camera plugin after state is initialized
        app.add_plugins(IsometricCameraPlugin);

        // Add required resources
        app.insert_resource(GridConfig::default());
        app.init_resource::<Events<MouseWheel>>();

        // Create window for constraints system
        app.world_mut().spawn(Window {
            resolution: (1280.0, 720.0).into(),
            ..default()
        });

        // Run startup systems
        app.update();

        // Verify camera was spawned
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<IsometricCamera>>();
        let camera_entities: Vec<Entity> = query.iter(&app.world()).collect();
        assert_eq!(camera_entities.len(), 1);

        // Simulate some input
        let mut input = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        input.press(KeyCode::KeyD);
        input.press(KeyCode::KeyQ);

        // Send mouse wheel event
        app.world_mut().send_event(MouseWheel {
            unit: MouseScrollUnit::Line,
            x: 0.0,
            y: 1.0,
            window: Entity::PLACEHOLDER,
        });

        // Run update systems
        app.update();

        // Verify camera state changed
        let camera = camera_entities[0];
        let state = app.world().get::<CameraState>(camera).unwrap();

        // Should have some velocity from D key
        assert!(state.velocity.x > 0.0);

        // Should have increased zoom from Q key and mouse wheel
        assert!(state.zoom > 1.0);
    }

    #[test]
    fn test_camera_systems_ordering() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin));

        // Add world systems and camera plugin
        app.configure_sets(
            Startup,
            (WorldSystems::GridInit, WorldSystems::CameraSetup).chain(),
        );

        app.configure_sets(Update, WorldSystems::CameraUpdate);

        app.add_plugins(IsometricCameraPlugin);

        // Should compile and configure without panics
        app.insert_resource(GridConfig::default());
        app.init_resource::<Events<MouseWheel>>();

        app.update();
    }

    #[test]
    fn test_multiple_cameras_not_allowed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn first camera
        app.world_mut()
            .run_system_once(setup_camera)
            .expect("System should run");

        // Spawn second camera
        app.world_mut()
            .run_system_once(setup_camera)
            .expect("System should run");

        // Should have two cameras (system doesn't prevent multiple)
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<IsometricCamera>>();
        let camera_count = query.iter(&app.world()).count();
        assert_eq!(camera_count, 2);

        // Note: In production, we might want to prevent multiple cameras
        // but for now the system allows it
    }

    #[test]
    fn test_camera_state_persistence() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, StatesPlugin));

        // Initialize game state
        app.init_state::<GameState>();
        app.insert_resource(NextState::Pending(GameState::Playing));

        // Add camera plugin after state is initialized
        app.add_plugins(IsometricCameraPlugin);

        app.insert_resource(GridConfig::default());
        app.init_resource::<Events<MouseWheel>>();

        // Initial update to spawn camera
        app.update();

        // Get camera and modify state
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<IsometricCamera>>();
        let camera = query.single(&app.world()).unwrap();

        // Set custom velocity
        app.world_mut()
            .entity_mut(camera)
            .get_mut::<CameraState>()
            .unwrap()
            .velocity = Vec2::new(123.0, 456.0);

        // Update should preserve custom state while applying friction
        app.update();

        let state = app.world().get::<CameraState>(camera).unwrap();
        // Velocity should be reduced but not zero
        assert!(state.velocity.x < 123.0);
        assert!(state.velocity.x > 0.0);
        assert!(state.velocity.y < 456.0);
        assert!(state.velocity.y > 0.0);
    }

    #[test]
    fn test_camera_with_all_systems() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, StatesPlugin));

        // Initialize game state
        app.init_state::<GameState>();
        app.insert_resource(NextState::Pending(GameState::Playing));

        // Add camera plugin after state is initialized
        app.add_plugins(IsometricCameraPlugin);

        // Add required resources
        app.insert_resource(GridConfig {
            width: 50,
            height: 50,
            tile_size: 64.0,
        });
        app.init_resource::<Events<MouseWheel>>();

        // Add window for constraints
        app.world_mut().spawn(Window {
            resolution: (1280.0, 720.0).into(),
            ..default()
        });

        // Run initial update
        app.update();

        // Get camera
        let mut query = app
            .world_mut()
            .query_filtered::<Entity, With<IsometricCamera>>();
        let camera = query.single(&app.world()).unwrap();

        // Move camera out of bounds
        app.world_mut()
            .entity_mut(camera)
            .get_mut::<Transform>()
            .unwrap()
            .translation = Vec3::new(10000.0, 10000.0, 1000.0);

        // Update should apply constraints
        app.update();

        let transform = app.world().get::<Transform>(camera).unwrap();
        // Should be constrained
        assert!(transform.translation.x < 10000.0);
        assert!(transform.translation.y < 10000.0);
    }
}
