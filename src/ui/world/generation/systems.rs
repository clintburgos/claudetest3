//! Generation Systems - Systems that execute map generation
//!
//! This file contains the startup system that generates the initial map.
//! It uses the MapGenerator to create biomes, then spawns tile entities.
//!
//! # Process
//! 1. Read grid configuration
//! 2. Generate biome map
//! 3. Spawn tile entity for each position
//! 4. Update GridMap resource with references

use super::generator::{DefaultMapGenerator, MapGenerator};
use crate::ui::world::{
    grid::{GridConfig, GridMap},
    tiles::{spawn_tile, Tile, TilePosition},
};
use bevy::prelude::*;

/// Startup system that generates the initial map
pub fn generate_map_system(
    mut commands: Commands,
    mut grid_map: ResMut<GridMap>,
    grid_config: Res<GridConfig>,
) {
    info!(
        "Generating map: {}x{}",
        grid_config.width, grid_config.height
    );

    // Create generator
    let generator = DefaultMapGenerator::default();

    // Generate biome map
    let biome_map = generator.generate(grid_config.width, grid_config.height);

    // Spawn tiles
    for y in 0..grid_config.height {
        for x in 0..grid_config.width {
            let biome = biome_map[y as usize][x as usize];
            let position = TilePosition::ground(x, y);

            let entity = spawn_tile(&mut commands, position, biome, grid_config.tile_size);

            grid_map.insert_tile(x, y, entity);
        }
    }

    info!("Map generation complete");
}

/// Cleanup system that removes all tiles when exiting playing state
pub fn cleanup_map_system(
    mut commands: Commands,
    mut grid_map: ResMut<GridMap>,
    tile_query: Query<Entity, With<Tile>>,
) {
    info!("Cleaning up map tiles");

    // Despawn all tile entities
    for entity in tile_query.iter() {
        commands.entity(entity).despawn();
    }

    // Clear the grid map
    grid_map.clear();

    info!("Map cleanup complete");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::world::TileBiome;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_generate_map_system() {
        let mut app = App::new();

        // Add required resources
        let config = GridConfig {
            width: 10,
            height: 10,
            tile_size: 32.0,
        };
        app.insert_resource(config);
        app.insert_resource(GridMap::new(10, 10));

        // Run the system
        app.world_mut()
            .run_system_once(generate_map_system)
            .expect("System should run");

        // Check that tiles were created
        let grid_map = app.world().resource::<GridMap>();
        let mut tile_count = 0;
        for y in 0..10 {
            for x in 0..10 {
                if grid_map.get_tile(x, y).is_some() {
                    tile_count += 1;
                }
            }
        }

        assert_eq!(tile_count, 100, "Should create exactly 100 tiles");

        // Verify all entities have required components
        let mut tile_query = app
            .world_mut()
            .query::<(&Tile, &TilePosition, &TileBiome)>();
        let tiles: Vec<_> = tile_query.iter(&app.world()).collect();
        assert_eq!(
            tiles.len(),
            100,
            "All tiles should have required components"
        );
    }

    #[test]
    fn test_generate_map_system_grid_positions() {
        let mut app = App::new();

        // Small grid for easier testing
        let config = GridConfig {
            width: 5,
            height: 5,
            tile_size: 32.0,
        };
        app.insert_resource(config);
        app.insert_resource(GridMap::new(5, 5));

        // Run the system
        app.world_mut()
            .run_system_once(generate_map_system)
            .expect("System should run");

        // Check that each position has a tile
        let grid_map = app.world().resource::<GridMap>();
        for y in 0..5 {
            for x in 0..5 {
                let entity = grid_map.get_tile(x, y);
                assert!(
                    entity.is_some(),
                    "Grid position ({}, {}) should have a tile",
                    x,
                    y
                );

                // Verify tile has correct position
                if let Some(entity) = entity {
                    let position = app.world().get::<TilePosition>(entity).unwrap();
                    assert_eq!(position.x, x);
                    assert_eq!(position.y, y);
                    assert_eq!(position.z, 0);
                }
            }
        }
    }

    #[test]
    fn test_cleanup_map_system() {
        let mut app = App::new();

        // Create some test tiles
        let mut grid_map = GridMap::new(3, 3);
        for y in 0..3 {
            for x in 0..3 {
                let entity = app
                    .world_mut()
                    .spawn((Tile, TilePosition::ground(x, y), TileBiome::Plain))
                    .id();
                grid_map.insert_tile(x, y, entity);
            }
        }
        app.insert_resource(grid_map);

        // Verify tiles exist
        let tile_count = app.world_mut().query::<&Tile>().iter(&app.world()).count();
        assert_eq!(tile_count, 9, "Should have 9 tiles before cleanup");

        // Run cleanup
        app.world_mut()
            .run_system_once(cleanup_map_system)
            .expect("System should run");

        // Verify all tiles are gone
        let tile_count = app.world_mut().query::<&Tile>().iter(&app.world()).count();
        assert_eq!(tile_count, 0, "Should have 0 tiles after cleanup");

        // Verify grid map is cleared
        let grid_map = app.world().resource::<GridMap>();
        for y in 0..3 {
            for x in 0..3 {
                assert!(
                    grid_map.get_tile(x, y).is_none(),
                    "Grid position ({}, {}) should be empty",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_cleanup_map_system_empty_grid() {
        let mut app = App::new();

        // Create empty grid
        app.insert_resource(GridMap::new(5, 5));

        // Run cleanup on empty grid - should not panic
        app.world_mut()
            .run_system_once(cleanup_map_system)
            .expect("System should handle empty grid");
    }
}
