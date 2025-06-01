//! Biome Rules - Logic for determining biome types from environmental factors
//! 
//! This file contains the rules for biome placement based on:
//! - Elevation (height above sea level)
//! - Moisture (rainfall/humidity)
//! - Adjacency constraints
//! 
//! # Biome Distribution
//! - Water: elevation < water_level
//! - Coast: adjacent to water, low elevation
//! - Desert: low moisture, medium elevation
//! - Plain: medium moisture, medium elevation
//! - Forest: high moisture, medium elevation
//! - Mountain: elevation > mountain_level

use crate::ui::world::tiles::TileBiome;

/// Evaluate which biome should be at given environmental conditions
pub fn evaluate_biome(
    elevation: f64,
    moisture: f64,
    water_level: f64,
    mountain_level: f64,
) -> TileBiome {
    // Water at low elevations
    if elevation < water_level {
        return TileBiome::Water;
    }
    
    // Mountains at high elevations
    if elevation > mountain_level {
        return TileBiome::Mountain;
    }
    
    // For medium elevations, use moisture
    if moisture < 0.3 {
        TileBiome::Desert
    } else if moisture > 0.6 {
        TileBiome::Forest
    } else {
        TileBiome::Plain
    }
}

/// Check if a biome placement is valid given surrounding biomes
pub fn is_valid_placement(
    biome: TileBiome,
    neighbors: &[TileBiome],
) -> bool {
    match biome {
        // Coast must be adjacent to water
        TileBiome::Coast => {
            neighbors.iter().any(|&b| b == TileBiome::Water)
        }
        // Desert shouldn't be directly adjacent to water
        TileBiome::Desert => {
            !neighbors.iter().any(|&b| b == TileBiome::Water)
        }
        // Other biomes have no strict constraints
        _ => true,
    }
}

/// Get valid biome transitions (for smoother terrain)
pub fn valid_transitions(from: TileBiome) -> Vec<TileBiome> {
    match from {
        TileBiome::Water => vec![TileBiome::Water, TileBiome::Coast],
        TileBiome::Coast => vec![TileBiome::Water, TileBiome::Coast, TileBiome::Plain, TileBiome::Forest],
        TileBiome::Plain => vec![TileBiome::Coast, TileBiome::Plain, TileBiome::Forest, TileBiome::Desert, TileBiome::Mountain],
        TileBiome::Forest => vec![TileBiome::Coast, TileBiome::Plain, TileBiome::Forest, TileBiome::Mountain],
        TileBiome::Desert => vec![TileBiome::Plain, TileBiome::Desert, TileBiome::Mountain],
        TileBiome::Mountain => vec![TileBiome::Plain, TileBiome::Forest, TileBiome::Desert, TileBiome::Mountain],
    }
}