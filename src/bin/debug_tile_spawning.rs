//! Debug tile spawning to understand why tiles are missing

use bevy::prelude::*;
use claudetest3::ui::world::{
    camera::{CameraState, IsometricCamera},
    grid::{coordinates::*, GridConfig, GridMap},
    tiles::{
        components::{Tile, TileBiome, TilePosition},
        systems::{create_tile_diamond_mesh, spawn_tile, TileMeshes},
    },
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Playing,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        .insert_resource(GridConfig {
            width: 200,
            height: 200,
            tile_size: 64.0,
        })
        .insert_resource(GridMap::default())
        .add_systems(Startup, setup)
        .add_systems(Update, check_tile_pattern)
        .run();
}

fn setup(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create tile meshes
    let tile_mesh = create_tile_diamond_mesh(grid_config.tile_size, grid_config.tile_size * 0.5);
    commands.insert_resource(TileMeshes {
        diamond: meshes.add(tile_mesh),
    });

    // Setup camera at center of map
    let center = grid_center_world(grid_config.width, grid_config.height, grid_config.tile_size);
    commands.spawn((
        Camera2d::default(),
        IsometricCamera,
        CameraState {
            zoom: 1.0,
            min_zoom: 0.1,
            max_zoom: 5.0,
            ..default()
        },
        Transform::from_xyz(center.x, center.y, 1000.0),
    ));
    
    // Manually spawn a grid of tiles to test
    let min_x = 90;
    let max_x = 110;
    let min_y = 90;
    let max_y = 110;
    
    info!("Spawning test grid from ({},{}) to ({},{})", min_x, min_y, max_x, max_y);
    
    let mut spawn_count = 0;
    let tile_meshes = commands.spawn_empty().id();
    
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let position = TilePosition::ground(x, y);
            let biome = TileBiome::Plain;
            
            // Calculate expected position
            let world_pos = grid_to_world(x, y, 0, grid_config.tile_size);
            
            // Spawn tile directly
            let entity = commands.spawn((
                Mesh2d(meshes.add(create_tile_diamond_mesh(grid_config.tile_size, grid_config.tile_size * 0.5))),
                MeshMaterial2d(materials.add(ColorMaterial::from_color(biome.color()))),
                Transform::from_translation(world_pos),
                Tile,
                position,
                biome,
            )).id();
            
            spawn_count += 1;
            
            if spawn_count <= 5 || (x == 100 && y == 100) {
                info!("Spawned tile {} at grid ({},{}) world ({:.1},{:.1},{:.1})", 
                    spawn_count, x, y, world_pos.x, world_pos.y, world_pos.z);
            }
        }
    }
    
    info!("Total tiles spawned: {}", spawn_count);
    info!("Expected tiles: {}", (max_x - min_x + 1) * (max_y - min_y + 1));
}

fn check_tile_pattern(
    tile_query: Query<&TilePosition, With<Tile>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    static mut CHECKED: bool = false;
    
    if keyboard_input.just_pressed(KeyCode::Space) || unsafe { !CHECKED } {
        unsafe { CHECKED = true; }
        
        // Collect all tile positions
        let mut positions = HashSet::new();
        for pos in tile_query.iter() {
            positions.insert((pos.x, pos.y));
        }
        
        info!("Found {} tiles in the world", positions.len());
        
        // Check for gaps in a 21x21 grid around center
        let mut missing = Vec::new();
        for y in 90..=110 {
            for x in 90..=110 {
                if !positions.contains(&(x, y)) {
                    missing.push((x, y));
                }
            }
        }
        
        if !missing.is_empty() {
            warn!("Missing {} tiles!", missing.len());
            for (x, y) in missing.iter().take(10) {
                warn!("  Missing tile at ({}, {})", x, y);
            }
            if missing.len() > 10 {
                warn!("  ... and {} more", missing.len() - 10);
            }
        } else {
            info!("All expected tiles are present!");
        }
        
        // Print a visual pattern
        println!("\nTile pattern (# = present, . = missing):");
        for y in 95..=105 {
            let mut line = String::new();
            for x in 95..=105 {
                if positions.contains(&(x, y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
                line.push(' ');
            }
            println!("{}", line);
        }
    }
}