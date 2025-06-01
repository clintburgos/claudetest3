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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_config_default() {
        let config = GridConfig::default();
        assert_eq!(config.tile_size, 64.0);
        assert_eq!(config.width, 100);
        assert_eq!(config.height, 100);
    }

    #[test]
    fn test_grid_config_clone() {
        let config = GridConfig {
            tile_size: 32.0,
            width: 50,
            height: 75,
        };
        let cloned = config.clone();
        assert_eq!(cloned.tile_size, 32.0);
        assert_eq!(cloned.width, 50);
        assert_eq!(cloned.height, 75);
    }

    #[test]
    fn test_grid_map_new() {
        let grid = GridMap::new(10, 20);
        assert_eq!(grid.dimensions(), (10, 20));
        assert_eq!(grid.positions().count(), 0);
    }

    #[test]
    fn test_grid_map_default() {
        let grid = GridMap::default();
        assert_eq!(grid.dimensions(), (0, 0));
        assert_eq!(grid.positions().count(), 0);
    }

    #[test]
    fn test_grid_map_insert_and_get_tile() {
        let mut grid = GridMap::new(10, 10);
        let entity = Entity::from_raw(42);
        
        // Insert tile
        grid.insert_tile(5, 5, entity);
        
        // Get existing tile
        assert_eq!(grid.get_tile(5, 5), Some(entity));
        
        // Get non-existing tile
        assert_eq!(grid.get_tile(0, 0), None);
    }

    #[test]
    fn test_grid_map_remove_tile() {
        let mut grid = GridMap::new(10, 10);
        let entity = Entity::from_raw(42);
        
        // Insert and remove tile
        grid.insert_tile(3, 3, entity);
        let removed = grid.remove_tile(3, 3);
        assert_eq!(removed, Some(entity));
        assert_eq!(grid.get_tile(3, 3), None);
        
        // Remove non-existing tile
        let removed_none = grid.remove_tile(3, 3);
        assert_eq!(removed_none, None);
    }

    #[test]
    fn test_grid_map_in_bounds() {
        let grid = GridMap::new(10, 10);
        
        // Valid positions
        assert!(grid.in_bounds(0, 0));
        assert!(grid.in_bounds(9, 9));
        assert!(grid.in_bounds(5, 5));
        
        // Out of bounds positions
        assert!(!grid.in_bounds(-1, 0));
        assert!(!grid.in_bounds(0, -1));
        assert!(!grid.in_bounds(10, 0));
        assert!(!grid.in_bounds(0, 10));
        assert!(!grid.in_bounds(10, 10));
    }

    #[test]
    fn test_grid_map_positions() {
        let mut grid = GridMap::new(10, 10);
        
        // Add multiple tiles
        grid.insert_tile(1, 1, Entity::from_raw(1));
        grid.insert_tile(2, 3, Entity::from_raw(2));
        grid.insert_tile(5, 7, Entity::from_raw(3));
        
        // Check positions
        let positions: Vec<_> = grid.positions().collect();
        assert_eq!(positions.len(), 3);
        assert!(positions.contains(&&(1, 1)));
        assert!(positions.contains(&&(2, 3)));
        assert!(positions.contains(&&(5, 7)));
    }

    #[test]
    fn test_grid_map_overwrite_tile() {
        let mut grid = GridMap::new(10, 10);
        let entity1 = Entity::from_raw(1);
        let entity2 = Entity::from_raw(2);
        
        // Insert first entity
        grid.insert_tile(5, 5, entity1);
        assert_eq!(grid.get_tile(5, 5), Some(entity1));
        
        // Overwrite with second entity
        grid.insert_tile(5, 5, entity2);
        assert_eq!(grid.get_tile(5, 5), Some(entity2));
    }

    #[test]
    fn test_grid_map_edge_cases() {
        let mut grid = GridMap::new(1, 1);
        let entity = Entity::from_raw(1);
        
        // Test single tile grid
        assert!(grid.in_bounds(0, 0));
        assert!(!grid.in_bounds(1, 0));
        assert!(!grid.in_bounds(0, 1));
        
        grid.insert_tile(0, 0, entity);
        assert_eq!(grid.get_tile(0, 0), Some(entity));
        assert_eq!(grid.positions().count(), 1);
    }
}