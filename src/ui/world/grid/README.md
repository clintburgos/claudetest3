# Grid Module

Grid structure, coordinate systems, and spatial organization.

## Files

### `mod.rs`
- **Purpose**: Module exports and GridPlugin registration
- **Plugin**: `GridPlugin` - Registers grid resources

### `components.rs`
- **Purpose**: Grid-related resources and configuration
- **Resources**:
  - `GridMap` - Stores grid dimensions and tile references
  - `GridConfig` - Configuration for tile size and grid bounds

### `coordinates.rs`
- **Purpose**: Coordinate transformation functions
- **Functions**:
  - `grid_to_world` - Convert grid (x,y,z) to world position
  - `world_to_grid` - Convert world position to grid coords
  - `grid_to_isometric` - Convert to isometric screen space

## Coordinate System

```
Grid Space (x, y, z):
- x: West → East (increases right)
- y: North → South (increases down)
- z: Elevation (future use)

World Space:
- Standard Bevy coordinates
- Centered on grid center

Isometric Space:
- 2:1 width:height ratio
- Diamond-shaped tiles
```

## Usage

```rust
// Convert grid position to world
let world_pos = grid_to_world(x, y, z, tile_size);

// Get tile at grid position
let tile = grid_map.get_tile(x, y);
```