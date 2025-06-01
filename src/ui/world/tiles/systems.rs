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

use bevy::prelude::*;
use super::components::{Tile, TilePosition, TileBiome};
use crate::ui::world::grid::coordinates::grid_to_world;

/// Spawn a single tile entity at the given position
pub fn spawn_tile(
    commands: &mut Commands,
    position: TilePosition,
    biome: TileBiome,
    tile_size: f32,
) -> Entity {
    let world_pos = grid_to_world(position.x, position.y, position.z, tile_size);
    
    commands.spawn((
        Tile,
        position,
        biome,
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(world_pos.x),
            top: Val::Px(world_pos.y),
            width: Val::Px(tile_size),
            height: Val::Px(tile_size * 0.5), // Isometric ratio
            ..default()
        },
        BackgroundColor(biome.color()),
    )).id()
}

/// Update tile visuals based on their biome type
pub fn update_tile_visuals_system(
    mut tiles: Query<(&TileBiome, &mut BackgroundColor), Changed<TileBiome>>,
) {
    for (biome, mut color) in tiles.iter_mut() {
        *color = BackgroundColor(biome.color());
    }
}