//! Top Bar - Game controls and status display
//!
//! This module creates a top bar with game control buttons
//! and status information.

use super::components::*;
use crate::game::GameState;
use crate::ui::styles::*;
use bevy::prelude::*;

/// Plugin for the top bar UI
pub struct TopBarPlugin;

impl Plugin for TopBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_top_bar)
            .add_systems(OnExit(GameState::Playing), despawn_top_bar)
            .add_systems(
                Update,
                handle_button_interactions.run_if(in_state(GameState::Playing)),
            );
    }
}

/// Spawns the top bar UI
fn spawn_top_bar(mut commands: Commands) {
    // Top bar container
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                height: Val::Px(60.0),
                padding: UiRect::horizontal(Val::Px(20.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.95)),
            TopBar,
        ))
        .with_children(|parent| {
            // Left section - Title
            parent.spawn((
                Text::new("Isometric World"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Right section - Control buttons
            parent
                .spawn((Node {
                    flex_direction: FlexDirection::Row,
                    column_gap: Val::Px(10.0),
                    ..default()
                },))
                .with_children(|parent| {
                    // Pause button
                    parent
                        .spawn((
                            Button,
                            Node {
                                padding: UiRect::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                ..default()
                            },
                            BackgroundColor(BUTTON_COLOR),
                            GameControlButton,
                            ButtonAction::Pause,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Pause"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Menu button
                    parent
                        .spawn((
                            Button,
                            Node {
                                padding: UiRect::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                ..default()
                            },
                            BackgroundColor(BUTTON_COLOR),
                            GameControlButton,
                            ButtonAction::MainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Menu"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });

                    // Quit button
                    parent
                        .spawn((
                            Button,
                            Node {
                                padding: UiRect::new(
                                    Val::Px(20.0),
                                    Val::Px(20.0),
                                    Val::Px(10.0),
                                    Val::Px(10.0),
                                ),
                                ..default()
                            },
                            BackgroundColor(BUTTON_COLOR),
                            GameControlButton,
                            ButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Quit"),
                                TextFont {
                                    font_size: 18.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

/// Type alias for control button interaction query
type ControlButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static ButtonAction,
    ),
    (Changed<Interaction>, With<GameControlButton>),
>;

/// Handles button interactions
fn handle_button_interactions(
    mut interaction_query: ControlButtonQuery,
    mut exit: EventWriter<AppExit>,
    mut next_state: ResMut<NextState<GameState>>,
    current_state: Res<State<GameState>>,
) {
    for (interaction, mut bg_color, action) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                *bg_color = BUTTON_PRESSED_COLOR.into();

                // Handle button actions
                match action {
                    ButtonAction::Pause => {
                        info!("Pause button pressed");
                        if *current_state.get() == GameState::Playing {
                            next_state.set(GameState::Paused);
                        }
                    }
                    ButtonAction::Resume => {
                        info!("Resume button pressed");
                        if *current_state.get() == GameState::Paused {
                            next_state.set(GameState::Playing);
                        }
                    }
                    ButtonAction::MainMenu => {
                        info!("Menu button pressed");
                        next_state.set(GameState::MainMenu);
                    }
                    ButtonAction::Quit => {
                        info!("Quit button pressed");
                        exit.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *bg_color = BUTTON_HOVER_COLOR.into();
            }
            Interaction::None => {
                *bg_color = BUTTON_COLOR.into();
            }
        }
    }
}

/// Despawns the top bar when leaving the playing state
fn despawn_top_bar(mut commands: Commands, bar_query: Query<Entity, With<TopBar>>) {
    for entity in bar_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;
    use bevy::state::app::StatesPlugin;

    #[test]
    fn test_spawn_top_bar() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.world_mut()
            .run_system_once(spawn_top_bar)
            .expect("System should run");

        // Check that top bar was created
        let bar_count = app
            .world_mut()
            .query::<&TopBar>()
            .iter(&app.world())
            .count();
        assert_eq!(bar_count, 1, "Should create top bar");

        // Check that control buttons were created
        let button_count = app
            .world_mut()
            .query::<&GameControlButton>()
            .iter(&app.world())
            .count();
        assert_eq!(button_count, 3, "Should create 3 control buttons");

        // Check button types
        let pause_buttons = app
            .world_mut()
            .query::<(&GameControlButton, &ButtonAction)>()
            .iter(&app.world())
            .filter(|(_, action)| **action == ButtonAction::Pause)
            .count();
        assert_eq!(pause_buttons, 1, "Should have pause button");

        let menu_buttons = app
            .world_mut()
            .query::<(&GameControlButton, &ButtonAction)>()
            .iter(&app.world())
            .filter(|(_, action)| **action == ButtonAction::MainMenu)
            .count();
        assert_eq!(menu_buttons, 1, "Should have menu button");

        let quit_buttons = app
            .world_mut()
            .query::<(&GameControlButton, &ButtonAction)>()
            .iter(&app.world())
            .filter(|(_, action)| **action == ButtonAction::Quit)
            .count();
        assert_eq!(quit_buttons, 1, "Should have quit button");
    }

    #[test]
    fn test_despawn_top_bar() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create top bar
        let bar_entity = app.world_mut().spawn(TopBar).id();

        app.world_mut()
            .run_system_once(despawn_top_bar)
            .expect("System should run");

        // Check that top bar was removed
        assert!(!app.world().entities().contains(bar_entity));
    }

    #[test]
    fn test_handle_button_interactions_pause() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();
        app.insert_resource(State::new(GameState::Playing));
        app.init_resource::<Events<AppExit>>();

        // Create pause button with pressed interaction
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(BUTTON_COLOR),
            GameControlButton,
            ButtonAction::Pause,
        ));

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that next state was set to Paused
        let next_state = app.world().resource::<NextState<GameState>>();
        match next_state {
            NextState::Pending(state) => assert_eq!(*state, GameState::Paused),
            _ => panic!("Expected pending state transition to Paused"),
        }
    }

    #[test]
    fn test_handle_button_interactions_resume() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();
        app.insert_resource(State::new(GameState::Paused));
        app.init_resource::<Events<AppExit>>();

        // Create resume button with pressed interaction
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(BUTTON_COLOR),
            GameControlButton,
            ButtonAction::Resume,
        ));

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that next state was set to Playing
        let next_state = app.world().resource::<NextState<GameState>>();
        match next_state {
            NextState::Pending(state) => assert_eq!(*state, GameState::Playing),
            _ => panic!("Expected pending state transition to Playing"),
        }
    }

    #[test]
    fn test_handle_button_interactions_menu() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();
        app.init_resource::<Events<AppExit>>();

        // Create menu button with pressed interaction
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(BUTTON_COLOR),
            GameControlButton,
            ButtonAction::MainMenu,
        ));

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that next state was set to MainMenu
        let next_state = app.world().resource::<NextState<GameState>>();
        match next_state {
            NextState::Pending(state) => assert_eq!(*state, GameState::MainMenu),
            _ => panic!("Expected pending state transition to MainMenu"),
        }
    }

    #[test]
    fn test_handle_button_interactions_quit() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();
        app.init_resource::<Events<AppExit>>();

        // Create quit button with pressed interaction
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(BUTTON_COLOR),
            GameControlButton,
            ButtonAction::Quit,
        ));

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that app exit event was sent
        let exit_events = app.world().resource::<Events<AppExit>>();
        assert_eq!(exit_events.len(), 1, "Should send app exit event");
    }

    #[test]
    fn test_handle_button_interactions_hover() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();
        app.init_resource::<Events<AppExit>>();

        // Create button with hover interaction
        let button = app
            .world_mut()
            .spawn((
                Interaction::Hovered,
                BackgroundColor(BUTTON_COLOR),
                GameControlButton,
                ButtonAction::Pause,
            ))
            .id();

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that background color changed
        let bg_color = app.world().get::<BackgroundColor>(button).unwrap();
        assert_eq!(bg_color.0, BUTTON_HOVER_COLOR);
    }

    #[test]
    fn test_handle_button_interactions_none() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, StatesPlugin));
        app.init_state::<GameState>();
        app.init_resource::<Events<AppExit>>();

        // Create button with no interaction
        let button = app
            .world_mut()
            .spawn((
                Interaction::None,
                BackgroundColor(BUTTON_HOVER_COLOR),
                GameControlButton,
                ButtonAction::Pause,
            ))
            .id();

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that background color changed back to normal
        let bg_color = app.world().get::<BackgroundColor>(button).unwrap();
        assert_eq!(bg_color.0, BUTTON_COLOR);
    }
}
