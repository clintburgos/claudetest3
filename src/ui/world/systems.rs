//! World System Sets - Ordering and grouping for world systems
//!
//! This file defines system sets for proper ordering of world-related systems.
//! These sets ensure that grid initialization happens before tile spawning,
//! and tile spawning happens before visual updates.

use bevy::prelude::*;

/// System sets for organizing world systems execution order
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorldSystems {
    /// Grid initialization and setup
    GridInit,
    /// Tile spawning and generation
    TileSpawn,
    /// Tile visual updates and rendering
    TileUpdate,
    /// Camera setup and controls
    CameraSetup,
    /// Camera movement and updates
    CameraUpdate,
    /// Map generation systems
    MapGeneration,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_systems_derive_traits() {
        // Test that WorldSystems implements required traits
        let system1 = WorldSystems::GridInit;
        let system2 = WorldSystems::GridInit;
        let system3 = WorldSystems::TileSpawn;

        // Test PartialEq
        assert_eq!(system1, system2);
        assert_ne!(system1, system3);

        // Test Clone
        let cloned = system1.clone();
        assert_eq!(system1, cloned);

        // Test Debug
        let debug_str = format!("{:?}", system1);
        assert_eq!(debug_str, "GridInit");
    }

    #[test]
    fn test_world_systems_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(WorldSystems::GridInit);
        set.insert(WorldSystems::TileSpawn);
        set.insert(WorldSystems::GridInit); // Duplicate

        // Should only have 2 unique items
        assert_eq!(set.len(), 2);
        assert!(set.contains(&WorldSystems::GridInit));
        assert!(set.contains(&WorldSystems::TileSpawn));
    }
}
