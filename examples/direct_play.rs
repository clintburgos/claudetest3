use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Isometric World - Direct Play".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            game::GameStatePlugin,
            ui::world::WorldPlugin,
            ui::panels::UIPanelsPlugin,
        ))
        // Start directly in Playing state
        .add_systems(
            Startup,
            |mut next_state: ResMut<NextState<game::GameState>>| {
                next_state.set(game::GameState::Playing);
            },
        )
        .run();
}
