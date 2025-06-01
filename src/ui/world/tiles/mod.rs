//! Tiles Module - Individual tile entities and rendering
//! 
//! This module manages the individual tiles that make up the world map.
//! Each tile is an entity with position, biome, and visual components.
//! 
//! # Responsibilities
//! - Define tile components (position, biome, marker)
//! - Spawn tile entities with proper transforms
//! - Update tile visuals based on biome type
//! - Provide tile querying capabilities
//! 
//! # Components
//! - `Tile`: Marker component for tile entities
//! - `TilePosition`: Grid coordinates (x, y, z)
//! - `TileBiome`: Terrain type (Plain, Forest, etc.)

use bevy::prelude::*;

pub mod components;
pub mod systems;

pub use components::{Tile, TilePosition, TileBiome};
pub use systems::spawn_tile;

/// Plugin that manages tile entities and rendering
pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::update_tile_visuals_system);
    }
}