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
            TileBiome::Plain => Color::srgb(0.56, 0.93, 0.56), // Light Green
            TileBiome::Forest => Color::srgb(0.13, 0.55, 0.13), // Dark Green
            TileBiome::Coast => Color::srgb(0.96, 0.64, 0.38), // Sandy
            TileBiome::Water => Color::srgb(0.27, 0.51, 0.71), // Blue
            TileBiome::Desert => Color::srgb(0.94, 0.90, 0.55), // Yellow
            TileBiome::Mountain => Color::srgb(0.50, 0.50, 0.50), // Gray
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tile_position_new() {
        let pos = TilePosition::new(10, 20, 5);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 20);
        assert_eq!(pos.z, 5);
    }

    #[test]
    fn test_tile_position_ground() {
        let pos = TilePosition::ground(15, 25);
        assert_eq!(pos.x, 15);
        assert_eq!(pos.y, 25);
        assert_eq!(pos.z, 0);
    }

    #[test]
    fn test_tile_position_negative_coordinates() {
        let pos = TilePosition::new(-5, -10, -2);
        assert_eq!(pos.x, -5);
        assert_eq!(pos.y, -10);
        assert_eq!(pos.z, -2);
    }

    #[test]
    fn test_tile_position_equality() {
        let pos1 = TilePosition::new(1, 2, 3);
        let pos2 = TilePosition::new(1, 2, 3);
        let pos3 = TilePosition::new(1, 2, 4);

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_tile_position_clone() {
        let pos1 = TilePosition::new(7, 8, 9);
        let pos2 = pos1.clone();
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_tile_position_copy() {
        let pos1 = TilePosition::new(3, 4, 5);
        let pos2 = pos1; // Copy trait
        assert_eq!(pos1, pos2);
    }

    #[test]
    fn test_tile_position_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(TilePosition::new(1, 2, 3));
        set.insert(TilePosition::new(1, 2, 3)); // Duplicate
        set.insert(TilePosition::new(4, 5, 6));

        assert_eq!(set.len(), 2); // Only 2 unique positions
        assert!(set.contains(&TilePosition::new(1, 2, 3)));
        assert!(set.contains(&TilePosition::new(4, 5, 6)));
    }

    #[test]
    fn test_tile_biome_colors() {
        // Test each biome has a unique color
        let colors = [
            TileBiome::Plain.color(),
            TileBiome::Forest.color(),
            TileBiome::Coast.color(),
            TileBiome::Water.color(),
            TileBiome::Desert.color(),
            TileBiome::Mountain.color(),
        ];

        // Verify all colors are different
        for i in 0..colors.len() {
            for j in (i + 1)..colors.len() {
                assert_ne!(
                    colors[i], colors[j],
                    "Colors at {} and {} are the same",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_tile_biome_color_values() {
        // Verify specific color values match design
        assert_eq!(TileBiome::Plain.color(), Color::srgb(0.56, 0.93, 0.56));
        assert_eq!(TileBiome::Forest.color(), Color::srgb(0.13, 0.55, 0.13));
        assert_eq!(TileBiome::Coast.color(), Color::srgb(0.96, 0.64, 0.38));
        assert_eq!(TileBiome::Water.color(), Color::srgb(0.27, 0.51, 0.71));
        assert_eq!(TileBiome::Desert.color(), Color::srgb(0.94, 0.90, 0.55));
        assert_eq!(TileBiome::Mountain.color(), Color::srgb(0.50, 0.50, 0.50));
    }

    #[test]
    fn test_tile_biome_equality() {
        assert_eq!(TileBiome::Plain, TileBiome::Plain);
        assert_ne!(TileBiome::Plain, TileBiome::Forest);
    }

    #[test]
    fn test_tile_biome_clone() {
        let biome1 = TileBiome::Water;
        let biome2 = biome1.clone();
        assert_eq!(biome1, biome2);
    }

    #[test]
    fn test_tile_biome_copy() {
        let biome1 = TileBiome::Desert;
        let biome2 = biome1; // Copy trait
        assert_eq!(biome1, biome2);
    }

    #[test]
    fn test_tile_default() {
        // Test that Tile component can be default constructed
        let _tile = Tile::default();
    }

    #[test]
    fn test_tile_position_debug() {
        let pos = TilePosition::new(1, 2, 3);
        let debug_str = format!("{:?}", pos);
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("3"));
    }

    #[test]
    fn test_tile_biome_debug() {
        let biome = TileBiome::Mountain;
        let debug_str = format!("{:?}", biome);
        assert_eq!(debug_str, "Mountain");
    }
}
