//! Spawn All Tiles System - Spawns all tiles when culling is disabled
//!
//! This system spawns all tiles in the map when view culling is disabled.
//! It runs as a fallback to ensure tiles are visible even without culling.

use super::{spawn_tile, SpawnedTiles, ViewCullingConfig, TilePosition, TileBiome};
use crate::ui::world::tiles::systems::TileMeshes;
use crate::ui::world::grid::{GridConfig, GridMap};
use bevy::prelude::*;

/// Spawn all tiles in the map when culling is disabled
pub fn spawn_all_tiles_system(
    mut commands: Commands,
    culling_config: Res<ViewCullingConfig>,
    grid_config: Res<GridConfig>,
    tile_meshes: Res<TileMeshes>,
    mut spawned_tiles: ResMut<SpawnedTiles>,
    mut grid_map: ResMut<GridMap>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut biome_cache: Local<Vec<Vec<TileBiome>>>,
) {
    // Only run if culling is disabled and we haven't spawned all tiles yet
    if culling_config.enabled {
        return;
    }
    
    if spawned_tiles.count() >= (grid_config.width * grid_config.height) as usize {
        return;
    }
    
    info!("spawn_all_tiles_system: Culling is disabled, spawning tiles...");

    // Initialize biome cache if needed
    if biome_cache.is_empty() {
        use crate::ui::world::generation::generator::{DefaultMapGenerator, MapGenerator};
        let generator = DefaultMapGenerator::default();
        *biome_cache = generator.generate(grid_config.width, grid_config.height);
    }

    info!("Culling disabled - spawning all {} tiles", grid_config.width * grid_config.height);

    // Spawn all tiles
    let mut spawn_count = 0;
    for y in 0..grid_config.height {
        for x in 0..grid_config.width {
            if !spawned_tiles.contains(x, y) {
                // Get biome from cache
                let biome = biome_cache[y as usize][x as usize];
                let position = TilePosition::ground(x, y);
                
                // Spawn tile
                let entity = spawn_tile(
                    &mut commands,
                    position,
                    biome,
                    grid_config.tile_size,
                    tile_meshes.diamond.clone(),
                    &mut materials,
                );
                
                // Update tracking
                grid_map.insert_tile(x, y, entity);
                spawned_tiles.insert(x, y);
                spawn_count += 1;
            }
        }
    }

    if spawn_count > 0 {
        info!("Spawned {} tiles (culling disabled)", spawn_count);
    }
}