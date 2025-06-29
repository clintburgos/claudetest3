//! Generation Module - Procedural map creation
//!
//! This module handles procedural generation of the tile map.
//! It uses noise functions and biome rules to create realistic worlds.
//!
//! # Responsibilities
//! - Generate elevation and moisture maps
//! - Apply biome placement rules
//! - Ensure map has water borders
//! - Create varied but realistic terrain
//!
//! # Algorithm
//! 1. Generate height map using Perlin noise
//! 2. Generate moisture map using different noise
//! 3. Determine biomes from height + moisture
//! 4. Apply constraints for realistic placement
//! 5. Spawn tile entities

use crate::game::GameState;
use bevy::prelude::*;

pub mod biome_rules;
pub mod generator;
pub mod systems;

pub use generator::{DefaultMapGenerator, MapGenerator};

/// Plugin that handles map generation on startup
pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        use crate::ui::world::tiles::{
            systems::init_tile_meshes, view_culling_system, SpawnedTiles, ViewCullingConfig,
        };
        use crate::ui::world::WorldSystems;

        // Add view culling resources
        app.insert_resource(ViewCullingConfig::default())
            .insert_resource(SpawnedTiles::default());

        app.add_systems(
            OnEnter(GameState::Playing),
            (
                init_tile_meshes.in_set(WorldSystems::GridInit),
                systems::generate_map_system
                    .in_set(WorldSystems::MapGeneration)
                    .after(WorldSystems::GridInit),
            ),
        )
        .add_systems(
            Update,
            (
                view_culling_system,
                crate::ui::world::tiles::spawn_all_tiles::spawn_all_tiles_system,
            )
                .chain()
                .in_set(WorldSystems::TileUpdate)
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnExit(GameState::Playing), systems::cleanup_map_system);
    }
}
