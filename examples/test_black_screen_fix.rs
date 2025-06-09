//! Test that the black screen issue is fixed when transitioning from MainMenu to Playing state

use bevy::prelude::*;
use claudetest3::{game::GameState, ui, logging};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Black Screen Fix Test".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_plugins((
            game::GameStatePlugin,
            logging::LoggingPlugin,
            ui::world::WorldPlugin,
            ui::panels::UIPanelsPlugin,
        ))
        .add_systems(Update, show_instructions)
        .run();
}

#[derive(Component)]
struct InstructionText;

fn show_instructions(
    mut commands: Commands,
    query: Query<Entity, With<InstructionText>>,
    state: Res<State<GameState>>,
) {
    // Remove old instruction text
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
    
    // Add instruction text based on current state
    let instruction = match state.get() {
        GameState::MainMenu => "Press START GAME button to test transition",
        GameState::Playing => "Game is now playing! Press ESC to pause. The map should be visible.",
        GameState::Paused => "Game paused. Press ESC to resume.",
        _ => "",
    };
    
    if !instruction.is_empty() {
        commands.spawn((
            Text::new(instruction),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(1.0, 1.0, 0.0)),
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            InstructionText,
        ));
    }
}