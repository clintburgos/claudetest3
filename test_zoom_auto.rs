use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use bevy::input::ButtonState;

fn main() {
    println!("Starting zoom test...");
    
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Zoom Test".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        claudetest3::game::GameStatePlugin,
        claudetest3::logging::LoggingPlugin,
        claudetest3::ui::world::WorldPlugin,
        claudetest3::ui::panels::UIPanelsPlugin,
    ))
    .add_systems(Update, auto_test_sequence);

    app.run();
}

#[derive(Default)]
struct TestState {
    timer: f32,
    phase: TestPhase,
}

#[derive(Default, Debug, Clone, Copy, PartialEq)]
enum TestPhase {
    #[default]
    WaitingForMenu,
    ClickingStart,
    WaitingForGame,
    TestingZoom,
    Done,
}

fn auto_test_sequence(
    mut commands: Commands,
    mut test_state: Local<TestState>,
    time: Res<Time>,
    game_state: Res<State<claudetest3::game::GameState>>,
    mut next_state: ResMut<NextState<claudetest3::game::GameState>>,
    button_query: Query<Entity, With<Button>>,
    mut key_events: EventWriter<KeyboardInput>,
) {
    test_state.timer += time.delta_secs();
    
    let seconds = test_state.timer as i32;
    let last_seconds = (test_state.timer - time.delta_secs()) as i32;
    let new_second = seconds != last_seconds;
    
    match test_state.phase {
        TestPhase::WaitingForMenu => {
            if new_second {
                println!("[{:.1}s] Waiting for menu...", test_state.timer);
            }
            if *game_state.get() == claudetest3::game::GameState::MainMenu && test_state.timer > 1.0 {
                test_state.phase = TestPhase::ClickingStart;
                println!("[{:.1}s] Menu loaded, clicking start button", test_state.timer);
            }
        }
        
        TestPhase::ClickingStart => {
            // Directly transition to playing state
            next_state.set(claudetest3::game::GameState::Playing);
            test_state.phase = TestPhase::WaitingForGame;
            println!("[{:.1}s] Transitioning to game", test_state.timer);
        }
        
        TestPhase::WaitingForGame => {
            if *game_state.get() == claudetest3::game::GameState::Playing {
                test_state.phase = TestPhase::TestingZoom;
                test_state.timer = 0.0; // Reset timer for zoom test
                println!("Game started! Beginning zoom test sequence...");
            }
        }
        
        TestPhase::TestingZoom => {
            // Test zoom out with E key
            if test_state.timer < 5.0 {
                // Send E key press event continuously
                key_events.write(KeyboardInput {
                    key_code: KeyCode::KeyE,
                    logical_key: bevy::input::keyboard::Key::Character("e".into()),
                    state: ButtonState::Pressed,
                    window: Entity::PLACEHOLDER,
                    repeat: false,
                    text: None,
                });
                
                if new_second {
                    println!("[{:.1}s] Zooming out (pressing E)...", test_state.timer);
                }
            } else if test_state.timer < 7.0 {
                // Stop zooming
                key_events.write(KeyboardInput {
                    key_code: KeyCode::KeyE,
                    logical_key: bevy::input::keyboard::Key::Character("e".into()),
                    state: ButtonState::Released,
                    window: Entity::PLACEHOLDER,
                    repeat: false,
                    text: None,
                });
                
                if new_second {
                    println!("[{:.1}s] Stopped zooming, observing...", test_state.timer);
                }
            } else if test_state.timer < 12.0 {
                // Test zoom in with Q key
                key_events.write(KeyboardInput {
                    key_code: KeyCode::KeyQ,
                    logical_key: bevy::input::keyboard::Key::Character("q".into()),
                    state: ButtonState::Pressed,
                    window: Entity::PLACEHOLDER,
                    repeat: false,
                    text: None,
                });
                
                if new_second {
                    println!("[{:.1}s] Zooming in (pressing Q)...", test_state.timer);
                }
            } else {
                // Stop zooming
                key_events.write(KeyboardInput {
                    key_code: KeyCode::KeyQ,
                    logical_key: bevy::input::keyboard::Key::Character("q".into()),
                    state: ButtonState::Released,
                    window: Entity::PLACEHOLDER,
                    repeat: false,
                    text: None,
                });
                
                if test_state.phase != TestPhase::Done {
                    test_state.phase = TestPhase::Done;
                    println!("[{:.1}s] Test complete!", test_state.timer);
                    println!("Check the logs directory for screenshots showing the zoom behavior");
                }
            }
        }
        
        TestPhase::Done => {
            // Test complete
        }
    }
}