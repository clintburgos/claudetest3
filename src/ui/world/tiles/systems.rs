//! Tile Systems - Spawning and visual updates for tiles
//!
//! This file contains systems that manage tile entities:
//! - Spawning tiles with proper components
//! - Updating tile visuals based on biome
//! - Future: tile animations, state changes
//!
//! # Design Notes
//! - Tiles are spawned as individual entities for flexibility
//! - Visual updates are separate from spawning for modularity
//! - Color-based rendering for now, sprite support planned

use super::components::{Tile, TileBiome, TilePosition};
use crate::ui::world::grid::{coordinates::grid_to_world, GridConfig, GridMap};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh2d, PrimitiveTopology};

/// Create a diamond mesh for isometric tiles
fn create_tile_diamond_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // Vertices for a diamond shape (clockwise from top)
    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top
        [half_width, 0.0, 0.0],   // Right
        [0.0, -half_height, 0.0], // Bottom
        [-half_width, 0.0, 0.0],  // Left
    ];

    // Normals (all facing forward for 2D)
    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    // UV coordinates
    let uvs: Vec<[f32; 2]> = vec![
        [0.5, 0.0], // Top
        [1.0, 0.5], // Right
        [0.5, 1.0], // Bottom
        [0.0, 0.5], // Left
    ];

    // Indices for two triangles
    let indices = vec![0, 1, 2, 0, 2, 3];

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

/// Resource to store the shared tile mesh
#[derive(Resource)]
pub struct TileMeshes {
    pub diamond: Handle<Mesh>,
}

/// Spawn a single tile entity at the given position
pub fn spawn_tile(
    commands: &mut Commands,
    position: TilePosition,
    biome: TileBiome,
    tile_size: f32,
    mesh: Handle<Mesh>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) -> Entity {
    let world_pos = grid_to_world(position.x, position.y, position.z, tile_size);

    // Create material with biome color
    let material = materials.add(ColorMaterial::from(biome.color()));

    commands
        .spawn((
            Tile,
            position,
            biome,
            Mesh2d(mesh),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(world_pos),
            GlobalTransform::default(),
        ))
        .id()
}

/// Initialize tile meshes resource
pub fn init_tile_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    grid_config: Res<GridConfig>,
) {
    let tile_mesh = create_tile_diamond_mesh(grid_config.tile_size, grid_config.tile_size * 0.5);
    commands.insert_resource(TileMeshes {
        diamond: meshes.add(tile_mesh),
    });
}

/// System to spawn tiles for the entire grid
pub fn spawn_tile_system(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    mut grid_map: ResMut<GridMap>,
    tile_meshes: Res<TileMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Only spawn if grid is empty
    if grid_map.positions().next().is_some() {
        return;
    }

    info!(
        "Spawning {} x {} tile grid",
        grid_config.width, grid_config.height
    );

    // Spawn tiles for a small test grid (10x10 for now)
    let test_width = 10.min(grid_config.width);
    let test_height = 10.min(grid_config.height);

    for y in 0..test_height {
        for x in 0..test_width {
            // Validate bounds before spawning
            if !grid_map.in_bounds(x, y) {
                warn!(
                    "Attempting to spawn tile outside grid bounds at ({}, {})",
                    x, y
                );
                continue;
            }

            let position = TilePosition::ground(x, y);
            // Create a simple pattern for testing
            let biome = match (x + y) % 6 {
                0 => TileBiome::Plain,
                1 => TileBiome::Forest,
                2 => TileBiome::Coast,
                3 => TileBiome::Water,
                4 => TileBiome::Desert,
                _ => TileBiome::Mountain,
            };

            let entity = spawn_tile(
                &mut commands,
                position,
                biome,
                grid_config.tile_size,
                tile_meshes.diamond.clone(),
                &mut materials,
            );
            grid_map.insert_tile(x, y, entity);
        }
    }
}

/// Update tile visuals based on their biome type
pub fn update_tile_visuals_system(
    tiles: Query<(&TileBiome, &MeshMaterial2d<ColorMaterial>), Changed<TileBiome>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (biome, material_handle) in tiles.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            material.color = biome.color();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::asset::AssetPlugin;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_spawn_tile() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.init_asset::<Mesh>();
        app.init_asset::<ColorMaterial>();

        // Create mesh and spawn tile
        let entity = app
            .world_mut()
            .run_system_once(
                |mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>| {
                    let mesh = meshes.add(create_tile_diamond_mesh(64.0, 32.0));
                    spawn_tile(
                        &mut commands,
                        TilePosition::new(5, 10, 2),
                        TileBiome::Forest,
                        64.0,
                        mesh,
                        &mut materials,
                    )
                },
            )
            .expect("System should run successfully");

        let world = app.world();

        // Check entity exists with correct components
        assert!(world.get::<Tile>(entity).is_some());

        let position = world.get::<TilePosition>(entity).unwrap();
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 10);
        assert_eq!(position.z, 2);

        let biome = world.get::<TileBiome>(entity).unwrap();
        assert_eq!(*biome, TileBiome::Forest);

        let transform = world.get::<Transform>(entity).unwrap();
        let expected_pos = grid_to_world(5, 10, 2, 64.0);
        assert_eq!(transform.translation, expected_pos);
    }

    #[test]
    fn test_init_tile_meshes() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.init_asset::<Mesh>();
        app.insert_resource(GridConfig::default());

        app.world_mut()
            .run_system_once(init_tile_meshes)
            .expect("System should run");

        // Check that TileMeshes resource was created
        assert!(app.world().get_resource::<TileMeshes>().is_some());
    }

    // Note: Most sprite-based tests have been removed since we switched to mesh rendering
    // The remaining tests focus on core functionality
}
