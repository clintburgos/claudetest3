//! Game State - State management for game flow
//!
//! This module defines the game states and handles transitions between them.
//! States include MainMenu, Playing, Paused, and GameOver.

use bevy::prelude::*;

/// The current state of the game
#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    /// Main menu is displayed
    #[default]
    MainMenu,
    /// Game is actively playing
    Playing,
    /// Game is paused
    Paused,
    /// Game over screen
    GameOver,
}

/// Plugin that manages game state transitions
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(OnEnter(GameState::MainMenu), enter_main_menu)
            .add_systems(OnExit(GameState::MainMenu), exit_main_menu)
            .add_systems(OnEnter(GameState::Playing), enter_playing)
            .add_systems(OnEnter(GameState::Paused), enter_paused)
            .add_systems(OnExit(GameState::Paused), exit_paused)
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                handle_resume_input.run_if(in_state(GameState::Paused)),
            )
            .add_systems(
                Update,
                handle_menu_buttons.run_if(in_state(GameState::MainMenu)),
            );
    }
}

/// Marker component for main menu UI
#[derive(Component)]
struct MainMenuUI;

/// Marker component for pause menu UI
#[derive(Component)]
struct PauseMenuUI;

/// Called when entering the main menu state
fn enter_main_menu(mut commands: Commands) {
    info!("Entering main menu");

    // Spawn main menu UI
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("Isometric World"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Play button
            parent
                .spawn((
                    Button,
                    Node {
                        padding: UiRect::new(
                            Val::Px(40.0),
                            Val::Px(40.0),
                            Val::Px(20.0),
                            Val::Px(20.0),
                        ),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                    StartGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Start Game"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

/// Called when exiting the main menu state
fn exit_main_menu(mut commands: Commands, menu_query: Query<Entity, With<MainMenuUI>>) {
    info!("Exiting main menu");

    // Remove main menu UI
    for entity in menu_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Called when entering the playing state
fn enter_playing() {
    info!("Game started - entering playing state");
}

/// Called when entering the paused state
fn enter_paused(mut commands: Commands) {
    info!("Game paused");

    // Spawn pause overlay
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            PauseMenuUI,
        ))
        .with_children(|parent| {
            // Paused text
            parent.spawn((
                Text::new("PAUSED"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Instructions
            parent.spawn((
                Text::new("Press ESC to resume"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgba(0.8, 0.8, 0.8, 1.0)),
            ));
        });
}

/// Called when exiting the paused state
fn exit_paused(mut commands: Commands, pause_query: Query<Entity, With<PauseMenuUI>>) {
    info!("Resuming game");

    // Remove pause overlay
    for entity in pause_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Handle pause input during gameplay
fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Paused);
    }
}

/// Handle resume input during pause
fn handle_resume_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Playing);
    }
}

/// Marker component for the start game button
#[derive(Component)]
pub struct StartGameButton;

/// Type alias for button interaction query
type ButtonInteractionQuery<'w, 's> = Query<
    'w,
    's,
    (&'static Interaction, &'static mut BackgroundColor),
    (Changed<Interaction>, With<StartGameButton>),
>;

/// System to handle main menu button interactions
fn handle_menu_buttons(
    mut interaction_query: ButtonInteractionQuery,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut bg_color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = Color::srgb(0.4, 0.6, 0.4).into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *bg_color = Color::srgb(0.35, 0.55, 0.35).into();
            }
            Interaction::None => {
                *bg_color = Color::srgb(0.3, 0.5, 0.3).into();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::input::InputPlugin;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_game_state_default() {
        assert_eq!(GameState::default(), GameState::MainMenu);
    }

    #[test]
    fn test_enter_main_menu_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.world_mut()
            .run_system_once(enter_main_menu)
            .expect("System should run");

        // Check that main menu UI was created
        let menu_count = app
            .world_mut()
            .query::<&MainMenuUI>()
            .iter(&app.world())
            .count();
        assert_eq!(menu_count, 1, "Should create main menu UI");

        // Check that start button exists
        let button_count = app
            .world_mut()
            .query::<&StartGameButton>()
            .iter(&app.world())
            .count();
        assert_eq!(button_count, 1, "Should create start game button");
    }

    #[test]
    fn test_exit_main_menu_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create main menu UI
        let menu_entity = app.world_mut().spawn(MainMenuUI).id();

        app.world_mut()
            .run_system_once(exit_main_menu)
            .expect("System should run");

        // Check that main menu UI was removed
        assert!(!app.world().entities().contains(menu_entity));
    }

    #[test]
    fn test_enter_playing_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Run system - should just log, not panic
        app.world_mut()
            .run_system_once(enter_playing)
            .expect("System should run");
    }

    #[test]
    fn test_enter_paused_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.world_mut()
            .run_system_once(enter_paused)
            .expect("System should run");

        // Check that pause menu UI was created
        let pause_count = app
            .world_mut()
            .query::<&PauseMenuUI>()
            .iter(&app.world())
            .count();
        assert_eq!(pause_count, 1, "Should create pause menu UI");
    }

    #[test]
    fn test_exit_paused_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create pause menu UI
        let pause_entity = app.world_mut().spawn(PauseMenuUI).id();

        app.world_mut()
            .run_system_once(exit_paused)
            .expect("System should run");

        // Check that pause menu UI was removed
        assert!(!app.world().entities().contains(pause_entity));
    }

    #[test]
    fn test_handle_pause_input_system() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, StatesPlugin));
        app.init_state::<GameState>();
        app.init_resource::<ButtonInput<KeyCode>>();

        // Simulate ESC key press
        let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keyboard.press(KeyCode::Escape);

        app.world_mut()
            .run_system_once(handle_pause_input)
            .expect("System should run");

        // Check that next state was set to Paused
        let next_state = app.world().resource::<NextState<GameState>>();
        match next_state {
            NextState::Pending(state) => assert_eq!(*state, GameState::Paused),
            _ => panic!("Expected pending state transition to Paused"),
        }
    }

    #[test]
    fn test_handle_resume_input_system() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, StatesPlugin));
        app.init_state::<GameState>();
        app.init_resource::<ButtonInput<KeyCode>>();

        // Simulate ESC key press
        let mut keyboard = app.world_mut().resource_mut::<ButtonInput<KeyCode>>();
        keyboard.press(KeyCode::Escape);

        app.world_mut()
            .run_system_once(handle_resume_input)
            .expect("System should run");

        // Check that next state was set to Playing
        let next_state = app.world().resource::<NextState<GameState>>();
        match next_state {
            NextState::Pending(state) => assert_eq!(*state, GameState::Playing),
            _ => panic!("Expected pending state transition to Playing"),
        }
    }

    #[test]
    fn test_handle_menu_buttons_start_game() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();

        // Create a start button with pressed interaction
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(Color::WHITE),
            StartGameButton,
        ));

        app.world_mut()
            .run_system_once(handle_menu_buttons)
            .expect("System should run");

        // Check that next state was set to Playing
        let next_state = app.world().resource::<NextState<GameState>>();
        match next_state {
            NextState::Pending(state) => assert_eq!(*state, GameState::Playing),
            _ => panic!("Expected pending state transition to Playing"),
        }
    }

    #[test]
    fn test_handle_menu_buttons_hover() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();

        // Create a start button with hover interaction
        let button = app
            .world_mut()
            .spawn((
                Interaction::Hovered,
                BackgroundColor(Color::srgb(0.3, 0.5, 0.3)),
                StartGameButton,
            ))
            .id();

        app.world_mut()
            .run_system_once(handle_menu_buttons)
            .expect("System should run");

        // Check that background color changed
        let bg_color = app.world().get::<BackgroundColor>(button).unwrap();
        assert_eq!(bg_color.0, Color::srgb(0.35, 0.55, 0.35));
    }

    #[test]
    fn test_handle_menu_buttons_none() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();

        // Create a start button with no interaction
        let button = app
            .world_mut()
            .spawn((
                Interaction::None,
                BackgroundColor(Color::srgb(0.4, 0.6, 0.4)),
                StartGameButton,
            ))
            .id();

        app.world_mut()
            .run_system_once(handle_menu_buttons)
            .expect("System should run");

        // Check that background color changed back to normal
        let bg_color = app.world().get::<BackgroundColor>(button).unwrap();
        assert_eq!(bg_color.0, Color::srgb(0.3, 0.5, 0.3));
    }

    #[test]
    fn test_game_state_plugin() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, InputPlugin, StatesPlugin));
        app.add_plugins(GameStatePlugin);

        // Should initialize state and add all systems without panicking
        app.update();

        // Check that state was initialized
        let state = app.world().resource::<State<GameState>>();
        assert_eq!(*state.get(), GameState::MainMenu);
    }
}
