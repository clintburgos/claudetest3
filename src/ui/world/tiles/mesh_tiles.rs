//! Mesh-based Tile Rendering
//!
//! This module provides mesh-based rendering for tiles, which can be more
//! efficient and flexible than sprite-based rendering for isometric games.
//!
//! Benefits of mesh tiles:
//! - True diamond/rhombus shapes without transparency
//! - Better performance with instancing potential
//! - Easier to implement tile borders and effects
//! - Custom vertex colors for gradients

use super::components::{Tile, TileBiome, TilePosition};
use crate::ui::world::grid::coordinates::grid_to_world;
use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::mesh::{Indices, Mesh2d},
};

/// Resource to store the shared tile mesh
#[derive(Resource)]
pub struct TileMeshes {
    pub diamond: Handle<Mesh>,
}

impl TileMeshes {
    /// Create the standard tile meshes
    pub fn new(meshes: &mut Assets<Mesh>) -> Self {
        Self {
            diamond: meshes.add(create_tile_diamond_mesh(1.0, 0.5)),
        }
    }
}

/// Create a diamond-shaped mesh for isometric tiles
pub fn create_tile_diamond_mesh(width: f32, height: f32) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let half_width = width / 2.0;
    let half_height = height / 2.0;

    // Define vertices for a diamond shape
    // Order: top, right, bottom, left
    let vertices = vec![
        [0.0, half_height, 0.0],  // Top
        [half_width, 0.0, 0.0],   // Right
        [0.0, -half_height, 0.0], // Bottom
        [-half_width, 0.0, 0.0],  // Left
    ];

    // All normals point forward (positive Z)
    let normals = vec![[0.0, 0.0, 1.0]; 4];

    // UV coordinates for texture mapping
    let uvs = vec![
        [0.5, 1.0], // Top
        [1.0, 0.5], // Right
        [0.5, 0.0], // Bottom
        [0.0, 0.5], // Left
    ];

    // Two triangles to form the diamond
    let indices = Indices::U32(vec![
        0, 1, 3, // Top-left triangle
        1, 2, 3, // Bottom-right triangle
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

/// Spawn a tile using mesh rendering
pub fn spawn_mesh_tile(
    commands: &mut Commands,
    position: TilePosition,
    biome: TileBiome,
    tile_size: f32,
    tile_meshes: &TileMeshes,
    materials: &mut Assets<ColorMaterial>,
) -> Entity {
    let world_pos = grid_to_world(position.x, position.y, position.z, tile_size);

    // Create material with biome color
    let material = materials.add(ColorMaterial {
        color: biome.color(),
        ..default()
    });

    commands
        .spawn((
            Tile,
            position,
            biome,
            Mesh2d(tile_meshes.diamond.clone()),
            MeshMaterial2d(material),
            Transform::from_translation(world_pos).with_scale(Vec3::new(tile_size, tile_size, 1.0)),
            GlobalTransform::default(),
        ))
        .id()
}

/// System to initialize tile meshes resource
pub fn setup_tile_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let tile_meshes = TileMeshes::new(&mut meshes);
    commands.insert_resource(tile_meshes);
}

/// System to update mesh tile materials when biome changes
pub fn update_mesh_tile_materials(
    mut tiles: Query<(&TileBiome, &mut MeshMaterial2d<ColorMaterial>), Changed<TileBiome>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (biome, mut material_handle) in tiles.iter_mut() {
        // Create new material with updated color
        *material_handle = MeshMaterial2d(materials.add(ColorMaterial {
            color: biome.color(),
            ..default()
        }));
    }
}

/// Create a hexagonal tile mesh (alternative tile shape)
pub fn create_tile_hexagon_mesh(radius: f32) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Create vertices for a hexagon
    let mut vertices = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    // Center vertex
    vertices.push([0.0, 0.0, 0.0]);
    normals.push([0.0, 0.0, 1.0]);
    uvs.push([0.5, 0.5]);

    // Add 6 vertices around the center
    for i in 0..6 {
        let angle = i as f32 * std::f32::consts::PI * 2.0 / 6.0;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);

        // UV mapping for hexagon
        let u = 0.5 + 0.5 * angle.cos();
        let v = 0.5 + 0.5 * angle.sin();
        uvs.push([u, v]);
    }

    // Create triangles from center to each edge
    let mut indices = Vec::new();
    for i in 0..6 {
        indices.push(0); // Center
        indices.push((i + 1) as u32);
        indices.push(((i + 1) % 6 + 1) as u32);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

/// Create a tile mesh with beveled edges for a 3D effect
pub fn create_beveled_tile_mesh(width: f32, height: f32, bevel: f32) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let half_width = width / 2.0;
    let half_height = height / 2.0;
    let inner_width = half_width - bevel;
    let inner_height = half_height - bevel;

    // Define vertices: outer diamond, then inner diamond
    let vertices = vec![
        // Outer vertices
        [0.0, half_height, 0.0],  // 0: Top
        [half_width, 0.0, 0.0],   // 1: Right
        [0.0, -half_height, 0.0], // 2: Bottom
        [-half_width, 0.0, 0.0],  // 3: Left
        // Inner vertices (for bevel)
        [0.0, inner_height, 0.0],  // 4: Inner top
        [inner_width, 0.0, 0.0],   // 5: Inner right
        [0.0, -inner_height, 0.0], // 6: Inner bottom
        [-inner_width, 0.0, 0.0],  // 7: Inner left
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 8];

    let uvs = vec![
        // Outer UVs
        [0.5, 1.0],
        [1.0, 0.5],
        [0.5, 0.0],
        [0.0, 0.5],
        // Inner UVs
        [0.5, 0.8],
        [0.8, 0.5],
        [0.5, 0.2],
        [0.2, 0.5],
    ];

    // Create triangles for beveled effect
    let indices = Indices::U32(vec![
        // Center quad
        4, 5, 7, 5, 6, 7, // Bevel triangles
        0, 4, 7, 0, 7, 3, 0, 1, 4, 1, 5, 4, 1, 2, 5, 2, 6, 5, 2, 3, 6, 3, 7, 6,
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_tile_diamond_mesh() {
        let mesh = create_tile_diamond_mesh(2.0, 1.0);

        // Check that mesh has the correct attributes
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
        assert!(mesh.indices().is_some());

        // Check vertex count
        if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            assert_eq!(vertices.len(), 4); // Diamond has 4 vertices
        }

        // Check index count
        if let Some(indices) = mesh.indices() {
            assert_eq!(indices.len(), 6); // 2 triangles * 3 vertices each
        }
    }

    #[test]
    fn test_create_tile_hexagon_mesh() {
        let mesh = create_tile_hexagon_mesh(1.0);

        // Check that mesh has the correct attributes
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
        assert!(mesh.indices().is_some());

        // Check vertex count
        if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            assert_eq!(vertices.len(), 7); // Center + 6 edges
        }

        // Check index count
        if let Some(indices) = mesh.indices() {
            assert_eq!(indices.len(), 18); // 6 triangles * 3 vertices each
        }
    }

    #[test]
    fn test_create_beveled_tile_mesh() {
        let mesh = create_beveled_tile_mesh(2.0, 1.0, 0.1);

        // Check that mesh has the correct attributes
        assert!(mesh.attribute(Mesh::ATTRIBUTE_POSITION).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_NORMAL).is_some());
        assert!(mesh.attribute(Mesh::ATTRIBUTE_UV_0).is_some());
        assert!(mesh.indices().is_some());

        // Check vertex count
        if let Some(vertices) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
            assert_eq!(vertices.len(), 8); // 4 outer + 4 inner vertices
        }
    }

    #[test]
    fn test_tile_meshes_resource() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(AssetPlugin::default());

        app.add_systems(Startup, setup_tile_meshes);
        app.update();

        // Check that TileMeshes resource was created
        assert!(app.world().get_resource::<TileMeshes>().is_some());
    }
}
