use bevy::prelude::*;
use claudetest3::{game, logging, testing, ui};
use std::time::Duration;

fn main() {
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Test Zoom Fix".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        game::GameStatePlugin,
        logging::LoggingPlugin,
        ui::world::WorldPlugin,
        ui::panels::UIPanelsPlugin,
    ))
    .add_systems(Startup, auto_start_game)
    .add_systems(Update, auto_zoom_out);
    
    app.run();
}

#[derive(Resource)]
struct AutoZoomTimer {
    timer: Timer,
    zoom_phase: u32,
}

fn auto_start_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<game::GameState>>,
) {
    // Start the game immediately
    next_state.set(game::GameState::Playing);
    
    // Setup zoom timer
    commands.insert_resource(AutoZoomTimer {
        timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
        zoom_phase: 0,
    });
}

fn auto_zoom_out(
    mut timer: ResMut<AutoZoomTimer>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut key_events: EventWriter<bevy::input::keyboard::KeyboardInput>,
    current_state: Res<State<game::GameState>>,
) {
    if current_state.get() != &game::GameState::Playing {
        return;
    }
    
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        timer.zoom_phase += 1;
        
        // Simulate pressing E key to zoom out
        if timer.zoom_phase <= 5 {
            info!("Auto-zooming out - phase {}", timer.zoom_phase);
            
            // Send key press event
            key_events.send(bevy::input::keyboard::KeyboardInput {
                key_code: KeyCode::KeyE,
                logical_key: bevy::input::keyboard::Key::Character("e".into()),
                state: bevy::input::ButtonState::Pressed,
                window: Entity::PLACEHOLDER,
            });
        } else if timer.zoom_phase == 10 {
            info!("Test complete - check screenshots to verify zoom behavior");
        }
    }
}