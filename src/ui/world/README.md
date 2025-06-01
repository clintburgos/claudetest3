# World Module

Isometric tile-based world implementation with procedural generation and camera controls.

## Architecture Overview

The world system is divided into four independent subsystems:
- **Tiles**: Individual tile entities and rendering
- **Grid**: Coordinate system and spatial organization
- **Generation**: Procedural map creation
- **Camera**: View controls and constraints

## Directory Structure

```
world/
├── mod.rs         # Module exports and WorldPlugin
├── tiles/         # Tile components and rendering
├── grid/          # Grid structure and coordinates
├── generation/    # Map generation algorithms
└── camera/        # Camera controls and movement
```

## Module Responsibilities

### `mod.rs`
- **Purpose**: Central registration point for world systems
- **Plugin**: `WorldPlugin` that adds all subsystem plugins
- **Exports**: Public APIs from each subsystem

### `tiles/`
- **Purpose**: Individual tile representation and visualization
- **Components**: `Tile`, `TilePosition`, `TileBiome`
- **Systems**: Tile spawning and visual updates

### `grid/`
- **Purpose**: Spatial organization and coordinate conversions
- **Resources**: `GridMap`, `GridConfig`
- **Functions**: Coordinate system transformations

### `generation/`
- **Purpose**: Procedural world creation
- **Traits**: `MapGenerator` for different algorithms
- **Rules**: Biome placement constraints

### `camera/`
- **Purpose**: View control and navigation
- **Components**: `IsometricCamera`, `CameraState`
- **Systems**: Input handling and boundary constraints

## Quick Reference

### Coordinate System
```rust
// Grid coordinates (x, y, z)
// x: West → East
// y: North → South
// z: Elevation (future use)

// Convert grid to world position
let world_pos = grid_to_world(x, y, z);

// Convert to isometric screen coordinates
let screen_pos = grid_to_isometric(x, y, z);
```

### Biome Types
- `Plain` - Grasslands (#90EE90)
- `Forest` - Dense trees (#228B22)
- `Coast` - Sandy beaches (#F4A460)
- `Water` - Ocean/lakes (#4682B4)
- `Desert` - Arid lands (#F0E68C)
- `Mountain` - Rocky peaks (#808080)

### Camera Controls
- **WASD/Arrows**: Pan camera
- **Q/E**: Zoom in/out
- **Mouse wheel**: Zoom
- **Trackpad**: Two-finger pan, pinch zoom

## Design Principles

1. **Single Responsibility**: Each module handles one aspect
2. **Loose Coupling**: Modules communicate through events/resources
3. **Data-Driven**: Components store data, systems provide behavior
4. **Extensibility**: Easy to add new biomes, controls, or generators

## Usage Example

```rust
// The WorldPlugin handles all setup
app.add_plugins(WorldPlugin);

// Access grid data
let grid = world.resource::<GridMap>();
let tile_entity = grid.get_tile(x, y);

// Query tiles
for (entity, position, biome) in tiles.iter() {
    // Process tiles
}
```