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

use crate::constants::generation::biome::*;
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
    if moisture < DESERT_MOISTURE {
        TileBiome::Desert
    } else if moisture > FOREST_MOISTURE {
        TileBiome::Forest
    } else {
        TileBiome::Plain
    }
}

/// Check if a biome placement is valid given surrounding biomes
pub fn is_valid_placement(biome: TileBiome, neighbors: &[TileBiome]) -> bool {
    match biome {
        // Coast must be adjacent to water
        TileBiome::Coast => neighbors.contains(&TileBiome::Water),
        // Desert shouldn't be directly adjacent to water
        TileBiome::Desert => !neighbors.contains(&TileBiome::Water),
        // Other biomes have no strict constraints
        _ => true,
    }
}

/// Get valid biome transitions (for smoother terrain)
pub fn valid_transitions(from: TileBiome) -> Vec<TileBiome> {
    match from {
        TileBiome::Water => vec![TileBiome::Water, TileBiome::Coast],
        TileBiome::Coast => vec![
            TileBiome::Water,
            TileBiome::Coast,
            TileBiome::Plain,
            TileBiome::Forest,
        ],
        TileBiome::Plain => vec![
            TileBiome::Coast,
            TileBiome::Plain,
            TileBiome::Forest,
            TileBiome::Desert,
            TileBiome::Mountain,
        ],
        TileBiome::Forest => vec![
            TileBiome::Coast,
            TileBiome::Plain,
            TileBiome::Forest,
            TileBiome::Mountain,
        ],
        TileBiome::Desert => vec![TileBiome::Plain, TileBiome::Desert, TileBiome::Mountain],
        TileBiome::Mountain => vec![
            TileBiome::Plain,
            TileBiome::Forest,
            TileBiome::Desert,
            TileBiome::Mountain,
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_biome_water() {
        // Low elevation should always be water
        assert_eq!(evaluate_biome(0.1, 0.5, 0.3, 0.7), TileBiome::Water);
        assert_eq!(evaluate_biome(0.2, 0.8, 0.3, 0.7), TileBiome::Water);
        assert_eq!(evaluate_biome(0.29, 0.1, 0.3, 0.7), TileBiome::Water);
    }

    #[test]
    fn test_evaluate_biome_mountain() {
        // High elevation should always be mountain
        assert_eq!(evaluate_biome(0.8, 0.5, 0.3, 0.7), TileBiome::Mountain);
        assert_eq!(evaluate_biome(0.9, 0.2, 0.3, 0.7), TileBiome::Mountain);
        assert_eq!(evaluate_biome(1.0, 0.8, 0.3, 0.7), TileBiome::Mountain);
    }

    #[test]
    fn test_evaluate_biome_desert() {
        // Medium elevation with low moisture
        assert_eq!(evaluate_biome(0.5, 0.1, 0.3, 0.7), TileBiome::Desert);
        assert_eq!(evaluate_biome(0.6, 0.2, 0.3, 0.7), TileBiome::Desert);
        assert_eq!(evaluate_biome(0.4, 0.29, 0.3, 0.7), TileBiome::Desert);
    }

    #[test]
    fn test_evaluate_biome_forest() {
        // Medium elevation with high moisture
        assert_eq!(evaluate_biome(0.5, 0.8, 0.3, 0.7), TileBiome::Forest);
        assert_eq!(evaluate_biome(0.6, 0.71, 0.3, 0.7), TileBiome::Forest);
        assert_eq!(evaluate_biome(0.4, 0.9, 0.3, 0.7), TileBiome::Forest);
    }

    #[test]
    fn test_evaluate_biome_plain() {
        // Medium elevation with medium moisture
        assert_eq!(evaluate_biome(0.5, 0.5, 0.3, 0.7), TileBiome::Plain);
        assert_eq!(evaluate_biome(0.6, 0.4, 0.3, 0.7), TileBiome::Plain);
        assert_eq!(evaluate_biome(0.4, 0.6, 0.3, 0.7), TileBiome::Plain);
    }

    #[test]
    fn test_evaluate_biome_edge_cases() {
        // Test exact threshold values
        assert_eq!(evaluate_biome(0.3, 0.5, 0.3, 0.7), TileBiome::Plain);
        assert_eq!(evaluate_biome(0.7, 0.5, 0.3, 0.7), TileBiome::Plain);
        assert_eq!(evaluate_biome(0.5, 0.3, 0.3, 0.7), TileBiome::Plain);
        assert_eq!(evaluate_biome(0.5, 0.7, 0.3, 0.7), TileBiome::Plain);

        // Test just beyond thresholds
        assert_eq!(evaluate_biome(0.29, 0.5, 0.3, 0.7), TileBiome::Water);
        assert_eq!(evaluate_biome(0.71, 0.5, 0.3, 0.7), TileBiome::Mountain);
        assert_eq!(evaluate_biome(0.5, 0.29, 0.3, 0.7), TileBiome::Desert);
        assert_eq!(evaluate_biome(0.5, 0.71, 0.3, 0.7), TileBiome::Forest);
    }

    #[test]
    fn test_evaluate_biome_custom_thresholds() {
        // Test with different threshold values
        assert_eq!(evaluate_biome(0.15, 0.5, 0.2, 0.8), TileBiome::Water);
        assert_eq!(evaluate_biome(0.25, 0.5, 0.2, 0.8), TileBiome::Plain);
        assert_eq!(evaluate_biome(0.85, 0.5, 0.2, 0.8), TileBiome::Mountain);
        assert_eq!(evaluate_biome(0.75, 0.5, 0.2, 0.8), TileBiome::Plain);
    }

    #[test]
    fn test_is_valid_placement_coast() {
        // Coast must be adjacent to water
        assert!(is_valid_placement(
            TileBiome::Coast,
            &[TileBiome::Water, TileBiome::Plain]
        ));
        assert!(is_valid_placement(
            TileBiome::Coast,
            &[TileBiome::Plain, TileBiome::Water, TileBiome::Forest]
        ));
        assert!(!is_valid_placement(
            TileBiome::Coast,
            &[TileBiome::Plain, TileBiome::Forest]
        ));
        assert!(!is_valid_placement(TileBiome::Coast, &[]));
    }

    #[test]
    fn test_is_valid_placement_desert() {
        // Desert should not be adjacent to water
        assert!(!is_valid_placement(
            TileBiome::Desert,
            &[TileBiome::Water, TileBiome::Plain]
        ));
        assert!(!is_valid_placement(
            TileBiome::Desert,
            &[TileBiome::Coast, TileBiome::Water]
        ));
        assert!(is_valid_placement(
            TileBiome::Desert,
            &[TileBiome::Plain, TileBiome::Mountain]
        ));
        assert!(is_valid_placement(TileBiome::Desert, &[]));
    }

    #[test]
    fn test_is_valid_placement_other_biomes() {
        // Other biomes have no constraints
        assert!(is_valid_placement(
            TileBiome::Plain,
            &[TileBiome::Water, TileBiome::Desert]
        ));
        assert!(is_valid_placement(
            TileBiome::Forest,
            &[TileBiome::Mountain, TileBiome::Coast]
        ));
        assert!(is_valid_placement(TileBiome::Mountain, &[]));
        assert!(is_valid_placement(TileBiome::Water, &[TileBiome::Desert]));
    }

    #[test]
    fn test_valid_transitions_water() {
        let transitions = valid_transitions(TileBiome::Water);
        assert!(transitions.contains(&TileBiome::Water));
        assert!(transitions.contains(&TileBiome::Coast));
        assert_eq!(transitions.len(), 2);
    }

    #[test]
    fn test_valid_transitions_coast() {
        let transitions = valid_transitions(TileBiome::Coast);
        assert!(transitions.contains(&TileBiome::Water));
        assert!(transitions.contains(&TileBiome::Plain));
        assert!(transitions.contains(&TileBiome::Forest));
        assert_eq!(transitions.len(), 4);
    }

    #[test]
    fn test_valid_transitions_plain() {
        let transitions = valid_transitions(TileBiome::Plain);
        assert!(transitions.contains(&TileBiome::Coast));
        assert!(transitions.contains(&TileBiome::Forest));
        assert!(transitions.contains(&TileBiome::Desert));
        assert!(transitions.contains(&TileBiome::Mountain));
        assert_eq!(transitions.len(), 5);
    }

    #[test]
    fn test_valid_transitions_forest() {
        let transitions = valid_transitions(TileBiome::Forest);
        assert!(transitions.contains(&TileBiome::Coast));
        assert!(transitions.contains(&TileBiome::Plain));
        assert!(transitions.contains(&TileBiome::Mountain));
        assert_eq!(transitions.len(), 4);
    }

    #[test]
    fn test_valid_transitions_desert() {
        let transitions = valid_transitions(TileBiome::Desert);
        assert!(transitions.contains(&TileBiome::Plain));
        assert!(transitions.contains(&TileBiome::Mountain));
        assert_eq!(transitions.len(), 3);
    }

    #[test]
    fn test_valid_transitions_mountain() {
        let transitions = valid_transitions(TileBiome::Mountain);
        assert!(transitions.contains(&TileBiome::Plain));
        assert!(transitions.contains(&TileBiome::Forest));
        assert!(transitions.contains(&TileBiome::Desert));
        assert!(transitions.contains(&TileBiome::Mountain));
        assert_eq!(transitions.len(), 4);
    }
}
