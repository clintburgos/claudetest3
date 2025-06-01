//! Grid Components - Resources for grid management
//! 
//! This file defines the grid structure and configuration.
//! The GridMap stores references to all tiles by position.
//! GridConfig defines tile dimensions and grid parameters.
//! 
//! # Design Notes
//! - GridMap uses HashMap for sparse storage
//! - Coordinates can be negative to allow centering
//! - Tile size affects isometric projection ratio

use bevy::prelude::*;
use std::collections::HashMap;

/// Configuration for the grid system
#[derive(Resource, Clone, Debug)]
pub struct GridConfig {
    /// Size of each tile in world units
    pub tile_size: f32,
    /// Grid width in tiles
    pub width: i32,
    /// Grid height in tiles
    pub height: i32,
}

impl Default for GridConfig {
    fn default() -> Self {
        Self {
            tile_size: 64.0,
            width: 100,
            height: 100,
        }
    }
}

/// Resource storing the grid structure and tile references
#[derive(Resource, Default)]
pub struct GridMap {
    /// Map from grid position to tile entity
    tiles: HashMap<(i32, i32), Entity>,
    /// Grid dimensions
    width: i32,
    height: i32,
}

impl GridMap {
    /// Create a new empty grid
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: HashMap::new(),
            width,
            height,
        }
    }
    
    /// Insert a tile entity at the given position
    pub fn insert_tile(&mut self, x: i32, y: i32, entity: Entity) {
        self.tiles.insert((x, y), entity);
    }
    
    /// Get the tile entity at the given position
    pub fn get_tile(&self, x: i32, y: i32) -> Option<Entity> {
        self.tiles.get(&(x, y)).copied()
    }
    
    /// Remove a tile from the grid
    pub fn remove_tile(&mut self, x: i32, y: i32) -> Option<Entity> {
        self.tiles.remove(&(x, y))
    }
    
    /// Check if a position is within grid bounds
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }
    
    /// Get grid dimensions
    pub fn dimensions(&self) -> (i32, i32) {
        (self.width, self.height)
    }
    
    /// Get all tile positions
    pub fn positions(&self) -> impl Iterator<Item = &(i32, i32)> {
        self.tiles.keys()
    }
}