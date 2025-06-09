use bevy::prelude::*;
use claudetest3::{game, ui};
use std::env;

#[derive(Debug, Clone, Copy)]
enum TourType {
    MapOverview,    // Zoom out to see entire map
    CornerTour,     // Visit all four corners
    ZoomDemo,       // Demonstrate zoom levels
    EdgeScroll,     // Scroll along edges
}

#[derive(Resource)]
struct CameraTour {
    tour_type: TourType,
    step: usize,
    timer: Timer,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let tour_type = if args.len() > 1 {
        match args[1].as_str() {
            "overview" => TourType::MapOverview,
            "corners" => TourType::CornerTour,
            "zoom" => TourType::ZoomDemo,
            "edges" => TourType::EdgeScroll,
            _ => {
                print_usage();
                return;
            }
        }
    } else {
        print_usage();
        return;
    };
    
    println!("Starting camera tour: {:?}", tour_type);
    
    let mut app = App::new();
    
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("Camera Tour - {:?}", tour_type).to_string(),
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
    .insert_resource(CameraTour {
        tour_type,
        step: 0,
        timer: Timer::from_seconds(2.0, TimerMode::Once),
    })
    // Skip menu and go directly to playing
    .add_systems(Startup, |mut next_state: ResMut<NextState<game::GameState>>| {
        next_state.set(game::GameState::Playing);
    })
    .add_systems(Update, execute_camera_tour.run_if(in_state(game::GameState::Playing)));
    
    app.run();
}

fn execute_camera_tour(
    mut tour: ResMut<CameraTour>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut ui::world::camera::CameraState), With<ui::world::camera::IsometricCamera>>,
    grid_config: Res<ui::world::grid::GridConfig>,
    mut app_exit: EventWriter<AppExit>,
) {
    tour.timer.tick(time.delta());
    
    if !tour.timer.finished() {
        return;
    }
    
    println!("Timer finished, executing step {} of {:?}", tour.step, tour.tour_type);
    
    let Ok((mut transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };
    
    match tour.tour_type {
        TourType::MapOverview => {
            match tour.step {
                0 => {
                    println!("Step 1: Center on map");
                    let center = ui::world::grid::coordinates::grid_center_world(
                        grid_config.width,
                        grid_config.height,
                        grid_config.tile_size,
                    );
                    transform.translation.x = center.x;
                    transform.translation.y = center.y;
                    tour.timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
                1 => {
                    println!("Step 2: Zoom out to minimum");
                    camera_state.zoom = camera_state.min_zoom;
                    tour.timer = Timer::from_seconds(3.0, TimerMode::Once);
                }
                2 => {
                    println!("Step 3: Slowly zoom in");
                    camera_state.zoom = 1.0;
                    tour.timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
                _ => {
                    println!("Tour complete!");
                    app_exit.write(AppExit::Success);
                    return;
                }
            }
        }
        
        TourType::CornerTour => {
            let positions = [
                (10, 10, "bottom-left"),
                (grid_config.width - 10, 10, "bottom-right"),
                (grid_config.width - 10, grid_config.height - 10, "top-right"),
                (10, grid_config.height - 10, "top-left"),
                (grid_config.width / 2, grid_config.height / 2, "center"),
            ];
            
            if tour.step < positions.len() {
                let (x, y, name) = positions[tour.step];
                println!("Step {}: Moving to {} corner (tile {}, {})", tour.step + 1, name, x, y);
                let world_pos = ui::world::grid::coordinates::grid_to_world(x, y, 0, grid_config.tile_size);
                transform.translation.x = world_pos.x;
                transform.translation.y = world_pos.y;
                tour.timer = Timer::from_seconds(2.0, TimerMode::Once);
            } else {
                println!("Tour complete! Exiting...");
                app_exit.write(AppExit::Success);
                std::process::exit(0);
            }
        }
        
        TourType::ZoomDemo => {
            let zoom_levels = [
                (camera_state.min_zoom, "Minimum zoom (full map)"),
                (0.5, "Half zoom"),
                (1.0, "Default zoom"),
                (2.0, "2x zoom"),
                (5.0, "5x zoom"),
                (camera_state.max_zoom, "Maximum zoom"),
                (1.0, "Back to default"),
            ];
            
            if tour.step < zoom_levels.len() {
                let (zoom, desc) = zoom_levels[tour.step];
                println!("Step {}: {} (zoom = {:.2})", tour.step + 1, desc, zoom);
                camera_state.zoom = zoom.clamp(camera_state.min_zoom, camera_state.max_zoom);
                tour.timer = Timer::from_seconds(2.0, TimerMode::Once);
            } else {
                println!("Tour complete! Exiting...");
                app_exit.write(AppExit::Success);
                std::process::exit(0);
            }
        }
        
        TourType::EdgeScroll => {
            match tour.step {
                0 => {
                    println!("Step 1: Move to left edge");
                    let world_pos = ui::world::grid::coordinates::grid_to_world(10, grid_config.height / 2, 0, grid_config.tile_size);
                    transform.translation.x = world_pos.x;
                    transform.translation.y = world_pos.y;
                    tour.timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
                1..=10 => {
                    println!("Step {}: Scrolling right", tour.step + 1);
                    transform.translation.x += 200.0;
                    tour.timer = Timer::from_seconds(0.5, TimerMode::Once);
                }
                11 => {
                    println!("Step 12: Move to top edge");
                    let world_pos = ui::world::grid::coordinates::grid_to_world(grid_config.width / 2, grid_config.height - 10, 0, grid_config.tile_size);
                    transform.translation.x = world_pos.x;
                    transform.translation.y = world_pos.y;
                    tour.timer = Timer::from_seconds(2.0, TimerMode::Once);
                }
                12..=21 => {
                    println!("Step {}: Scrolling down", tour.step + 1);
                    transform.translation.y -= 200.0;
                    tour.timer = Timer::from_seconds(0.5, TimerMode::Once);
                }
                _ => {
                    println!("Tour complete!");
                    app_exit.write(AppExit::Success);
                    return;
                }
            }
        }
    }
    
    tour.step += 1;
}

fn print_usage() {
    println!("Camera Tour - Predefined camera movement sequences");
    println!();
    println!("Usage: cargo run --bin camera_tour -- <tour_type>");
    println!();
    println!("Tour types:");
    println!("  overview    - Zoom out to see entire map, then zoom back in");
    println!("  corners     - Visit all four corners of the map");
    println!("  zoom        - Demonstrate different zoom levels");
    println!("  edges       - Scroll along the edges of the map");
    println!();
    println!("Example:");
    println!("  cargo run --bin camera_tour -- overview");
}