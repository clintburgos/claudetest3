use bevy::prelude::*;
use claudetest3::{game, ui};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "UI Blocking Test".to_string(),
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
    ))
    // Skip menu and go straight to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    // Add debug system
    .add_systems(Update, check_blocking_ui.run_if(run_once()));

    app.run();
}

fn check_blocking_ui(
    ui_query: Query<(Entity, &Node, Option<&BackgroundColor>, Option<&Name>), Without<Parent>>,
    camera_query: Query<Entity, With<Camera2d>>,
    tile_query: Query<Entity, With<ui::world::Tile>>,
) {
    println!("\n=== UI BLOCKING CHECK ===");
    
    // Check cameras
    let camera_count = camera_query.iter().count();
    println!("Cameras: {}", camera_count);
    
    // Check tiles
    let tile_count = tile_query.iter().count();
    println!("Tiles spawned: {}", tile_count);
    
    // Check for full-screen UI elements
    println!("\nRoot UI nodes:");
    for (entity, node, bg_color, name) in ui_query.iter() {
        let name_str = name.map(|n| n.as_str()).unwrap_or("unnamed");
        
        // Check if it's full screen
        let is_fullscreen = node.position_type == PositionType::Absolute
            && node.width == Val::Percent(100.0)
            && node.height == Val::Percent(100.0);
            
        if is_fullscreen {
            println!("  FULLSCREEN {:?} [{}]", entity, name_str);
            if let Some(color) = bg_color {
                println!("    Background: {:?}", color.0);
                if color.0.alpha() > 0.0 {
                    println!("    ⚠️  BLOCKING - Has opaque background!");
                }
            }
        } else {
            println!("  {:?} [{}] - size: {:?}x{:?}", entity, name_str, node.width, node.height);
        }
    }
    
    println!("=========================\n");
}

fn run_once() -> impl FnMut() -> bool {
    let mut has_run = false;
    move || {
        if has_run {
            false
        } else {
            has_run = true;
            true
        }
    }
}