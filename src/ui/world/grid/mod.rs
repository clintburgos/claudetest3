//! Grid Module - Spatial organization and coordinate systems
//! 
//! This module manages the grid structure that organizes tiles in space.
//! It provides coordinate conversions between grid, world, and screen space.
//! 
//! # Responsibilities
//! - Store grid configuration (size, tile dimensions)
//! - Maintain tile entity references by position
//! - Convert between coordinate systems
//! - Define grid boundaries
//! 
//! # Coordinate Systems
//! - Grid: Integer tile positions (x, y, z)
//! - World: Bevy world coordinates
//! - Isometric: Screen-space isometric projection

use bevy::prelude::*;

pub mod components;
pub mod coordinates;

pub use components::{GridMap, GridConfig};
pub use coordinates::{grid_to_world, world_to_grid, grid_to_isometric};

/// Plugin that manages grid resources and systems
pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        // Initialize with default config
        app.insert_resource(GridConfig::default())
           .insert_resource(GridMap::new(100, 100)); // Default 100x100 grid
    }
}