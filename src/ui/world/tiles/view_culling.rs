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
use crate::ui::world::grid::{coordinates::world_to_grid, GridConfig, GridMap};
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
    window: &Window,
    grid_config: &GridConfig,
    base_buffer: i32,
) -> (i32, i32, i32, i32) {
    // Calculate visible world area
    // When camera scale < 1.0 (zoomed out), we can see MORE of the world
    // When camera scale > 1.0 (zoomed in), we can see LESS of the world
    // The scale affects how much world space fits in our window
    let camera_scale = camera_transform.scale.x; // Assuming uniform scale

    // Prevent division by zero and handle extreme zoom
    let camera_scale = camera_scale.max(0.001); // Very small minimum to prevent division by zero

    // Dynamic buffer based on zoom level
    // When zoomed in (scale > 1), we need more buffer to prevent tiles from popping
    // When zoomed out (scale < 1), we need less buffer since we see more tiles anyway
    let dynamic_buffer = if camera_scale > 1.0 {
        // Zoomed in: increase buffer proportionally
        (base_buffer as f32 * camera_scale.sqrt()) as i32
    } else {
        // Zoomed out: reduce buffer but keep minimum
        (base_buffer as f32 * camera_scale).max(MIN_DYNAMIC_BUFFER) as i32
    };

    // When camera scale < 1.0 (zoomed out), we see MORE of the world
    // When camera scale > 1.0 (zoomed in), we see LESS of the world
    // The visible world size is inversely proportional to the camera scale
    let visible_width = window.width() / camera_scale;
    let visible_height = window.height() / camera_scale;

    // Camera position in world space
    let cam_x = camera_transform.translation.x;
    let cam_y = camera_transform.translation.y;

    // Calculate world bounds
    let left = cam_x - visible_width * 0.5;
    let right = cam_x + visible_width * 0.5;
    let bottom = cam_y - visible_height * 0.5;
    let top = cam_y + visible_height * 0.5;

    // Debug logging - always log to help diagnose issues
    if camera_scale < 0.2 || camera_scale > 5.0 {
        info!(
            "View Culling: scale={:.3}, window={}x{}, visible={:.0}x{:.0}, cam_pos=({:.0},{:.0}), world_bounds=({:.0},{:.0})-({:.0},{:.0})",
            camera_scale, window.width() as i32, window.height() as i32, 
            visible_width, visible_height,
            cam_x, cam_y, left, bottom, right, top
        );
    }

    // For isometric view, we need to check more points around the visible diamond
    // The camera sees a diamond shape, not a rectangle
    let center_x = cam_x;
    let center_y = cam_y;
    
    // Check 8 points around the diamond perimeter for better coverage
    let check_points = [
        // Four corners
        Vec3::new(left, bottom, 0.0),
        Vec3::new(right, top, 0.0),
        Vec3::new(left, top, 0.0),
        Vec3::new(right, bottom, 0.0),
        // Four midpoints for better diamond coverage
        Vec3::new(center_x, bottom, 0.0),
        Vec3::new(center_x, top, 0.0),
        Vec3::new(left, center_y, 0.0),
        Vec3::new(right, center_y, 0.0),
    ];
    
    // Convert all points to grid coordinates and find extremes
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    
    for point in &check_points {
        let (grid_x, grid_y, _) = world_to_grid(*point, grid_config.tile_size);
        min_x = min_x.min(grid_x);
        max_x = max_x.max(grid_x);
        min_y = min_y.min(grid_y);
        max_y = max_y.max(grid_y);
    }
    
    // Add buffer
    min_x = min_x.saturating_sub(dynamic_buffer);
    max_x = max_x.saturating_add(dynamic_buffer);
    min_y = min_y.saturating_sub(dynamic_buffer);
    max_y = max_y.saturating_add(dynamic_buffer);

    // Debug grid bounds before clamping
    if camera_scale < 0.5 {
        debug!(
            "Grid bounds before clamp: x=({}-{}), y=({}-{}), buffer={}, scale={}",
            min_x, max_x, min_y, max_y, dynamic_buffer, camera_scale
        );
    }

    // When zoomed out enough to potentially see the whole map,
    // ensure we include all tiles that might be visible
    if camera_scale < 0.3 {
        // At low zoom, the rectangular camera view can see beyond the diamond map bounds
        // Just include the entire map to ensure nothing is missed
        min_x = 0;
        max_x = grid_config.width - 1;
        min_y = 0;
        max_y = grid_config.height - 1;
        
        if camera_scale < 0.2 {
            info!("Low zoom {:.3}: Including entire map (0-{}, 0-{})", 
                 camera_scale, max_x, max_y);
        }
    } else {
        // Normal culling for closer zoom levels
        min_x = min_x.max(0);
        max_x = max_x.min(grid_config.width - 1);
        min_y = min_y.max(0);
        max_y = max_y.min(grid_config.height - 1);
        
        // Ensure we have valid bounds
        max_x = max_x.max(min_x);
        max_y = max_y.max(min_y);
    }
    
    let (min_x, max_x, min_y, max_y) = (min_x, max_x, min_y, max_y);

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
    camera_query: Query<&Transform, With<IsometricCamera>>,
    tile_query: Query<(Entity, &TilePosition), With<Tile>>,
    windows: Query<&Window>,
    mut biome_cache: Local<Vec<Vec<TileBiome>>>,
) {
    // Skip if culling is disabled
    if !culling_config.enabled {
        return;
    }

    // Get camera and window
    let Ok(camera_transform) = camera_query.single() else {
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
    }

    // Calculate visible bounds
    let (min_x, min_y, max_x, max_y) = calculate_visible_bounds(
        camera_transform,
        window,
        &grid_config,
        culling_config.buffer_tiles,
    );

    // Always log for debugging
    info!(
        "Culling: scale={:.3}, visible_tiles=({}-{}, {}-{}), grid_size={}x{}, tiles_to_spawn={}, total_spawned={}",
        camera_transform.scale.x,
        min_x, max_x,
        min_y, max_y,
        grid_config.width, grid_config.height,
        (max_x - min_x + 1) * (max_y - min_y + 1),
        spawned_tiles.count()
    );

    // Collect tiles to despawn (outside visible bounds)
    let mut tiles_to_despawn = Vec::new();
    for (entity, position) in tile_query.iter() {
        if position.x < min_x || position.x > max_x || position.y < min_y || position.y > max_y {
            tiles_to_despawn.push((entity, position.x, position.y));
        }
    }

    // Despawn tiles outside view
    for (entity, x, y) in tiles_to_despawn {
        commands.entity(entity).despawn();
        grid_map.remove_tile(x, y);
        spawned_tiles.remove(x, y);
    }

    // Spawn tiles within view (limited per frame)
    let mut tiles_spawned = 0;

    'outer: for y in min_y..=max_y {
        for x in min_x..=max_x {
            // Skip if already spawned
            if spawned_tiles.contains(x, y) {
                continue;
            }

            // Skip if we've hit the per-frame limit
            if tiles_spawned >= culling_config.tiles_per_frame {
                break 'outer;
            }

            // Get biome from cache
            let biome = biome_cache[y as usize][x as usize];
            let position = TilePosition::ground(x, y);

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
            grid_map.insert_tile(x, y, entity);
            spawned_tiles.insert(x, y);
            tiles_spawned += 1;
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
        assert_eq!(config.buffer_tiles, 5);
        assert_eq!(config.tiles_per_frame, 50);
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
