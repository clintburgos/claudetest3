use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Tile Test - No Logging".to_string(),
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
    ));

    // Start directly in Playing state
    app.insert_resource(NextState::Pending(game::GameState::Playing));

    app.run();
}