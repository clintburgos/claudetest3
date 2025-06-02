# Tiles Module

Individual tile representation, components, and rendering systems with advanced view culling for performance optimization.

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
  - `spawn_tile` - Creates a single tile entity
  - `spawn_tile_system` - Legacy batch tile spawning (for testing)
  - `update_tile_visuals_system` - Updates appearance based on biome
  - `init_tile_meshes` - Initializes shared mesh resources

### `view_culling.rs` ⭐ NEW
- **Purpose**: Performance optimization through view-based tile culling
- **Resources**:
  - `ViewCullingConfig` - Configuration for culling behavior
  - `SpawnedTiles` - Tracks currently spawned tiles
- **Systems**:
  - `view_culling_system` - Spawns/despawns tiles based on camera view
  - `clear_spawned_tiles_system` - Cleanup system
- **Key Features**:
  - Only spawns tiles visible to the camera
  - Configurable buffer zone around visible area
  - Batch processing to prevent frame drops
  - Automatic cleanup of off-screen tiles

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

### `mesh_tiles.rs`
- **Purpose**: Mesh-based tile rendering
- **Functions**:
  - `create_tile_diamond_mesh` - Creates isometric diamond mesh
  - `create_tile_hexagon_mesh` - Creates hexagonal tile mesh
  - `create_beveled_tile_mesh` - Creates 3D-effect tile mesh
- **Resources**:
  - `TileMeshes` - Stores shared mesh handles

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

### View Culling System (Performance Optimization)

The tile system now includes advanced view culling that dramatically improves performance on large maps:

```rust
// Configure view culling
commands.insert_resource(ViewCullingConfig {
    buffer_tiles: 5,         // Spawn 5 tiles beyond visible area
    tiles_per_frame: 50,     // Spawn up to 50 tiles per frame
    enabled: true,           // Enable/disable culling
});
```

**Benefits:**
- Only visible tiles are kept in memory
- Smooth scrolling with configurable buffer zone
- Batch processing prevents frame drops
- Works with maps of any size (tested up to 1000x1000)

### Tile Entity Structure

Each tile entity has:
- `Tile` marker
- `TilePosition` for grid location
- `TileBiome` for terrain type
- `Transform` for world position
- `Mesh2d` and `MeshMaterial2d` for rendering

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

## Example

Run the view culling demo to see the system in action:

```bash
cargo run --example view_culling_demo
```

**Controls:**
- WASD/Arrow keys: Move camera
- Q/E: Zoom in/out  
- Mouse wheel: Zoom
- C: Toggle view culling on/off

Watch the console to see tiles being dynamically spawned and despawned as you move around!