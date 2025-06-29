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
//! - Manage view culling for performance optimization
//!
//! # Components
//! - `Tile`: Marker component for tile entities
//! - `TilePosition`: Grid coordinates (x, y, z)
//! - `TileBiome`: Terrain type (Plain, Forest, etc.)

use crate::game::GameState;
use bevy::prelude::*;

pub mod components;
pub mod culling_toggle;
pub mod interaction;
pub mod isometric_culling;
pub mod mesh_tiles;
pub mod spawn_all_tiles;
pub mod systems;
pub mod view_culling;

pub use components::{Tile, TileBiome, TileHighlighted, TilePosition, TileSelected};
pub use interaction::{HoveredTile, SelectedTile, TileInteractionPlugin};
pub use systems::{init_tile_meshes, spawn_tile, spawn_tile_system, TileMeshes};
pub use view_culling::{
    clear_spawned_tiles_system, view_culling_system, SpawnedTiles, ViewCullingConfig,
};

/// Plugin that manages tile entities and rendering
pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        use crate::ui::world::WorldSystems;

        // Note: Tile spawning is handled by MapGenerationPlugin
        // This plugin handles tile visual updates and interaction
        app.add_plugins(TileInteractionPlugin).add_systems(
            Update,
            (
                systems::update_tile_visuals_system,
                culling_toggle::toggle_culling_system,
            )
                .in_set(WorldSystems::TileUpdate)
                .run_if(in_state(GameState::Playing)),
        );
    }
}
