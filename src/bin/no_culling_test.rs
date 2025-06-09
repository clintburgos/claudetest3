use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    println!("Starting game with culling DISABLED...");
    
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "No Culling Test - All Tiles Rendered".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }).set(bevy::log::LogPlugin {
        filter: "warn,claudetest3::ui::world::tiles::view_culling=info".to_string(),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
    .add_plugins((
        game::GameStatePlugin,
        ui::world::WorldPlugin,
        ui::panels::UIPanelsPlugin,
    ))
    // Disable culling by inserting the config before the world plugin runs
    .insert_resource(ui::world::tiles::ViewCullingConfig {
        enabled: false,
        tiles_per_frame: 40000, // Spawn all tiles in one frame
        buffer_tiles: 0,
    })
    // Skip menu and go directly to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    // Add toggle system
    .add_systems(Update, toggle_culling.run_if(in_state(game::GameState::Playing)));
    
    app.run();
}

fn toggle_culling(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut culling_config: ResMut<ui::world::tiles::ViewCullingConfig>,
) {
    // Toggle with C key
    if keyboard.just_pressed(KeyCode::KeyC) {
        culling_config.enabled = !culling_config.enabled;
        println!(
            "Culling {} (press C to toggle)",
            if culling_config.enabled { "ENABLED" } else { "DISABLED" }
        );
    }
    
    // Show current tile count with D key
    if keyboard.just_pressed(KeyCode::KeyD) {
        println!(
            "Culling config: enabled={}, buffer_tiles={}, tiles_per_frame={}",
            culling_config.enabled, culling_config.buffer_tiles, culling_config.tiles_per_frame
        );
    }
    
    // Exit with Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        std::process::exit(0);
    }
}