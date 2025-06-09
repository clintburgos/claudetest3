use bevy::prelude::*;
use bevy::render::{mesh::*, render_asset::RenderAssetUsages};

use super::components::{Tile, TileBiome, TilePosition};
use crate::ui::world::grid::coordinates::grid_to_world;

/// Create a diamond-shaped mesh for isometric tiles
///
/// # Arguments
/// * `width` - The width of the diamond (point to point horizontally)
/// * `height` - The height of the diamond (point to point vertically)
///
/// # Returns
/// A mesh representing a diamond shape for isometric projection
pub fn create_tile_diamond_mesh(width: f32, height: f32) -> Mesh {
    let half_width = width * 0.5;
    let half_height = height * 0.5;

    // Vertices for a diamond shape (counter-clockwise from top for correct winding)
    let vertices: Vec<[f32; 3]> = vec![
        [0.0, half_height, 0.0],  // Top
        [-half_width, 0.0, 0.0],  // Left
        [0.0, -half_height, 0.0], // Bottom
        [half_width, 0.0, 0.0],   // Right
    ];

    // Normals (all facing forward for 2D)
    let normals: Vec<[f32; 3]> = vec![[0.0, 0.0, 1.0]; 4];

    // UV coordinates
    let uvs: Vec<[f32; 2]> = vec![
        [0.5, 0.0], // Top
        [0.0, 0.5], // Left
        [0.5, 1.0], // Bottom
        [1.0, 0.5], // Right
    ];

    // Indices for two triangles (counter-clockwise winding)
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

    // Create material with biome color - ensure full opacity
    let mut color = biome.color();
    color.set_alpha(1.0); // Ensure full opacity
    let material = materials.add(ColorMaterial::from(color));

    commands
        .spawn((
            Tile,
            position,
            biome,
            Mesh2d(mesh),
            MeshMaterial2d(material),
            Transform::from_translation(world_pos),
            // Explicitly add visibility components to ensure tile is visible
            Visibility::Visible,
            InheritedVisibility::default(),
            ViewVisibility::default(),
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

/// Legacy system to spawn tiles for testing purposes
/// Note: In production, view culling handles tile spawning
pub fn spawn_tile_system(
    mut commands: Commands,
    grid_config: Res<GridConfig>,
    mut grid_map: ResMut<GridMap>,
    tile_meshes: Res<TileMeshes>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Only spawn if grid already has tiles
    if grid_map.positions().count() > 0 {
        return;
    }

    info!("Spawning test tiles in a 10x10 grid");

    // Simple test pattern
    for y in 0..10 {
        for x in 0..10 {
            let position = TilePosition::ground(x, y);
            let biome = match (x + y) % 4 {
                0 => TileBiome::Plain,
                1 => TileBiome::Forest,
                2 => TileBiome::Water,
                _ => TileBiome::Desert,
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

/// System to update tile visuals based on state changes
pub fn update_tile_visuals_system(
    _tile_query: Query<(&TilePosition, &TileBiome), Changed<TileBiome>>,
) {
    // This will be implemented when we add tile state changes
    // For now, tiles don't change after being spawned
}

use crate::ui::world::grid::{GridConfig, GridMap};

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_create_tile_diamond_mesh() {
        let mesh = create_tile_diamond_mesh(64.0, 32.0);

        // Check that mesh has correct attributes
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());

        // Check vertex count
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();
        assert_eq!(positions.len(), 4); // Diamond has 4 vertices

        // Check that indices are present
        assert!(mesh.indices().is_some());
    }

    #[test]
    fn test_spawn_tile() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default()));
        app.init_asset::<ColorMaterial>();
        app.init_asset::<Mesh>();

        let entity = app
            .world_mut()
            .run_system_once(
                |mut commands: Commands,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mut materials: ResMut<Assets<ColorMaterial>>| {
                    let mesh_handle = meshes.add(create_tile_diamond_mesh(64.0, 32.0));
                    spawn_tile(
                        &mut commands,
                        TilePosition::ground(5, 10),
                        TileBiome::Forest,
                        64.0,
                        mesh_handle,
                        &mut materials,
                    )
                },
            )
            .expect("Failed to spawn tile");

        // Verify components
        let world = app.world();
        assert!(world.get::<Tile>(entity).is_some());
        assert!(world.get::<TilePosition>(entity).is_some());
        assert!(world.get::<TileBiome>(entity).is_some());
        assert!(world.get::<Mesh2d>(entity).is_some());
        assert!(world.get::<MeshMaterial2d<ColorMaterial>>(entity).is_some());
        assert!(world.get::<Transform>(entity).is_some());

        // Check position
        let position = world.get::<TilePosition>(entity).unwrap();
        assert_eq!(position.x, 5);
        assert_eq!(position.y, 10);
        assert_eq!(position.z, 0);

        // Check biome
        let biome = world.get::<TileBiome>(entity).unwrap();
        assert_eq!(*biome, TileBiome::Forest);

        // Check transform
        let transform = world.get::<Transform>(entity).unwrap();
        let expected_pos = grid_to_world(5, 10, 0, 64.0);
        assert_eq!(transform.translation, expected_pos);
    }

    #[test]
    fn test_init_tile_meshes() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default()));
        app.init_asset::<Mesh>();
        app.insert_resource(GridConfig {
            width: 20,
            height: 20,
            tile_size: 64.0,
        });

        app.world_mut()
            .run_system_once(init_tile_meshes)
            .expect("Failed to initialize tile meshes");

        // Check that resource was created
        assert!(app.world().get_resource::<TileMeshes>().is_some());
    }

    #[test]
    fn test_spawn_tile_system_empty_grid() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default()));
        app.init_asset::<Mesh>();
        app.init_asset::<ColorMaterial>();

        // Add required resources
        app.insert_resource(GridConfig::default());
        app.insert_resource(GridMap::default());

        // Initialize tile meshes first
        app.world_mut()
            .run_system_once(init_tile_meshes)
            .expect("Failed to initialize tile meshes");

        // Run spawn system
        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("Failed to run spawn tile system");

        // Check that tiles were spawned
        let grid_map = app.world().resource::<GridMap>();
        assert!(grid_map.positions().count() > 0);

        // Check that we have 10x10 = 100 tiles
        let tile_count = app
            .world()
            .query_filtered::<Entity, With<Tile>>()
            .iter(app.world())
            .count();
        assert_eq!(tile_count, 100);
    }

    #[test]
    fn test_spawn_tile_system_existing_grid() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, bevy::asset::AssetPlugin::default()));
        app.init_asset::<Mesh>();
        app.init_asset::<ColorMaterial>();

        // Add required resources
        app.insert_resource(GridConfig::default());
        let mut grid_map = GridMap::default();
        grid_map.insert_tile(0, 0, Entity::PLACEHOLDER);
        app.insert_resource(grid_map);

        // Initialize tile meshes
        app.world_mut()
            .run_system_once(init_tile_meshes)
            .expect("Failed to initialize tile meshes");

        // Run spawn system
        app.world_mut()
            .run_system_once(spawn_tile_system)
            .expect("Failed to run spawn tile system");

        // Check that no new tiles were spawned (grid wasn't empty)
        let tile_count = app
            .world_mut()
            .query_filtered::<Entity, With<Tile>>()
            .iter(app.world())
            .count();
        assert_eq!(tile_count, 0);
    }

    #[test]
    fn test_diamond_mesh_winding_order() {
        let mesh = create_tile_diamond_mesh(64.0, 32.0);

        // Get vertices
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .unwrap()
            .as_float3()
            .unwrap();

        // Verify we have 4 vertices
        assert_eq!(positions.len(), 4);

        // Get indices
        let indices = mesh.indices().unwrap();
        let indices_u32 = match indices {
            bevy::render::mesh::Indices::U16(v) => v.iter().map(|&i| i as u32).collect::<Vec<_>>(),
            bevy::render::mesh::Indices::U32(v) => v.clone(),
        };

        // We should have 6 indices (2 triangles)
        assert_eq!(indices_u32.len(), 6);

        // The indices should form valid triangles
        assert!(indices_u32.iter().all(|&i| i < 4));
    }
}