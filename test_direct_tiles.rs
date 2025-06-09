use bevy::prelude::*;
use claudetest3::{game::GameState, ui::world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Direct Tile Test".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .init_state::<GameState>()
        // Start directly in Playing state to bypass menu
        .insert_resource(NextState::Pending(GameState::Playing))
        .add_plugins(WorldPlugin)
        .run();
}