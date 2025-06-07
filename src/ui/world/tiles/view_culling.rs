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
use crate::ui::world::grid::{
    coordinates::{grid_to_world, tile_intersects_rect},
    GridConfig, GridMap,
};
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

/// Calculate the world-space bounding box of the entire map
fn calculate_map_world_bounds(grid_config: &GridConfig) -> (Vec2, Vec2) {
    // Check all four corners of the map
    let corners = [
        (0, 0),
        (grid_config.width - 1, 0),
        (0, grid_config.height - 1),
        (grid_config.width - 1, grid_config.height - 1),
    ];

    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);

    for (x, y) in corners {
        let world_pos = grid_to_world(x, y, 0, grid_config.tile_size);
        min.x = min.x.min(world_pos.x - grid_config.tile_size * 0.5);
        max.x = max.x.max(world_pos.x + grid_config.tile_size * 0.5);
        min.y = min.y.min(world_pos.y - grid_config.tile_size * 0.25);
        max.y = max.y.max(world_pos.y + grid_config.tile_size * 0.25);
    }

    (min, max)
}

/// Calculate grid bounds to search for visible tiles
fn calculate_grid_search_bounds(
    _visible_min: Vec2,
    _visible_max: Vec2,
    _map_world_bounds: &(Vec2, Vec2),
    grid_config: &GridConfig,
) -> (i32, i32, i32, i32) {
    // For isometric maps, the visible rectangle could extend beyond the diamond-shaped map.
    // We need to search a conservative range that covers all potentially visible tiles.

    // Start with the full map bounds as our search space
    let grid_min_x = 0;
    let grid_max_x = grid_config.width - 1;
    let grid_min_y = 0;
    let grid_max_y = grid_config.height - 1;

    (grid_min_x, grid_min_y, grid_max_x, grid_max_y)
}

/// Find tiles that are actually visible
fn find_visible_tiles(
    search_bounds: (i32, i32, i32, i32),
    visible_min: Vec2,
    visible_max: Vec2,
    grid_config: &GridConfig,
    base_buffer: i32,
) -> (i32, i32, i32, i32) {
    let (search_min_x, search_min_y, search_max_x, search_max_y) = search_bounds;

    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    // Check each tile in the search range
    for y in search_min_y..=search_max_y {
        for x in search_min_x..=search_max_x {
            // Check if this tile is visible
            if tile_intersects_rect(x, y, grid_config.tile_size, visible_min, visible_max) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }

    // Handle case where no tiles are visible
    if min_x == i32::MAX {
        warn!(
            "No visible tiles found! visible_rect=({:.0},{:.0})-({:.0},{:.0}), search_bounds=({},{},{},{})",
            visible_min.x, visible_min.y, visible_max.x, visible_max.y,
            search_min_x, search_min_y, search_max_x, search_max_y
        );
        // Instead of returning empty, return a small area around the center of the search bounds
        let center_x = (search_min_x + search_max_x) / 2;
        let center_y = (search_min_y + search_max_y) / 2;
        let fallback_radius = base_buffer.max(5);
        return (
            (center_x - fallback_radius).max(0),
            (center_y - fallback_radius).max(0),
            (center_x + fallback_radius).min(grid_config.width - 1),
            (center_y + fallback_radius).min(grid_config.height - 1),
        );
    }

    // Apply buffer
    min_x = (min_x - base_buffer).max(0);
    max_x = (max_x + base_buffer).min(grid_config.width - 1);
    min_y = (min_y - base_buffer).max(0);
    max_y = (max_y + base_buffer).min(grid_config.height - 1);

    (min_x, min_y, max_x, max_y)
}

/// Calculate the visible tile bounds based on camera position and zoom
fn calculate_visible_bounds(
    camera_transform: &Transform,
    camera_zoom: f32,
    window: &Window,
    grid_config: &GridConfig,
    base_buffer: i32,
) -> (i32, i32, i32, i32) {
    let camera_scale = camera_zoom;
    let camera_scale = camera_scale.max(0.001);

    // Calculate visible world area
    let visible_width = window.width() / camera_scale;
    let visible_height = window.height() / camera_scale;

    let cam_x = camera_transform.translation.x;
    let cam_y = camera_transform.translation.y;

    // Visible rectangle in world space
    let visible_min = Vec2::new(cam_x - visible_width * 0.5, cam_y - visible_height * 0.5);
    let visible_max = Vec2::new(cam_x + visible_width * 0.5, cam_y + visible_height * 0.5);

    // Calculate the world bounds of the entire map to optimize our search
    let map_world_bounds = calculate_map_world_bounds(grid_config);

    // Find grid search bounds
    let search_bounds =
        calculate_grid_search_bounds(visible_min, visible_max, &map_world_bounds, grid_config);

    // Dynamic buffer based on zoom level
    let dynamic_buffer = if camera_scale > 1.0 {
        // Zoomed in: larger buffer for smooth panning
        (base_buffer as f32 * (1.0 + camera_scale.ln())).ceil() as i32
    } else {
        // Zoomed out: smaller buffer since we see more tiles
        (base_buffer as f32 * camera_scale.sqrt()).max(1.0).ceil() as i32
    };

    // Now find actual visible tiles within search bounds
    let (min_x, min_y, max_x, max_y) = find_visible_tiles(
        search_bounds,
        visible_min,
        visible_max,
        grid_config,
        dynamic_buffer,
    );

    // Debug logging for extreme zoom levels
    if !(0.2..=5.0).contains(&camera_scale) {
        info!(
            "View Culling: scale={:.3}, visible_world=({:.0},{:.0})-({:.0},{:.0}), visible_tiles=({},{})-({},{}), buffer={}",
            camera_scale,
            visible_min.x, visible_min.y, visible_max.x, visible_max.y,
            min_x, min_y, max_x, max_y,
            dynamic_buffer
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
    camera_query: Query<(&Transform, &crate::ui::world::camera::CameraState), With<IsometricCamera>>,
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
    if camera_transform.translation.x.abs() > 2000.0 || camera_transform.translation.y.abs() > 2000.0 {
        info!(
            "Edge position culling: cam_pos=({:.0},{:.0}), zoom={:.2}, visible_tiles=({},{},{},{})",
            camera_transform.translation.x,
            camera_transform.translation.y,
            camera_state.zoom,
            min_x, min_y, max_x, max_y
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
