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
            seed: 42,
            scale: 0.05,
            water_level: 0.3,
            mountain_level: 0.7,
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
        let border = 5;

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
                let moisture = (moisture_noise.get([nx * 1.5, ny * 1.5]) + 1.0) / 2.0;

                // Add distance from center factor for island shape
                let center_x = width as f64 / 2.0;
                let center_y = height as f64 / 2.0;
                let distance =
                    ((x as f64 - center_x).powi(2) + (y as f64 - center_y).powi(2)).sqrt();
                let max_distance = (center_x.min(center_y) - border as f64) * 0.8;
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
