//! Tile Systems - Spawning and visual updates for tiles
//!
//! This file contains systems that manage tile entities:
//! - Spawning tiles with proper components
//! - Updating tile visuals based on biome
//! - Future: tile animations, state changes
//!
//! # Design Notes
//! - Tiles are spawned as individual entities for flexibility
//! - Visual updates are separate from spawning for modularity
//! - Color-based rendering for now, sprite support planned

use super::components::{Tile, TileBiome, TilePosition};
use crate::ui::world::grid::{coordinates::grid_to_world, GridConfig, GridMap};
use bevy::prelude::*;
use bevy::sprite::{Anchor, Sprite};

/// Spawn a single tile entity at the given position
pub fn spawn_tile(
    commands: &mut Commands,
    position: TilePosition,
    biome: TileBiome,
    tile_size: f32,
) -> Entity {
    let world_pos = grid_to_world(position.x, position.y, position.z, tile_size);

    // Create isometric diamond shape for the tile
    // Using a square sprite rotated 45 degrees gives us a diamond
    commands
        .spawn((
            Tile,
            position,
            biome,
            Sprite {
                color: biome.color(),
                custom_size: Some(Vec2::new(tile_size, tile_size * 0.5)), // 2:1 ratio for isometric
                anchor: Anchor::Center,
                ..default()
            },
            Transform::from_translation(world_pos),
            GlobalTransform::default(),
        ))
        .id()
}

/// System to spawn tiles for the entire grid
pub fn spawn_tile_system(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    mut grid_map: ResMut<GridMap>,
) {
    // Only spawn if grid is empty
    if grid_map.positions().next().is_some() {
        return;
    }

    info!(
        "Spawning {} x {} tile grid",
        grid_config.width, grid_config.height
    );

    // Spawn tiles for a small test grid (10x10 for now)
    let test_width = 10.min(grid_config.width);
    let test_height = 10.min(grid_config.height);

    for y in 0..test_height {
        for x in 0..test_width {
            // Validate bounds before spawning
            if !grid_map.in_bounds(x, y) {
                warn!(
                    "Attempting to spawn tile outside grid bounds at ({}, {})",
                    x, y
                );
                continue;
            }

            let position = TilePosition::ground(x, y);
            // Create a simple pattern for testing
            let biome = match (x + y) % 6 {
                0 => TileBiome::Plain,
                1 => TileBiome::Forest,
                2 => TileBiome::Coast,
                3 => TileBiome::Water,
                4 => TileBiome::Desert,
                _ => TileBiome::Mountain,
            };

            let entity = spawn_tile(&mut commands, position, biome, grid_config.tile_size);
            grid_map.insert_tile(x, y, entity);
        }
    }
}

/// Update tile visuals based on their biome type
pub fn update_tile_visuals_system(mut tiles: Query<(&TileBiome, &mut Sprite), Changed<TileBiome>>) {
    for (biome, mut sprite) in tiles.iter_mut() {
        sprite.color = biome.color();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_spawn_tile() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let entity = app
            .world_mut()
            .run_system_once(|mut commands: Commands| {
                spawn_tile(
                    &mut commands,
                    TilePosition::new(5, 10, 2),
                    TileBiome::Forest,
                    64.0,
                )
            })
            .expect("System should run successfully");

        let world = app.world();

        // Check entity exists with correct components
        assert!(world.get::<Tile>(entity).is_some());

        let position = world.get::<TilePosition>(entity).unwrap();
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 10);
        assert_eq!(position.z, 2);

        let biome = world.get::<TileBiome>(entity).unwrap();
        assert_eq!(*biome, TileBiome::Forest);

        let sprite = world.get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.color, TileBiome::Forest.color());
        assert_eq!(sprite.custom_size, Some(Vec2::new(64.0, 32.0))); // 2:1 ratio
        assert_eq!(sprite.anchor, Anchor::Center);

        let transform = world.get::<Transform>(entity).unwrap();
        let expected_pos = grid_to_world(5, 10, 2, 64.0);
        assert_eq!(transform.translation, expected_pos);
    }

    #[test]
    fn test_spawn_tile_system_empty_grid() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(GridConfig {
            tile_size: 32.0,
            width: 20,
            height: 20,
        });
        app.insert_resource(GridMap::new(20, 20));

        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("System should run");

        let grid_map = app.world().resource::<GridMap>();

        // Should spawn 10x10 tiles (test grid)
        let positions: Vec<_> = grid_map.positions().collect();
        assert_eq!(positions.len(), 100);

        // Check some tiles exist
        assert!(grid_map.get_tile(0, 0).is_some());
        assert!(grid_map.get_tile(9, 9).is_some());
        assert!(grid_map.get_tile(10, 10).is_none()); // Outside test grid
    }

    #[test]
    fn test_spawn_tile_system_non_empty_grid() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(GridConfig::default());

        let mut grid_map = GridMap::new(100, 100);
        grid_map.insert_tile(0, 0, Entity::from_raw(1));
        app.insert_resource(grid_map);

        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("System should run");

        let grid_map = app.world().resource::<GridMap>();

        // Should not spawn more tiles if grid is not empty
        let positions: Vec<_> = grid_map.positions().collect();
        assert_eq!(positions.len(), 1);
    }

    #[test]
    fn test_spawn_tile_system_biome_pattern() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.insert_resource(GridConfig::default());
        app.insert_resource(GridMap::new(100, 100));

        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("System should run");

        let world = app.world();
        let grid_map = world.resource::<GridMap>();

        // Test the biome pattern
        let test_cases = vec![
            ((0, 0), TileBiome::Plain),    // (0+0) % 6 = 0
            ((1, 0), TileBiome::Forest),   // (1+0) % 6 = 1
            ((0, 2), TileBiome::Coast),    // (0+2) % 6 = 2
            ((1, 2), TileBiome::Water),    // (1+2) % 6 = 3
            ((2, 2), TileBiome::Desert),   // (2+2) % 6 = 4
            ((3, 2), TileBiome::Mountain), // (3+2) % 6 = 5
        ];

        for ((x, y), expected_biome) in test_cases {
            let entity = grid_map.get_tile(x, y).expect("Tile should exist");
            let biome = world.get::<TileBiome>(entity).expect("Should have biome");
            assert_eq!(
                *biome, expected_biome,
                "Tile at ({}, {}) has wrong biome",
                x, y
            );
        }
    }

    #[test]
    fn test_update_tile_visuals_system() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Spawn a tile with initial biome
        let entity = app
            .world_mut()
            .spawn((
                Tile,
                TileBiome::Plain,
                Sprite {
                    color: Color::BLACK, // Wrong color initially
                    ..default()
                },
            ))
            .id();

        // Run the update system
        app.world_mut()
            .run_system_once(update_tile_visuals_system)
            .expect("System should run");

        // Color should update because TileBiome was just added (triggers Changed filter)
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.color, TileBiome::Plain.color());

        // Change the biome
        app.world_mut().entity_mut(entity).insert(TileBiome::Water);

        // Run the update system again
        app.world_mut()
            .run_system_once(update_tile_visuals_system)
            .expect("System should run");

        // Now color should update
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(sprite.color, TileBiome::Water.color());
    }

    #[test]
    fn test_spawn_tile_different_sizes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let tile_sizes = [16.0, 32.0, 64.0, 128.0];

        for &size in &tile_sizes {
            let entity = app
                .world_mut()
                .run_system_once(move |mut commands: Commands| {
                    spawn_tile(
                        &mut commands,
                        TilePosition::ground(0, 0),
                        TileBiome::Plain,
                        size,
                    )
                })
                .expect("System should run successfully");

            let sprite = app.world().get::<Sprite>(entity).unwrap();
            assert_eq!(
                sprite.custom_size,
                Some(Vec2::new(size, size * 0.5)),
                "Tile size {} should have 2:1 ratio",
                size
            );

            app.world_mut().despawn(entity);
        }
    }

    #[test]
    fn test_spawn_tile_all_biomes() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        let biomes = [
            TileBiome::Plain,
            TileBiome::Forest,
            TileBiome::Coast,
            TileBiome::Water,
            TileBiome::Desert,
            TileBiome::Mountain,
        ];

        for (i, &biome) in biomes.iter().enumerate() {
            let entity = app
                .world_mut()
                .run_system_once(move |mut commands: Commands| {
                    spawn_tile(
                        &mut commands,
                        TilePosition::ground(i as i32, 0),
                        biome,
                        64.0,
                    )
                })
                .expect("System should run successfully");

            let world = app.world();
            let stored_biome = world.get::<TileBiome>(entity).unwrap();
            assert_eq!(*stored_biome, biome);

            let sprite = world.get::<Sprite>(entity).unwrap();
            assert_eq!(sprite.color, biome.color());
        }
    }

    #[test]
    fn test_spawn_tile_system_respects_grid_bounds() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a very small grid
        app.insert_resource(GridConfig {
            tile_size: 32.0,
            width: 5,
            height: 5,
        });
        app.insert_resource(GridMap::new(5, 5));

        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("System should run");

        let grid_map = app.world().resource::<GridMap>();

        // Should only spawn 5x5 tiles even though test grid tries for 10x10
        let positions: Vec<_> = grid_map.positions().collect();
        assert_eq!(positions.len(), 25);

        // Check bounds
        for (x, y) in positions {
            assert!(*x >= 0 && *x < 5);
            assert!(*y >= 0 && *y < 5);
        }
    }

    #[test]
    fn test_spawn_tile_system_handles_zero_size_grid() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a grid with zero size
        app.insert_resource(GridConfig {
            tile_size: 32.0,
            width: 0,
            height: 0,
        });
        app.insert_resource(GridMap::new(0, 0));

        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("System should run");

        let grid_map = app.world().resource::<GridMap>();

        // Should spawn no tiles
        let positions: Vec<_> = grid_map.positions().collect();
        assert_eq!(positions.len(), 0);
    }
}
