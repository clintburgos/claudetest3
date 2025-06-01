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

use bevy::prelude::*;

pub mod biome_rules;
pub mod generator;
pub mod systems;

pub use generator::{DefaultMapGenerator, MapGenerator};

/// Plugin that handles map generation on startup
pub struct MapGenerationPlugin;

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::generate_map_system);
    }
}
