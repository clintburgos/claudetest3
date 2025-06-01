//! Tile Components - Data structures for tile entities
//! 
//! This file defines all components used by tile entities.
//! Each tile has a position in grid space and a biome type.
//! 
//! # Design Notes
//! - TilePosition uses i32 to allow negative coordinates
//! - Z coordinate is included for future elevation features
//! - Biome enum is kept simple with only 6 types as specified

use bevy::prelude::*;

/// Marker component identifying an entity as a tile
#[derive(Component, Default)]
pub struct Tile;

/// Grid position of a tile in the world
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TilePosition {
    /// X coordinate (West to East)
    pub x: i32,
    /// Y coordinate (North to South)
    pub y: i32,
    /// Z coordinate (elevation - reserved for future use)
    pub z: i32,
}

impl TilePosition {
    /// Create a new tile position
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    
    /// Create a position at ground level (z = 0)
    pub fn ground(x: i32, y: i32) -> Self {
        Self { x, y, z: 0 }
    }
}

/// Biome type determining tile appearance and properties
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileBiome {
    /// Grassland - basic terrain
    Plain,
    /// Dense forest
    Forest,
    /// Sandy beach adjacent to water
    Coast,
    /// Ocean or lake
    Water,
    /// Arid desert
    Desert,
    /// Rocky mountains
    Mountain,
}

impl TileBiome {
    /// Get the color representation for this biome
    pub fn color(&self) -> Color {
        match self {
            TileBiome::Plain => Color::srgb(0.56, 0.93, 0.56),    // Light Green
            TileBiome::Forest => Color::srgb(0.13, 0.55, 0.13),   // Dark Green
            TileBiome::Coast => Color::srgb(0.96, 0.64, 0.38),    // Sandy
            TileBiome::Water => Color::srgb(0.27, 0.51, 0.71),    // Blue
            TileBiome::Desert => Color::srgb(0.94, 0.90, 0.55),   // Yellow
            TileBiome::Mountain => Color::srgb(0.50, 0.50, 0.50), // Gray
        }
    }
}