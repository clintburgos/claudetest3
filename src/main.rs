use bevy::prelude::*;
use claudetest3::{game, logging, testing, ui};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy Isometric World".to_string(),
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
    ));

    // Add testing plugin in debug builds
    #[cfg(debug_assertions)]
    app.add_plugins(testing::TestingPlugin);

    app.run();
}
