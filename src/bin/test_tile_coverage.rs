use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::window::WindowMode;
use claudetest3::{game, ui};

/// Test program that moves camera to various positions and takes screenshots
/// to verify tiles always cover the visible area
fn main() {
    println!("=== Tile Coverage Test ===");
    println!("This will test various camera positions and zoom levels");
    println!("Screenshots will be saved to tile_coverage_test/");
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tile Coverage Test".to_string(),
                resolution: (1280., 720.).into(),
                mode: WindowMode::Windowed,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            game::GameStatePlugin,
            ui::world::WorldPlugin,
            ui::panels::UIPanelsPlugin,
            TileCoverageTestPlugin,
        ))
        .run();
}

struct TileCoverageTestPlugin;

impl Plugin for TileCoverageTestPlugin {
    fn build(&self, app: &mut App) {
        use claudetest3::game::GameState;
        
        app.insert_resource(TestState::default())
            .add_systems(Startup, |mut next_state: ResMut<NextState<GameState>>| {
                // Automatically start playing
                next_state.set(GameState::Playing);
            })
            .add_systems(Update, run_coverage_test.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Resource)]
struct TestState {
    current_test: usize,
    timer: Timer,
    screenshot_pending: bool,
    tests_completed: bool,
}

impl Default for TestState {
    fn default() -> Self {
        Self {
            current_test: 0,
            timer: Timer::from_seconds(1.5, TimerMode::Repeating),
            screenshot_pending: false,
            tests_completed: false,
        }
    }
}

#[derive(Debug, Clone)]
struct CoverageTest {
    name: &'static str,
    camera_x: f32,
    camera_y: f32,
    zoom: f32,
}

fn run_coverage_test(
    mut commands: Commands,
    mut test_state: ResMut<TestState>,
    time: Res<Time>,
    mut camera_query: Query<(Entity, &mut Transform, &mut claudetest3::ui::world::camera::CameraState), With<claudetest3::ui::world::camera::IsometricCamera>>,
    mut app_exit: EventWriter<AppExit>,
) {
    if test_state.tests_completed {
        return;
    }
    
    // Update timer
    test_state.timer.tick(time.delta());
    
    if !test_state.timer.just_finished() {
        return;
    }
    
    // Skip if waiting for screenshot
    if test_state.screenshot_pending {
        test_state.screenshot_pending = false;
        return;
    }
    
    let Ok((camera_entity, mut transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };
    
    // Add DisableCameraConstraints component if not already present
    if test_state.current_test == 0 {
        commands.entity(camera_entity).insert(claudetest3::ui::world::camera::DisableCameraConstraints);
    }
    
    // Define comprehensive test cases
    let tests = vec![
        // Center positions at various zoom levels
        CoverageTest { name: "center_default", camera_x: 0.0, camera_y: 0.0, zoom: 1.0 },
        CoverageTest { name: "center_zoomed_out", camera_x: 0.0, camera_y: 0.0, zoom: 0.3 },
        CoverageTest { name: "center_zoomed_in", camera_x: 0.0, camera_y: 0.0, zoom: 3.0 },
        
        // Edge positions
        CoverageTest { name: "top_left", camera_x: -6000.0, camera_y: -3000.0, zoom: 1.0 },
        CoverageTest { name: "top_right", camera_x: 6000.0, camera_y: -3000.0, zoom: 1.0 },
        CoverageTest { name: "bottom_left", camera_x: -6000.0, camera_y: -3000.0, zoom: 1.0 },
        CoverageTest { name: "bottom_right", camera_x: 6000.0, camera_y: -3000.0, zoom: 1.0 },
        
        // Edge positions zoomed out (most likely to show gaps)
        CoverageTest { name: "edge_zoom_out_1", camera_x: 5000.0, camera_y: -2000.0, zoom: 0.2 },
        CoverageTest { name: "edge_zoom_out_2", camera_x: -5000.0, camera_y: -2000.0, zoom: 0.2 },
        CoverageTest { name: "edge_zoom_out_3", camera_x: 5000.0, camera_y: -5000.0, zoom: 0.2 },
        CoverageTest { name: "edge_zoom_out_4", camera_x: -5000.0, camera_y: -5000.0, zoom: 0.2 },
        
        // Intermediate positions
        CoverageTest { name: "mid_right", camera_x: 1500.0, camera_y: 0.0, zoom: 0.5 },
        CoverageTest { name: "mid_left", camera_x: -1500.0, camera_y: 0.0, zoom: 0.5 },
        CoverageTest { name: "mid_top", camera_x: 0.0, camera_y: 1500.0, zoom: 0.5 },
        CoverageTest { name: "mid_bottom", camera_x: 0.0, camera_y: -1500.0, zoom: 0.5 },
        
        // Diagonal positions at various zooms
        CoverageTest { name: "diagonal_ne", camera_x: 1000.0, camera_y: 1000.0, zoom: 0.8 },
        CoverageTest { name: "diagonal_nw", camera_x: -1000.0, camera_y: 1000.0, zoom: 0.8 },
        CoverageTest { name: "diagonal_se", camera_x: 1000.0, camera_y: -1000.0, zoom: 0.8 },
        CoverageTest { name: "diagonal_sw", camera_x: -1000.0, camera_y: -1000.0, zoom: 0.8 },
        
        // Extreme zoom out (full map view)
        CoverageTest { name: "full_map_view", camera_x: 0.0, camera_y: 0.0, zoom: 0.1 },
    ];
    
    if test_state.current_test >= tests.len() {
        test_state.tests_completed = true;
        println!("\nAll tests completed!");
        println!("Check tile_coverage_test/ directory for screenshots");
        println!("Look for any black areas where tiles should be visible");
        
        // Run analysis
        analyze_results();
        
        app_exit.write(AppExit::Success);
        return;
    }
    
    let test = &tests[test_state.current_test];
    
    // Apply test configuration
    println!("Test {}/{}: {} - pos({}, {}), zoom({})", 
        test_state.current_test + 1, 
        tests.len(),
        test.name,
        test.camera_x,
        test.camera_y,
        test.zoom
    );
    
    transform.translation.x = test.camera_x;
    transform.translation.y = test.camera_y;
    camera_state.zoom = test.zoom;
    
    // Take screenshot
    let filename = format!("tile_coverage_test/{:02}_{}.png", test_state.current_test, test.name);
    
    // Create directory
    std::fs::create_dir_all("tile_coverage_test").ok();
    
    commands
        .spawn(Screenshot::primary_window())
        .observe(save_to_disk(filename));
    
    test_state.screenshot_pending = true;
    test_state.current_test += 1;
}

fn analyze_results() {
    println!("\n=== Analysis Tips ===");
    println!("1. Open the screenshots in tile_coverage_test/");
    println!("2. Look for black background areas where tiles should be");
    println!("3. Pay special attention to:");
    println!("   - Edge test cases (top_left, bottom_right, etc.)");
    println!("   - Zoomed out views (edge_zoom_out_*)");
    println!("   - The full map view");
    println!("\nIf you see black areas, the tile culling system needs adjustment.");
}