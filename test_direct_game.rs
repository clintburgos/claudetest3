use bevy::prelude::*;
use claudetest3::ui::world::tiles::{
    components::{Tile, TileBiome, TilePosition},
    systems::{create_tile_diamond_mesh, spawn_tile},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.4, 0.4, 0.4)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("=== Direct Game Tile Test ===");
    
    // Spawn camera at same position as game
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, -3200.0, 1000.0),
    ));
    println!("Spawned camera at (0, -3200, 1000)");
    
    // Create the same diamond mesh as the game
    let tile_size = 64.0;
    let mesh = meshes.add(create_tile_diamond_mesh(tile_size, tile_size * 0.5));
    println!("Created diamond mesh with size {}x{}", tile_size, tile_size * 0.5);
    
    // Spawn a grid of tiles around camera position
    let mut tile_count = 0;
    for y in 90..110 {
        for x in 90..110 {
            let position = TilePosition::ground(x, y);
            let biome = match (x + y) % 6 {
                0 => TileBiome::Plain,
                1 => TileBiome::Forest,
                2 => TileBiome::Coast,
                3 => TileBiome::Water,
                4 => TileBiome::Desert,
                _ => TileBiome::Mountain,
            };
            
            spawn_tile(
                &mut commands,
                position,
                biome,
                tile_size,
                mesh.clone(),
                &mut materials,
            );
            tile_count += 1;
        }
    }
    
    println!("Spawned {} tiles around grid position (100,100)", tile_count);
    println!("\nExpected: Colorful diamond tiles visible");
    println!("Actual: Check if tiles are rendering");
}