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
  - `TileHighlighted` - Marker for hovered tiles
  - `TileSelected` - Marker for selected tiles

### `systems.rs`
- **Purpose**: Tile spawning and visual updates
- **Systems**:
  - `spawn_tile_system` - Creates tile entities
  - `update_tile_visuals_system` - Updates appearance based on biome

### `interaction.rs`
- **Purpose**: Mouse interaction with tiles
- **Resources**:
  - `HoveredTile` - Tracks currently hovered tile
  - `SelectedTile` - Tracks currently selected tile
- **Systems**:
  - `tile_hover_detection_system` - Detects tile under cursor
  - `tile_selection_system` - Handles click selection
  - `tile_highlight_visual_system` - Visual feedback for hover
  - `tile_selection_visual_system` - Visual feedback for selection
- **Plugin**: `TileInteractionPlugin` - Registers interaction systems

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

## Interaction Features

The tile system supports mouse interaction:
- **Hover**: Moving the mouse over a tile highlights it
- **Selection**: Left-clicking a tile selects it
- **Visual Feedback**: 
  - Hovered tiles appear brighter
  - Selected tiles have a blue tint
  
### Accessing Interaction State

```rust
// Get the currently hovered tile
let hovered_tile = world.resource::<HoveredTile>();
if let Some(entity) = hovered_tile.entity {
    // Process hovered tile
}

// Get the currently selected tile  
let selected_tile = world.resource::<SelectedTile>();
if let Some(position) = selected_tile.position {
    println!("Selected tile at: {:?}", position);
}
```