//! View Culling System - Manages tile visibility based on camera view
//!
//! This system optimizes performance by only spawning tiles that are visible
//! to the camera, and despawning tiles that move out of view.
//!
//! # Design Notes
//! - Uses a ViewCullingConfig resource to control culling behavior
//! - Maintains a buffer zone around the visible area to reduce spawning/despawning
//! - Tracks spawned tiles to avoid duplicate spawning
//! - Works with the existing GridMap to manage tile references

use super::components::{Tile, TileBiome, TilePosition};
use super::systems::{spawn_tile, TileMeshes};
use crate::constants::culling::*;
use crate::ui::world::camera::components::IsometricCamera;
use crate::ui::world::grid::{GridConfig, GridMap};
use bevy::prelude::*;
use std::collections::HashSet;

/// Configuration for view culling behavior
#[derive(Resource)]
pub struct ViewCullingConfig {
    /// Number of tiles to spawn beyond the visible area
    pub buffer_tiles: i32,
    /// Maximum number of tiles to spawn per frame
    pub tiles_per_frame: usize,
    /// Whether culling is enabled
    pub enabled: bool,
}

impl Default for ViewCullingConfig {
    fn default() -> Self {
        Self {
            buffer_tiles: DEFAULT_BUFFER_TILES,
            tiles_per_frame: DEFAULT_TILES_PER_FRAME,
            enabled: true,
        }
    }
}

/// Resource tracking which tiles are currently spawned
#[derive(Resource, Default)]
pub struct SpawnedTiles {
    positions: HashSet<(i32, i32)>,
}

impl SpawnedTiles {
    pub fn insert(&mut self, x: i32, y: i32) {
        self.positions.insert((x, y));
    }

    pub fn remove(&mut self, x: i32, y: i32) {
        self.positions.remove(&(x, y));
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        self.positions.contains(&(x, y))
    }

    pub fn clear(&mut self) {
        self.positions.clear();
    }

    pub fn count(&self) -> usize {
        self.positions.len()
    }
}


/// Calculate the visible tile bounds based on camera position and zoom
fn calculate_visible_bounds(
    camera_transform: &Transform,
    camera_zoom: f32,
    window: &Window,
    grid_config: &GridConfig,
    base_buffer: i32,
) -> (i32, i32, i32, i32) {
    // Use the proper isometric culling calculation
    let window_size = Vec2::new(window.width(), window.height());
    let (min_x, min_y, max_x, max_y) = super::isometric_culling::calculate_isometric_visible_tiles(
        camera_transform.translation,
        window_size,
        camera_zoom,
        grid_config.tile_size,
        grid_config.width,
        grid_config.height,
        base_buffer,
    );

    // Debug logging for extreme zoom levels
    if !(0.2..=5.0).contains(&camera_zoom) {
        info!(
            "View Culling: scale={:.3}, camera=({:.0},{:.0}), visible_tiles=({},{})-({},{})",
            camera_zoom,
            camera_transform.translation.x, camera_transform.translation.y,
            min_x, min_y, max_x, max_y
        );
    }

    (min_x, min_y, max_x, max_y)
}

/// System that spawns tiles within view and despawns tiles outside view
pub fn view_culling_system(
    mut commands: Commands,
    culling_config: Res<ViewCullingConfig>,
    grid_config: Res<GridConfig>,
    mut grid_map: ResMut<GridMap>,
    mut spawned_tiles: ResMut<SpawnedTiles>,
    tile_meshes: Res<TileMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    camera_query: Query<
        (&Transform, &crate::ui::world::camera::CameraState),
        With<IsometricCamera>,
    >,
    tile_query: Query<(Entity, &TilePosition), With<Tile>>,
    windows: Query<&Window>,
    mut biome_cache: Local<Vec<Vec<TileBiome>>>,
) {
    // Skip if culling is disabled
    if !culling_config.enabled {
        return;
    }

    // Get camera and window
    let Ok((camera_transform, camera_state)) = camera_query.single() else {
        return;
    };

    let Ok(window) = windows.single() else {
        return;
    };

    // Initialize biome cache if needed
    if biome_cache.is_empty() {
        // Generate biomes for the entire map once
        use crate::ui::world::generation::generator::{DefaultMapGenerator, MapGenerator};
        let generator = DefaultMapGenerator::default();
        *biome_cache = generator.generate(grid_config.width, grid_config.height);
        info!("Initialized biome cache: {} rows, {} cols per row", 
            biome_cache.len(), 
            if biome_cache.is_empty() { 0 } else { biome_cache[0].len() }
        );
    }

    // Calculate visible bounds
    let (min_x, min_y, max_x, max_y) = calculate_visible_bounds(
        camera_transform,
        camera_state.zoom,
        window,
        &grid_config,
        culling_config.buffer_tiles,
    );

    // Debug log for edge positions
    if camera_transform.translation.x.abs() > 2000.0
        || camera_transform.translation.y.abs() > 2000.0
    {
        info!(
            "Edge position culling: cam_pos=({:.0},{:.0}), zoom={:.2}, visible_tiles=({},{},{},{})",
            camera_transform.translation.x,
            camera_transform.translation.y,
            camera_state.zoom,
            min_x,
            min_y,
            max_x,
            max_y
        );
    }

    // Log only at extreme zoom levels for debugging
    if !(0.15..=5.0).contains(&camera_state.zoom) {
        info!(
            "Culling: zoom={:.3}, visible_tiles=({}-{}, {}-{}), grid_size={}x{}, tiles_to_spawn={}, total_spawned={}",
            camera_state.zoom,
            min_x, max_x,
            min_y, max_y,
            grid_config.width, grid_config.height,
            (max_x - min_x + 1) * (max_y - min_y + 1),
            spawned_tiles.count()
        );
    }

    // Collect tiles to despawn (outside visible bounds with extra margin)
    let mut tiles_to_despawn = Vec::new();
    // Add extra margin to avoid despawning tiles too aggressively
    let despawn_margin = 5;
    for (entity, position) in tile_query.iter() {
        if position.x < min_x - despawn_margin || position.x > max_x + despawn_margin || 
           position.y < min_y - despawn_margin || position.y > max_y + despawn_margin {
            tiles_to_despawn.push((entity, position.x, position.y));
        }
    }

    // Despawn tiles outside view
    let despawn_count = tiles_to_despawn.len();
    if despawn_count > 0 {
        if despawn_count < 10 {
            info!("Despawning {} tiles", despawn_count);
            for (entity, x, y) in &tiles_to_despawn {
                info!("  Despawning tile at ({}, {})", x, y);
            }
        } else if despawn_count > 100 {
            warn!("Despawning {} tiles - this seems excessive!", despawn_count);
        }
    }
    for (entity, x, y) in tiles_to_despawn {
        commands.entity(entity).despawn();
        grid_map.remove_tile(x, y);
        spawned_tiles.remove(x, y);
    }

    // Collect tiles that need spawning, prioritizing from center outward
    let center_x = (min_x + max_x) / 2;
    let center_y = (min_y + max_y) / 2;
    
    let mut tiles_to_spawn = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if !spawned_tiles.contains(x, y) {
                // Calculate distance from center for prioritization
                let dist_sq = ((x - center_x) * (x - center_x) + (y - center_y) * (y - center_y)) as f32;
                tiles_to_spawn.push((dist_sq, x, y));
            }
        }
    }
    
    // Sort by distance (closest first)
    tiles_to_spawn.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    
    // Spawn tiles within view (limited per frame)
    let mut tiles_spawned = 0;
    
    for (_dist, x, y) in tiles_to_spawn.iter() {
        // Skip if we've hit the per-frame limit
        if tiles_spawned >= culling_config.tiles_per_frame {
            break;
        }

        // Get biome from cache - check bounds first
        if *x < 0 || *y < 0 || (*x as usize) >= grid_config.width as usize || (*y as usize) >= grid_config.height as usize {
            warn!("Attempting to spawn tile outside grid bounds: ({}, {})", x, y);
            continue;
        }
        
        // Additional safety check for biome cache bounds
        if (*y as usize) >= biome_cache.len() || (*x as usize) >= biome_cache[*y as usize].len() {
            error!("Biome cache index out of bounds: tile ({},{}) but cache is {}x{}", 
                x, y, 
                if biome_cache.is_empty() { 0 } else { biome_cache[0].len() },
                biome_cache.len()
            );
            continue;
        }
        
        let biome = biome_cache[*y as usize][*x as usize];
        let position = TilePosition::ground(*x, *y);

        // Spawn tile
        let entity = spawn_tile(
            &mut commands,
            position,
            biome,
            grid_config.tile_size,
            tile_meshes.diamond.clone(),
            &mut materials,
        );

        // Update tracking
        grid_map.insert_tile(*x, *y, entity);
        spawned_tiles.insert(*x, *y);
        tiles_spawned += 1;
        
        // Debug: Log first few tile spawns
        if tiles_spawned <= 3 {
            let world_pos = crate::ui::world::grid::coordinates::grid_to_world(*x, *y, 0, grid_config.tile_size);
            info!("Spawned tile at grid ({},{}) world ({:.1},{:.1},{:.1}) entity {:?}", 
                x, y, world_pos.x, world_pos.y, world_pos.z, entity);
        }
    }
}

/// System to clear all spawned tiles when needed (e.g., state transitions)
pub fn clear_spawned_tiles_system(mut spawned_tiles: ResMut<SpawnedTiles>) {
    spawned_tiles.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_view_culling_config_default() {
        let config = ViewCullingConfig::default();
        assert_eq!(config.buffer_tiles, DEFAULT_BUFFER_TILES);
        assert_eq!(config.tiles_per_frame, DEFAULT_TILES_PER_FRAME);
        assert!(config.enabled);
    }

    #[test]
    fn test_spawned_tiles_operations() {
        let mut spawned = SpawnedTiles::default();

        // Test insert and contains
        spawned.insert(5, 10);
        assert!(spawned.contains(5, 10));
        assert!(!spawned.contains(10, 5));

        // Test remove
        spawned.remove(5, 10);
        assert!(!spawned.contains(5, 10));

        // Test clear
        spawned.insert(1, 1);
        spawned.insert(2, 2);
        spawned.clear();
        assert!(!spawned.contains(1, 1));
        assert!(!spawned.contains(2, 2));
    }

    #[test]
    fn test_calculate_visible_bounds() {
        let grid_config = GridConfig {
            width: 100,
            height: 100,
            tile_size: 64.0,
        };

        let mut camera_transform = Transform::from_xyz(0.0, 0.0, 0.0);
        camera_transform.scale = Vec3::splat(1.0); // Default zoom
        let window = Window {
            resolution: (1280.0, 720.0).into(),
            ..default()
        };

        let (min_x, min_y, max_x, max_y) = calculate_visible_bounds(
            &camera_transform,
            1.0, // Default zoom for testing
            &window,
            &grid_config,
            5, // Base buffer for testing
        );

        // With camera at origin (0,0) in world space, we should see tiles near grid origin
        // The visible area should be reasonable given the window size and tile size
        assert!(min_x >= 0);
        assert!(max_x < grid_config.width);
        assert!(min_y >= 0);
        assert!(max_y < grid_config.height);

        // Should see at least some tiles
        assert!(max_x > min_x);
        assert!(max_y > min_y);

        // With 1280x720 window and 64px tiles, we should see roughly 20x11 tiles
        let visible_tile_width = (max_x - min_x + 1) as i32;
        let visible_tile_height = (max_y - min_y + 1) as i32;
        assert!(visible_tile_width > 10 && visible_tile_width < 40);
        assert!(visible_tile_height > 5 && visible_tile_height < 30);
    }

    #[test]
    fn test_view_culling_disabled() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default()));
        app.init_asset::<Mesh>();
        app.init_asset::<ColorMaterial>();

        // Add resources with culling disabled
        let mut config = ViewCullingConfig::default();
        config.enabled = false;
        app.insert_resource(config);
        app.insert_resource(GridConfig::default());
        app.insert_resource(GridMap::default());
        app.insert_resource(SpawnedTiles::default());

        // Add TileMeshes resource
        app.world_mut()
            .run_system_once(
                |mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 grid_config: Res<GridConfig>| {
                    use super::super::systems::{create_tile_diamond_mesh, TileMeshes};
                    let tile_mesh = create_tile_diamond_mesh(
                        grid_config.tile_size,
                        grid_config.tile_size * 0.5,
                    );
                    commands.insert_resource(TileMeshes {
                        diamond: meshes.add(tile_mesh),
                    });
                },
            )
            .expect("Failed to initialize tile meshes");

        // Run system - should exit early
        app.world_mut()
            .run_system_once(view_culling_system)
            .expect("System should run");

        // No tiles should be spawned
        let spawned = app.world().resource::<SpawnedTiles>();
        assert_eq!(spawned.count(), 0);
    }
}
