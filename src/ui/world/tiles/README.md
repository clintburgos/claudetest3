# Tiles Module

Individual tile representation, components, and rendering systems.

## Files

### `mod.rs`
- **Purpose**: Module exports and TilePlugin registration
- **Plugin**: `TilePlugin` - Registers tile systems

### `components.rs`
- **Purpose**: Tile-related component definitions
- **Components**:
  - `Tile` - Marker component for tile entities
  - `TilePosition` - Grid coordinates (x, y, z)
  - `TileBiome` - Biome type enum

### `systems.rs`
- **Purpose**: Tile spawning and visual updates
- **Systems**:
  - `spawn_tile_system` - Creates tile entities
  - `update_tile_visuals_system` - Updates appearance based on biome

## Component Details

```rust
#[derive(Component)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub enum TileBiome {
    Plain,
    Forest,
    Coast,
    Water,
    Desert,
    Mountain,
}
```

## Usage

Tiles are spawned automatically by the map generation system. Each tile entity has:
- `Tile` marker
- `TilePosition` for grid location
- `TileBiome` for terrain type
- `Transform` for world position
- Visual components (sprites/colors)