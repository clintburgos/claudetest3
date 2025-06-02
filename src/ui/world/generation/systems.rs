//! Generation Systems - Systems that execute map generation
//!
//! This file contains the startup system that generates the initial map.
//! With view culling enabled, this system only initializes the grid structure.
//! Actual tile spawning is handled by the view culling system.
//!
//! # Process
//! 1. Read grid configuration
//! 2. Initialize empty GridMap with proper dimensions
//! 3. View culling system handles tile spawning based on camera position

use crate::ui::world::{
    grid::{GridConfig, GridMap},
    tiles::Tile,
};
use bevy::prelude::*;

/// Startup system that generates the initial map
/// Note: With view culling enabled, this only initializes resources.
/// Actual tile spawning is handled by the view culling system.
pub fn generate_map_system(mut grid_map: ResMut<GridMap>, grid_config: Res<GridConfig>) {
    info!(
        "Initializing map: {}x{} (view culling enabled)",
        grid_config.width, grid_config.height
    );

    // Just ensure the grid map is properly sized
    *grid_map = GridMap::new(grid_config.width, grid_config.height);

    info!("Map initialization complete");
}

/// Cleanup system that removes all tiles when exiting playing state
pub fn cleanup_map_system(
    mut commands: Commands,
    mut grid_map: ResMut<GridMap>,
    tile_query: Query<Entity, With<Tile>>,
    mut spawned_tiles: ResMut<crate::ui::world::tiles::SpawnedTiles>,
) {
    info!("Cleaning up map tiles");

    // Despawn all tile entities
    for entity in tile_query.iter() {
        commands.entity(entity).despawn();
    }

    // Clear the grid map
    grid_map.clear();

    // Clear spawned tiles tracking
    spawned_tiles.clear();

    info!("Map cleanup complete");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::world::tiles::{TileBiome, TilePosition};
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

        // With view culling, no tiles should be created immediately
        let grid_map = app.world().resource::<GridMap>();
        let mut tile_count = 0;
        for y in 0..10 {
            for x in 0..10 {
                if grid_map.get_tile(x, y).is_some() {
                    tile_count += 1;
                }
            }
        }

        assert_eq!(
            tile_count, 0,
            "No tiles should be created with view culling"
        );

        // Grid map should be properly sized
        assert_eq!(grid_map.dimensions(), (10, 10));
    }

    #[test]
    fn test_generate_map_system_grid_dimensions() {
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

        // Check that grid has correct dimensions
        let grid_map = app.world().resource::<GridMap>();
        assert_eq!(grid_map.dimensions(), (5, 5));

        // With view culling, no tiles should be spawned yet
        for y in 0..5 {
            for x in 0..5 {
                let entity = grid_map.get_tile(x, y);
                assert!(
                    entity.is_none(),
                    "Grid position ({}, {}) should not have a tile yet",
                    x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_cleanup_map_system() {
        let mut app = App::new();

        // Create some test tiles
        let mut grid_map = GridMap::new(3, 3);
        let mut spawned_tiles = crate::ui::world::tiles::SpawnedTiles::default();

        for y in 0..3 {
            for x in 0..3 {
                let entity = app
                    .world_mut()
                    .spawn((Tile, TilePosition::ground(x, y), TileBiome::Plain))
                    .id();
                grid_map.insert_tile(x, y, entity);
                spawned_tiles.insert(x, y);
            }
        }
        app.insert_resource(grid_map);
        app.insert_resource(spawned_tiles);

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

        // Verify spawned tiles is cleared
        let spawned_tiles = app
            .world()
            .resource::<crate::ui::world::tiles::SpawnedTiles>();
        assert!(
            !spawned_tiles.contains(1, 1),
            "SpawnedTiles should be cleared"
        );
    }

    #[test]
    fn test_cleanup_map_system_empty_grid() {
        let mut app = App::new();

        // Create empty grid
        app.insert_resource(GridMap::new(5, 5));
        app.insert_resource(crate::ui::world::tiles::SpawnedTiles::default());

        // Run cleanup on empty grid - should not panic
        app.world_mut()
            .run_system_once(cleanup_map_system)
            .expect("System should handle empty grid");
    }
}
