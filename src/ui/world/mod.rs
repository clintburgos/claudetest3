//! World Module - Isometric tile-based world system
//! 
//! This module provides a complete isometric world implementation including:
//! - Procedurally generated tile maps with multiple biomes
//! - Isometric projection and coordinate systems
//! - Camera controls for navigation
//! - Modular plugin architecture
//! 
//! # Architecture
//! 
//! The world system is composed of four independent subsystems:
//! - `tiles`: Individual tile entities and rendering
//! - `grid`: Spatial organization and coordinate math
//! - `generation`: Procedural map creation algorithms
//! - `camera`: View controls and constraints
//! 
//! # Usage
//! 
//! ```rust,no_run
//! # use bevy::prelude::*;
//! # use claudetest3::ui::world::WorldPlugin;
//! # let mut app = App::new();
//! app.add_plugins(WorldPlugin);
//! ```

use bevy::prelude::*;

pub mod tiles;
pub mod grid;
pub mod generation;
pub mod camera;

// Re-export commonly used items
pub use tiles::{Tile, TilePosition, TileBiome};
pub use grid::{GridMap, GridConfig};
pub use camera::IsometricCamera;

/// Main plugin that registers all world subsystems
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(tiles::TilePlugin)
            .add_plugins(grid::GridPlugin)
            .add_plugins(generation::MapGenerationPlugin)
            .add_plugins(camera::IsometricCameraPlugin);
    }
}