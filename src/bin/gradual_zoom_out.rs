use bevy::prelude::*;
use claudetest3::{game, ui};

#[derive(Resource)]
struct ZoomOutScript {
    current_zoom: f32,
    target_zoom: f32,
    zoom_step: f32,
    timer: Timer,
    screenshot_timer: Timer,
}

fn main() {
    println!("Starting gradual zoom out test...");
    
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Gradual Zoom Out Test".to_string(),
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
    .insert_resource(ZoomOutScript {
        current_zoom: 1.0,
        target_zoom: 0.084, // Minimum zoom
        zoom_step: 0.95, // Multiply by this each step (5% reduction)
        timer: Timer::from_seconds(0.5, TimerMode::Repeating),
        screenshot_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
    })
    // Skip menu and go directly to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    .add_systems(Update, gradual_zoom_out.run_if(in_state(game::GameState::Playing)));
    
    app.run();
}

fn gradual_zoom_out(
    mut script: ResMut<ZoomOutScript>,
    time: Res<Time>,
    mut camera_query: Query<(&Transform, &mut ui::world::camera::CameraState), With<ui::world::camera::IsometricCamera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut screenshot_events: EventWriter<bevy::window::RequestRedraw>,
    mut app_exit: EventWriter<AppExit>,
) {
    script.timer.tick(time.delta());
    script.screenshot_timer.tick(time.delta());
    
    if !script.timer.finished() {
        return;
    }
    
    let Ok((transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };
    
    // Check if we've reached minimum zoom
    if script.current_zoom <= script.target_zoom {
        println!("Reached minimum zoom: {:.3}", script.current_zoom);
        println!("Final camera position: ({:.0}, {:.0})", transform.translation.x, transform.translation.y);
        println!("Test complete! Exiting...");
        app_exit.write(AppExit::Success);
        std::process::exit(0);
    }
    
    // Apply zoom step
    script.current_zoom *= script.zoom_step;
    script.current_zoom = script.current_zoom.max(script.target_zoom);
    camera_state.zoom = script.current_zoom;
    
    println!(
        "Zoom level: {:.3} | Camera pos: ({:.0}, {:.0}) | Min zoom: {:.3}",
        script.current_zoom,
        transform.translation.x,
        transform.translation.y,
        camera_state.min_zoom
    );
    
    // Request screenshot
    if script.screenshot_timer.finished() {
        screenshot_events.send(bevy::window::RequestRedraw);
    }
    
    // Allow manual exit with Escape
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("Manual exit requested");
        app_exit.write(AppExit::Success);
        std::process::exit(0);
    }
}