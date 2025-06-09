use bevy::prelude::*;

fn main() {
    println!("Manual Zoom Test");
    println!("================");
    println!("Controls:");
    println!("  Q - Zoom in");
    println!("  E - Zoom out");
    println!("  WASD/Arrows - Move camera");
    println!("  ESC - Pause");
    println!("  Click 'Start Game' to begin");
    println!("");
    
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Manual Zoom Test".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }))
    .add_plugins((
        claudetest3::game::GameStatePlugin,
        claudetest3::logging::LoggingPlugin,
        claudetest3::ui::world::WorldPlugin,
        claudetest3::ui::panels::UIPanelsPlugin,
    ))
    .add_systems(Update, log_camera_info);

    app.run();
}

fn log_camera_info(
    camera_query: Query<(&Transform, &claudetest3::ui::world::camera::CameraState), With<claudetest3::ui::world::camera::IsometricCamera>>,
    time: Res<Time>,
    mut last_log: Local<f32>,
) {
    *last_log += time.delta_secs();
    
    // Log every 2 seconds
    if *last_log > 2.0 {
        if let Ok((transform, state)) = camera_query.get_single() {
            println!("[Camera] Position: ({:.1}, {:.1}), Zoom: {:.3}, Scale: {:?}", 
                transform.translation.x, 
                transform.translation.y, 
                state.zoom,
                transform.scale
            );
            *last_log = 0.0;
        }
    }
}