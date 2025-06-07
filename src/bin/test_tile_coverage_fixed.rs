use bevy::prelude::*;
use bevy::render::view::screenshot::{save_to_disk, Screenshot};
use bevy::window::WindowMode;
use claudetest3::{game, ui};
use claudetest3::ui::world::{grid, generation, tiles};

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
            // Add world plugins individually without camera
            grid::GridPlugin,
            generation::MapGenerationPlugin,
            tiles::TilePlugin,
            // Don't add camera::IsometricCameraPlugin
            ui::panels::UIPanelsPlugin,
            TileCoverageTestPlugin,
        ))
        .configure_sets(
            Startup,
            (
                ui::world::WorldSystems::GridInit,
                ui::world::WorldSystems::MapGeneration,
                ui::world::WorldSystems::CameraSetup,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                ui::world::WorldSystems::TileSpawn,
                ui::world::WorldSystems::TileUpdate,
                ui::world::WorldSystems::CameraUpdate,
            )
                .chain(),
        )
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
            .add_systems(
                OnEnter(GameState::Playing),
                setup_custom_camera.after(ui::world::WorldSystems::TileSpawn),
            )
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

fn setup_custom_camera(mut commands: Commands) {
    // Spawn camera with DisableCameraConstraints from the start
    commands.spawn((
        Camera2d,
        ui::world::camera::IsometricCamera,
        ui::world::camera::CameraState {
            zoom: 1.0,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
        ui::world::camera::DisableCameraConstraints, // Ensure this is added from the start
    ));
}

fn run_coverage_test(
    mut commands: Commands,
    mut test_state: ResMut<TestState>,
    time: Res<Time>,
    mut camera_query: Query<(&mut Transform, &mut ui::world::camera::CameraState), With<ui::world::camera::IsometricCamera>>,
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
    
    let Ok((mut transform, mut camera_state)) = camera_query.single_mut() else {
        return;
    };
    
    // Define comprehensive test cases
    // Note: For a 200x200 isometric map with tile_size=64:
    // - Map center is at world(0, -3200) 
    // - World bounds are approximately X: -6368 to 6368, Y: -6368 to 0
    let tests = vec![
        // Center positions at various zoom levels
        CoverageTest { name: "center_default", camera_x: 0.0, camera_y: -3200.0, zoom: 1.0 },
        CoverageTest { name: "center_zoomed_out", camera_x: 0.0, camera_y: -3200.0, zoom: 0.3 },
        CoverageTest { name: "center_zoomed_in", camera_x: 0.0, camera_y: -3200.0, zoom: 3.0 },
        
        // Edge positions (within actual map bounds)
        CoverageTest { name: "top_edge", camera_x: 0.0, camera_y: -100.0, zoom: 1.0 },
        CoverageTest { name: "bottom_edge", camera_x: 0.0, camera_y: -6000.0, zoom: 1.0 },
        CoverageTest { name: "left_edge", camera_x: -6000.0, camera_y: -3200.0, zoom: 1.0 },
        CoverageTest { name: "right_edge", camera_x: 6000.0, camera_y: -3200.0, zoom: 1.0 },
        
        // Corner positions
        CoverageTest { name: "top_right_corner", camera_x: 6000.0, camera_y: -3000.0, zoom: 1.0 },
        CoverageTest { name: "top_left_corner", camera_x: -6000.0, camera_y: -3000.0, zoom: 1.0 },
        CoverageTest { name: "bottom_right_corner", camera_x: 0.0, camera_y: -6000.0, zoom: 1.0 },
        CoverageTest { name: "bottom_left_corner", camera_x: 0.0, camera_y: -6000.0, zoom: 1.0 },
        
        // Edge positions zoomed out (most likely to show gaps)
        CoverageTest { name: "edge_zoom_out_1", camera_x: 5000.0, camera_y: -2000.0, zoom: 0.2 },
        CoverageTest { name: "edge_zoom_out_2", camera_x: -5000.0, camera_y: -2000.0, zoom: 0.2 },
        CoverageTest { name: "edge_zoom_out_3", camera_x: 5000.0, camera_y: -5000.0, zoom: 0.2 },
        CoverageTest { name: "edge_zoom_out_4", camera_x: -5000.0, camera_y: -5000.0, zoom: 0.2 },
        
        // Intermediate positions
        CoverageTest { name: "mid_right", camera_x: 3000.0, camera_y: -3200.0, zoom: 0.5 },
        CoverageTest { name: "mid_left", camera_x: -3000.0, camera_y: -3200.0, zoom: 0.5 },
        CoverageTest { name: "mid_top", camera_x: 0.0, camera_y: -1600.0, zoom: 0.5 },
        CoverageTest { name: "mid_bottom", camera_x: 0.0, camera_y: -4800.0, zoom: 0.5 },
        
        // Extreme zoom out (full map view)
        CoverageTest { name: "full_map_view", camera_x: 0.0, camera_y: -3200.0, zoom: 0.1 },
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