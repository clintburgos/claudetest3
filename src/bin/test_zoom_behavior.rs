use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Zoom Behavior Test".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
    .add_plugins((
        game::GameStatePlugin,
        ui::world::WorldPlugin,
        ui::panels::UIPanelsPlugin,
    ))
    // Skip menu and go directly to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    .add_systems(Update, automated_zoom_test.run_if(in_state(game::GameState::Playing)));
    
    app.run();
}

#[derive(Resource)]
struct ZoomTestState {
    timer: Timer,
    phase: usize,
    zoom_steps: Vec<f32>,
}

impl Default for ZoomTestState {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            phase: 0,
            // Test zoom levels from 1.0 down to minimum
            zoom_steps: vec![1.0, 0.8, 0.6, 0.4, 0.2, 0.1, 0.08],
        }
    }
}

fn automated_zoom_test(
    mut commands: Commands,
    time: Res<Time>,
    mut test_state: Local<ZoomTestState>,
    mut camera_query: Query<&mut ui::world::camera::CameraState, With<ui::world::camera::IsometricCamera>>,
) {
    test_state.timer.tick(time.delta());
    
    if test_state.timer.just_finished() {
        if let Ok(mut camera_state) = camera_query.single_mut() {
            if test_state.phase < test_state.zoom_steps.len() {
                let target_zoom = test_state.zoom_steps[test_state.phase];
                println!("Setting zoom to: {:.2} (min: {:.4}, max: {:.4})", 
                    target_zoom, camera_state.min_zoom, camera_state.max_zoom);
                
                camera_state.zoom = target_zoom.clamp(camera_state.min_zoom, camera_state.max_zoom);
                println!("Actual zoom after clamp: {:.4}", camera_state.zoom);
                
                test_state.phase += 1;
            } else {
                println!("Zoom test complete!");
                std::process::exit(0);
            }
        }
    }
}