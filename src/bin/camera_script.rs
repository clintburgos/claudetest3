use bevy::prelude::*;
use claudetest3::{game, ui};
use std::env;

#[derive(Debug, Clone)]
enum CameraAction {
    Pan(f32, f32),      // dx, dy in world units
    Zoom(f32),          // zoom factor (1.0 = no change, 2.0 = zoom in, 0.5 = zoom out)
    SetZoom(f32),       // absolute zoom level
    Wait(f32),          // wait in seconds
    CenterOnTile(i32, i32), // center on specific tile coordinates
}

#[derive(Resource)]
struct CameraScript {
    actions: Vec<CameraAction>,
    current_action: usize,
    action_timer: Timer,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    let actions = match parse_actions(&args[1..]) {
        Ok(actions) => actions,
        Err(e) => {
            eprintln!("Error parsing actions: {}", e);
            print_usage();
            return;
        }
    };
    
    if actions.is_empty() {
        eprintln!("No valid actions provided");
        print_usage();
        return;
    }
    
    println!("Executing camera script with {} actions:", actions.len());
    for (i, action) in actions.iter().enumerate() {
        println!("  {}: {:?}", i + 1, action);
    }
    
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Camera Script Runner".to_string(),
            resolution: (1280., 720.).into(),
            ..default()
        }),
        ..default()
    }).set(bevy::log::LogPlugin {
        filter: "warn".to_string(),
        ..default()
    }))
    .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
    .add_plugins((
        game::GameStatePlugin,
        ui::world::WorldPlugin,
        ui::panels::UIPanelsPlugin,
    ))
    .insert_resource(CameraScript {
        actions,
        current_action: 0,
        action_timer: Timer::from_seconds(0.1, TimerMode::Once),
    })
    // Skip menu and go directly to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    .add_systems(Update, execute_camera_script.run_if(in_state(game::GameState::Playing)));
    
    app.run();
}

fn parse_actions(args: &[String]) -> Result<Vec<CameraAction>, String> {
    let mut actions = Vec::new();
    let mut i = 0;
    
    while i < args.len() {
        match args[i].as_str() {
            "--pan" | "-p" => {
                if i + 2 >= args.len() {
                    return Err("--pan requires two arguments: dx dy".to_string());
                }
                let dx = args[i + 1].parse::<f32>()
                    .map_err(|_| format!("Invalid pan dx value: {}", args[i + 1]))?;
                let dy = args[i + 2].parse::<f32>()
                    .map_err(|_| format!("Invalid pan dy value: {}", args[i + 2]))?;
                actions.push(CameraAction::Pan(dx, dy));
                i += 3;
            }
            "--zoom" | "-z" => {
                if i + 1 >= args.len() {
                    return Err("--zoom requires one argument: factor".to_string());
                }
                let factor = args[i + 1].parse::<f32>()
                    .map_err(|_| format!("Invalid zoom factor: {}", args[i + 1]))?;
                actions.push(CameraAction::Zoom(factor));
                i += 2;
            }
            "--set-zoom" | "-sz" => {
                if i + 1 >= args.len() {
                    return Err("--set-zoom requires one argument: level".to_string());
                }
                let level = args[i + 1].parse::<f32>()
                    .map_err(|_| format!("Invalid zoom level: {}", args[i + 1]))?;
                actions.push(CameraAction::SetZoom(level));
                i += 2;
            }
            "--wait" | "-w" => {
                if i + 1 >= args.len() {
                    return Err("--wait requires one argument: seconds".to_string());
                }
                let seconds = args[i + 1].parse::<f32>()
                    .map_err(|_| format!("Invalid wait time: {}", args[i + 1]))?;
                actions.push(CameraAction::Wait(seconds));
                i += 2;
            }
            "--center" | "-c" => {
                if i + 2 >= args.len() {
                    return Err("--center requires two arguments: tile_x tile_y".to_string());
                }
                let x = args[i + 1].parse::<i32>()
                    .map_err(|_| format!("Invalid tile x: {}", args[i + 1]))?;
                let y = args[i + 2].parse::<i32>()
                    .map_err(|_| format!("Invalid tile y: {}", args[i + 2]))?;
                actions.push(CameraAction::CenterOnTile(x, y));
                i += 3;
            }
            _ => {
                return Err(format!("Unknown command: {}", args[i]));
            }
        }
    }
    
    Ok(actions)
}

fn execute_camera_script(
    mut script: ResMut<CameraScript>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut ui::world::camera::CameraState), With<ui::world::camera::IsometricCamera>>,
    grid_config: Res<ui::world::grid::GridConfig>,
    mut app_exit: EventWriter<AppExit>,
) {
    script.action_timer.tick(time.delta());
    
    if !script.action_timer.finished() {
        return;
    }
    
    if script.current_action >= script.actions.len() {
        println!("Camera script completed. Exiting...");
        app_exit.write(AppExit::Success);
        std::process::exit(0);
    }
    
    let Ok((mut transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };
    
    let action = script.actions[script.current_action].clone();
    match action {
        CameraAction::Pan(dx, dy) => {
            println!("Executing: Pan by ({}, {})", dx, dy);
            transform.translation.x += dx;
            transform.translation.y += dy;
            script.action_timer = Timer::from_seconds(0.1, TimerMode::Once);
        }
        CameraAction::Zoom(factor) => {
            println!("Executing: Zoom by factor {}", factor);
            camera_state.zoom *= factor;
            camera_state.zoom = camera_state.zoom.clamp(camera_state.min_zoom, camera_state.max_zoom);
            script.action_timer = Timer::from_seconds(0.1, TimerMode::Once);
        }
        CameraAction::SetZoom(level) => {
            println!("Executing: Set zoom to {}", level);
            camera_state.zoom = level.clamp(camera_state.min_zoom, camera_state.max_zoom);
            script.action_timer = Timer::from_seconds(0.1, TimerMode::Once);
        }
        CameraAction::Wait(seconds) => {
            println!("Executing: Wait {} seconds", seconds);
            script.action_timer = Timer::from_seconds(seconds, TimerMode::Once);
        }
        CameraAction::CenterOnTile(x, y) => {
            println!("Executing: Center on tile ({}, {})", x, y);
            let world_pos = ui::world::grid::coordinates::grid_to_world(x, y, 0, grid_config.tile_size);
            transform.translation.x = world_pos.x;
            transform.translation.y = world_pos.y;
            script.action_timer = Timer::from_seconds(0.1, TimerMode::Once);
        }
    }
    
    script.current_action += 1;
}

fn print_usage() {
    println!("Camera Script Runner - Execute camera movements and then quit");
    println!();
    println!("Usage: cargo run --bin camera_script -- [actions...]");
    println!();
    println!("Actions:");
    println!("  --pan, -p <dx> <dy>         Pan camera by dx, dy world units");
    println!("  --zoom, -z <factor>         Zoom by factor (2.0 = zoom in, 0.5 = zoom out)");
    println!("  --set-zoom, -sz <level>     Set absolute zoom level");
    println!("  --wait, -w <seconds>        Wait for specified seconds");
    println!("  --center, -c <x> <y>        Center camera on tile coordinates");
    println!();
    println!("Examples:");
    println!("  # Pan right 100 units, wait 2 seconds, then zoom in");
    println!("  cargo run --bin camera_script -- --pan 100 0 --wait 2 --zoom 2");
    println!();
    println!("  # Center on tile (50, 50), wait, then zoom out");
    println!("  cargo run --bin camera_script -- --center 50 50 --wait 1 --zoom 0.5");
    println!();
    println!("  # Complex sequence");
    println!("  cargo run --bin camera_script -- --center 100 100 --wait 1 --zoom 0.5 --wait 1 --pan -500 -500 --wait 1 --set-zoom 2.0");
}