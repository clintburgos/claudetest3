//! 2D Mesh Rendering Demo for Bevy 0.16
//!
//! This example demonstrates various ways to render 2D meshes in Bevy 0.16:
//! - Using built-in shapes (Rectangle, Circle)
//! - Creating custom meshes manually
//! - Using ColorMaterial for basic coloring

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::basic::*,
    prelude::*,
    render::mesh::{Indices, Mesh2d},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2d);

    // Example 1: Rectangle using built-in primitive
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 50.0))),
        MeshMaterial2d(materials.add(Color::from(PURPLE))),
        Transform::from_translation(Vec3::new(-200.0, 100.0, 0.0)),
    ));

    // Example 2: Circle using built-in primitive
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(40.0))),
        MeshMaterial2d(materials.add(Color::from(RED))),
        Transform::from_translation(Vec3::new(0.0, 100.0, 0.0)),
    ));

    // Example 3: Custom triangle mesh
    let triangle_mesh = create_triangle_mesh();
    commands.spawn((
        Mesh2d(meshes.add(triangle_mesh)),
        MeshMaterial2d(materials.add(Color::from(GREEN))),
        Transform::from_translation(Vec3::new(200.0, 100.0, 0.0)),
    ));

    // Example 4: Custom hexagon mesh
    let hexagon_mesh = create_hexagon_mesh(50.0);
    commands.spawn((
        Mesh2d(meshes.add(hexagon_mesh)),
        MeshMaterial2d(materials.add(Color::from(YELLOW))),
        Transform::from_translation(Vec3::new(-200.0, -100.0, 0.0)),
    ));

    // Example 5: Custom diamond/rhombus mesh (isometric tile shape)
    let diamond_mesh = create_diamond_mesh(80.0, 40.0);
    commands.spawn((
        Mesh2d(meshes.add(diamond_mesh)),
        MeshMaterial2d(materials.add(Color::from(AQUA))),
        Transform::from_translation(Vec3::new(0.0, -100.0, 0.0)),
    ));

    // Example 6: Custom star mesh
    let star_mesh = create_star_mesh(60.0, 30.0, 5);
    commands.spawn((
        Mesh2d(meshes.add(star_mesh)),
        MeshMaterial2d(materials.add(Color::from(YELLOW))),
        Transform::from_translation(Vec3::new(200.0, -100.0, 0.0)),
    ));

    // Add some text labels
    commands.spawn((
        Text::new("2D Mesh Examples"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 250.0, 0.0)),
    ));
}

/// Create a simple triangle mesh
fn create_triangle_mesh() -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Define vertices (x, y, z) - note z is always 0 for 2D
    let vertices = vec![
        [0.0, 50.0, 0.0],    // Top vertex
        [-43.3, -25.0, 0.0], // Bottom left
        [43.3, -25.0, 0.0],  // Bottom right
    ];

    // Define normals (all pointing forward in 2D)
    let normals = vec![[0.0, 0.0, 1.0]; 3];

    // Define UV coordinates
    let uvs = vec![
        [0.5, 1.0], // Top
        [0.0, 0.0], // Bottom left
        [1.0, 0.0], // Bottom right
    ];

    // Define triangle indices
    let indices = Indices::U32(vec![0, 1, 2]);

    // Add attributes to mesh
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

/// Create a hexagon mesh
fn create_hexagon_mesh(radius: f32) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Create vertices for a hexagon
    let mut vertices = vec![[0.0, 0.0, 0.0]]; // Center vertex
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];

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

/// Create a diamond/rhombus mesh (useful for isometric tiles)
fn create_diamond_mesh(width: f32, height: f32) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let half_width = width / 2.0;
    let half_height = height / 2.0;

    // Define vertices for a diamond shape
    let vertices = vec![
        [0.0, half_height, 0.0],  // Top
        [half_width, 0.0, 0.0],   // Right
        [0.0, -half_height, 0.0], // Bottom
        [-half_width, 0.0, 0.0],  // Left
    ];

    let normals = vec![[0.0, 0.0, 1.0]; 4];

    let uvs = vec![
        [0.5, 1.0], // Top
        [1.0, 0.5], // Right
        [0.5, 0.0], // Bottom
        [0.0, 0.5], // Left
    ];

    // Two triangles to form the diamond
    let indices = Indices::U32(vec![
        0, 1, 3, // Top triangle
        1, 2, 3, // Bottom triangle
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

/// Create a star mesh
fn create_star_mesh(outer_radius: f32, inner_radius: f32, points: usize) -> Mesh {
    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    let mut vertices = vec![[0.0, 0.0, 0.0]]; // Center
    let mut normals = vec![[0.0, 0.0, 1.0]];
    let mut uvs = vec![[0.5, 0.5]];

    // Create vertices alternating between outer and inner radius
    for i in 0..(points * 2) {
        let angle = i as f32 * std::f32::consts::PI / points as f32;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        let x = radius * angle.sin();
        let y = radius * angle.cos();
        vertices.push([x, y, 0.0]);
        normals.push([0.0, 0.0, 1.0]);

        let u = 0.5 + 0.5 * angle.sin();
        let v = 0.5 + 0.5 * angle.cos();
        uvs.push([u, v]);
    }

    // Create triangles from center to each point
    let mut indices = Vec::new();
    for i in 0..(points * 2) {
        indices.push(0); // Center
        indices.push((i + 1) as u32);
        indices.push(((i + 1) % (points * 2) + 1) as u32);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
