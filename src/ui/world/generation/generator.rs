//! Map Generator - Trait and implementations for map generation algorithms
//!
//! This file defines the MapGenerator trait and provides a default
//! implementation using Perlin noise for realistic terrain generation.
//!
//! # Design Notes
//! - Trait allows for different generation algorithms
//! - Height and moisture maps create varied biomes
//! - Border tiles are always water for island effect

use super::biome_rules::evaluate_biome;
use crate::constants::generation::*;
use crate::constants::grid::WATER_BORDER_SIZE;
use crate::ui::world::tiles::TileBiome;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

/// Trait for map generation algorithms
pub trait MapGenerator {
    /// Generate a map of biomes for the given dimensions
    fn generate(&self, width: i32, height: i32) -> Vec<Vec<TileBiome>>;
}

/// Default map generator using Perlin noise
pub struct DefaultMapGenerator {
    /// Seed for noise generation
    seed: u32,
    /// Scale factor for noise (larger = more variation)
    scale: f64,
    /// Water level threshold
    water_level: f64,
    /// Mountain level threshold
    mountain_level: f64,
}

impl Default for DefaultMapGenerator {
    fn default() -> Self {
        Self {
            seed: DEFAULT_SEED as u32,
            scale: DEFAULT_NOISE_SCALE,
            water_level: WATER_LEVEL,
            mountain_level: MOUNTAIN_LEVEL,
        }
    }
}

impl DefaultMapGenerator {
    /// Create a new generator with custom parameters
    pub fn new(seed: u32) -> Self {
        Self {
            seed,
            ..Default::default()
        }
    }
}

impl MapGenerator for DefaultMapGenerator {
    fn generate(&self, width: i32, height: i32) -> Vec<Vec<TileBiome>> {
        let elevation_noise = Perlin::new(self.seed);
        let moisture_noise = Perlin::new(self.seed + 1);

        let mut map = vec![vec![TileBiome::Water; width as usize]; height as usize];

        // Border size for water
        let border = WATER_BORDER_SIZE;

        for y in 0..height {
            for x in 0..width {
                // Force water at borders
                if x < border || x >= width - border || y < border || y >= height - border {
                    map[y as usize][x as usize] = TileBiome::Water;
                    continue;
                }

                // Sample noise for elevation and moisture
                let nx = x as f64 * self.scale;
                let ny = y as f64 * self.scale;

                let elevation = (elevation_noise.get([nx, ny]) + 1.0) / 2.0;
                let moisture = (moisture_noise.get([
                    nx * MOISTURE_SCALE_MULTIPLIER,
                    ny * MOISTURE_SCALE_MULTIPLIER,
                ]) + 1.0)
                    / 2.0;

                // Add distance from center factor for island shape
                let center_x = width as f64 / 2.0;
                let center_y = height as f64 / 2.0;
                let distance =
                    ((x as f64 - center_x).powi(2) + (y as f64 - center_y).powi(2)).sqrt();
                let max_distance =
                    (center_x.min(center_y) - border as f64) * ISLAND_DISTANCE_FACTOR;
                let distance_factor = 1.0 - (distance / max_distance).min(1.0);

                let adjusted_elevation = elevation * distance_factor;

                // Determine biome
                let biome = evaluate_biome(
                    adjusted_elevation,
                    moisture,
                    self.water_level,
                    self.mountain_level,
                );
                map[y as usize][x as usize] = biome;
            }
        }

        // Post-process to add coasts
        let mut final_map = map.clone();
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if map[y as usize][x as usize] != TileBiome::Water {
                    // Check if adjacent to water
                    let mut water_adjacent = false;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let ny = (y + dy) as usize;
                            let nx = (x + dx) as usize;
                            if map[ny][nx] == TileBiome::Water {
                                water_adjacent = true;
                                break;
                            }
                        }
                        if water_adjacent {
                            break;
                        }
                    }

                    // Convert to coast if next to water and low elevation
                    if water_adjacent && map[y as usize][x as usize] == TileBiome::Plain {
                        final_map[y as usize][x as usize] = TileBiome::Coast;
                    }
                }
            }
        }

        final_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_map_generator_new() {
        let gen = DefaultMapGenerator::new(123);
        assert_eq!(gen.seed, 123);
        assert_eq!(gen.scale, 0.05);
        assert_eq!(gen.water_level, 0.3);
        assert_eq!(gen.mountain_level, 0.7);
    }

    #[test]
    fn test_default_map_generator_default() {
        let gen = DefaultMapGenerator::default();
        assert_eq!(gen.seed, 42);
        assert_eq!(gen.scale, 0.05);
        assert_eq!(gen.water_level, 0.3);
        assert_eq!(gen.mountain_level, 0.7);
    }

    #[test]
    fn test_generate_map_dimensions() {
        let gen = DefaultMapGenerator::default();
        let map = gen.generate(20, 30);

        assert_eq!(map.len(), 30);
        for row in &map {
            assert_eq!(row.len(), 20);
        }
    }

    #[test]
    fn test_generate_map_borders_are_water() {
        let gen = DefaultMapGenerator::default();
        let map = gen.generate(50, 50);
        let border = 5;

        // Check top and bottom borders
        for y in 0..border {
            for x in 0..50 {
                assert_eq!(
                    map[y][x],
                    TileBiome::Water,
                    "Top border at ({}, {}) should be water",
                    x,
                    y
                );
                assert_eq!(
                    map[50 - 1 - y][x],
                    TileBiome::Water,
                    "Bottom border at ({}, {}) should be water",
                    x,
                    50 - 1 - y
                );
            }
        }

        // Check left and right borders
        for y in 0..50 {
            for x in 0..border {
                assert_eq!(
                    map[y][x],
                    TileBiome::Water,
                    "Left border at ({}, {}) should be water",
                    x,
                    y
                );
                assert_eq!(
                    map[y][50 - 1 - x],
                    TileBiome::Water,
                    "Right border at ({}, {}) should be water",
                    50 - 1 - x,
                    y
                );
            }
        }
    }

    #[test]
    fn test_generate_map_contains_various_biomes() {
        let gen = DefaultMapGenerator::default();
        let map = gen.generate(100, 100);

        let mut biome_counts = std::collections::HashMap::new();
        for row in &map {
            for biome in row {
                *biome_counts.entry(biome).or_insert(0) += 1;
            }
        }

        // Should have at least some water (borders guarantee this)
        assert!(biome_counts.get(&TileBiome::Water).unwrap_or(&0) > &0);

        // Should have some variety in biomes (at least 3 different types)
        assert!(
            biome_counts.len() >= 3,
            "Map should contain at least 3 different biome types"
        );
    }

    #[test]
    fn test_generate_map_coasts_adjacent_to_water() {
        let gen = DefaultMapGenerator::default();
        let map = gen.generate(50, 50);

        // Check that all coast tiles are adjacent to at least one water tile
        for y in 1..49 {
            for x in 1..49 {
                if map[y][x] == TileBiome::Coast {
                    let mut has_water_neighbor = false;
                    for dy in -1i32..=1 {
                        for dx in -1i32..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let ny = (y as i32 + dy) as usize;
                            let nx = (x as i32 + dx) as usize;
                            if map[ny][nx] == TileBiome::Water {
                                has_water_neighbor = true;
                                break;
                            }
                        }
                        if has_water_neighbor {
                            break;
                        }
                    }
                    assert!(
                        has_water_neighbor,
                        "Coast at ({}, {}) should be adjacent to water",
                        x, y
                    );
                }
            }
        }
    }

    #[test]
    fn test_generate_map_deterministic() {
        let gen1 = DefaultMapGenerator::new(999);
        let gen2 = DefaultMapGenerator::new(999);

        let map1 = gen1.generate(30, 30);
        let map2 = gen2.generate(30, 30);

        // Same seed should produce same map
        for y in 0..30 {
            for x in 0..30 {
                assert_eq!(
                    map1[y][x], map2[y][x],
                    "Maps should be identical at ({}, {})",
                    x, y
                );
            }
        }
    }

    #[test]
    fn test_generate_map_different_seeds() {
        let gen1 = DefaultMapGenerator::new(1);
        let gen2 = DefaultMapGenerator::new(2);

        let map1 = gen1.generate(30, 30);
        let map2 = gen2.generate(30, 30);

        // Different seeds should produce different maps (except forced borders)
        let mut differences = 0;
        for y in 10..20 {
            // Check center area to avoid borders
            for x in 10..20 {
                if map1[y][x] != map2[y][x] {
                    differences += 1;
                }
            }
        }

        assert!(
            differences > 0,
            "Different seeds should produce different maps"
        );
    }

    #[test]
    fn test_generate_small_map() {
        let gen = DefaultMapGenerator::default();
        let map = gen.generate(10, 10);

        assert_eq!(map.len(), 10);
        assert_eq!(map[0].len(), 10);

        // Even small maps should be all water due to border constraint
        for y in 0..10 {
            for x in 0..10 {
                assert_eq!(map[y][x], TileBiome::Water);
            }
        }
    }
}
