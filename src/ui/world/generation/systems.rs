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

use bevy::prelude::*;
use crate::ui::world::{
    tiles::{spawn_tile, TilePosition},
    grid::{GridMap, GridConfig},
};
use super::generator::{MapGenerator, DefaultMapGenerator};

/// Startup system that generates the initial map
pub fn generate_map_system(
    mut commands: Commands,
    mut grid_map: ResMut<GridMap>,
    grid_config: Res<GridConfig>,
) {
    info!("Generating map: {}x{}", grid_config.width, grid_config.height);
    
    // Create generator
    let generator = DefaultMapGenerator::default();
    
    // Generate biome map
    let biome_map = generator.generate(grid_config.width, grid_config.height);
    
    // Spawn tiles
    for y in 0..grid_config.height {
        for x in 0..grid_config.width {
            let biome = biome_map[y as usize][x as usize];
            let position = TilePosition::ground(x, y);
            
            let entity = spawn_tile(
                &mut commands,
                position,
                biome,
                grid_config.tile_size,
            );
            
            grid_map.insert_tile(x, y, entity);
        }
    }
    
    info!("Map generation complete");
}