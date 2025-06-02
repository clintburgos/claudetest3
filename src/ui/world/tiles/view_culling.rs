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
use crate::constants::camera::MIN_CAMERA_SCALE;
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
    let camera_scale = camera_scale.max(MIN_CAMERA_SCALE);

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

    // When zoomed out (scale < 1), we see more world units
    // When zoomed in (scale > 1), we see fewer world units
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

    // Debug logging
    debug!(
        "View Culling Debug: camera_scale={:.2}, window={}x{}, visible={}x{}, camera_pos=({:.0}, {:.0}), bounds=(L:{:.0}, R:{:.0}, B:{:.0}, T:{:.0})",
        camera_scale, window.width(), window.height(), visible_width, visible_height,
        cam_x, cam_y, left, right, bottom, top
    );

    // Convert corners to grid coordinates
    let (min_x_1, min_y_1, _) = world_to_grid(Vec3::new(left, bottom, 0.0), grid_config.tile_size);
    let (max_x_1, max_y_1, _) = world_to_grid(Vec3::new(right, top, 0.0), grid_config.tile_size);
    let (min_x_2, min_y_2, _) = world_to_grid(Vec3::new(left, top, 0.0), grid_config.tile_size);
    let (max_x_2, max_y_2, _) = world_to_grid(Vec3::new(right, bottom, 0.0), grid_config.tile_size);

    // Get the actual bounds (accounting for isometric projection)
    let min_x = min_x_1
        .min(min_x_2)
        .min(max_x_1)
        .min(max_x_2)
        .saturating_sub(dynamic_buffer);
    let max_x = min_x_1
        .max(min_x_2)
        .max(max_x_1)
        .max(max_x_2)
        .saturating_add(dynamic_buffer);
    let min_y = min_y_1
        .min(min_y_2)
        .min(max_y_1)
        .min(max_y_2)
        .saturating_sub(dynamic_buffer);
    let max_y = min_y_1
        .max(min_y_2)
        .max(max_y_1)
        .max(max_y_2)
        .saturating_add(dynamic_buffer);

    // Clamp to grid bounds
    let min_x = min_x.max(0);
    let max_x = max_x.min(grid_config.width - 1);
    let min_y = min_y.max(0);
    let max_y = max_y.min(grid_config.height - 1);

    // Ensure we have valid bounds
    let max_x = max_x.max(min_x);
    let max_y = max_y.max(min_y);

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

    // Debug info - log every second
    #[cfg(debug_assertions)]
    {
        use std::sync::Mutex;
        use std::time::Instant;
        static LAST_LOG: Mutex<Option<Instant>> = Mutex::new(None);

        if let Ok(mut last_log) = LAST_LOG.lock() {
            if last_log.is_none()
                || last_log.unwrap().elapsed().as_secs() >= DEBUG_LOG_INTERVAL_SECS
            {
                *last_log = Some(Instant::now());
                let dynamic_buffer = if camera_transform.scale.x > 1.0 {
                    (culling_config.buffer_tiles as f32 * camera_transform.scale.x.sqrt()) as i32
                } else {
                    (culling_config.buffer_tiles as f32 * camera_transform.scale.x)
                        .max(MIN_DYNAMIC_BUFFER) as i32
                };
                info!(
                    "Culling: scale={:.2}, buffer={}, cam=({:.0},{:.0}), bounds=({}-{}, {}-{}), tiles={}",
                    camera_transform.scale.x,
                    dynamic_buffer,
                    camera_transform.translation.x,
                    camera_transform.translation.y,
                    min_x, max_x,
                    min_y, max_y,
                    (max_x - min_x + 1) * (max_y - min_y + 1)
                );
            }
        }
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
