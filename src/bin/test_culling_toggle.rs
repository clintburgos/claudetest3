//! Test program that automatically toggles culling after a delay

use bevy::prelude::*;
use claudetest3::{game, logging, ui};
use claudetest3::ui::world::tiles::ViewCullingConfig;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Culling Toggle Test".to_string(),
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
        // Skip menu and go directly to playing
        .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
            next_state.set(game::GameState::Playing);
        })
        .add_systems(Startup, setup_test)
        .add_systems(Update, auto_toggle_culling.run_if(in_state(game::GameState::Playing)))
        .run();
}

fn setup_test(mut commands: Commands) {
    info!("Starting culling toggle test - culling will be toggled after 3 seconds");
    commands.insert_resource(ToggleTimer {
        timer: Timer::from_seconds(3.0, TimerMode::Once),
        toggled: false,
    });
}

#[derive(Resource)]
struct ToggleTimer {
    timer: Timer,
    toggled: bool,
}

fn auto_toggle_culling(
    mut timer: ResMut<ToggleTimer>,
    time: Res<Time>,
    mut culling_config: ResMut<ViewCullingConfig>,
) {
    if timer.toggled {
        return;
    }

    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        culling_config.enabled = !culling_config.enabled;
        timer.toggled = true;
        
        if culling_config.enabled {
            info!("AUTO: View culling ENABLED");
        } else {
            info!("AUTO: View culling DISABLED - all tiles should spawn now!");
        }
    }
}