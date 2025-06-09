use bevy::prelude::*;
use bevy::window::WindowCloseRequested;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (print_status, handle_escape))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera
    commands.spawn(Camera2d);
    
    // Spawn a simple colored sprite (no mesh, just a basic sprite)
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    // Spawn some UI text to verify rendering is working
    commands.spawn((
        Text::new("Red square should be visible in center"),
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
    
    println!("Setup complete: Camera, red sprite, and text spawned");
}

fn print_status(mut count: Local<u32>) {
    *count += 1;
    if *count % 60 == 0 {
        println!("Frame {}: Application is running", *count);
    }
}

fn handle_escape(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        println!("ESC pressed, exiting...");
        exit.send(AppExit::Success);
    }
}