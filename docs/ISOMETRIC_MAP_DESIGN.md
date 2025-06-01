# Isometric Tile Map System - Design Document

## Overview

This document outlines the architecture for an isometric tile-based map system in Bevy 0.16. The system emphasizes modularity, single responsibility, and clear separation of concerns.

## Core Components

### 1. Tile System (`ui/world/tiles/`)

#### `ui/world/tiles/components.rs`
**Responsibility**: Define tile-related components
- `TilePosition`: Stores grid coordinates (x, y, z)
- `TileBiome`: Enum defining biome types (Plain, Forest, Coast, Water, Desert, Mountain)
- `Tile`: Marker component for tile entities

#### `ui/world/tiles/systems.rs`
**Responsibility**: Tile rendering and updates
- `spawn_tile_system`: Creates tile entities with proper transforms
- `update_tile_visuals_system`: Updates tile appearance based on biome

### 2. Grid System (`ui/world/grid/`)

#### `ui/world/grid/components.rs`
**Responsibility**: Grid structure and metadata
- `GridMap`: Resource storing grid dimensions and tile references
- `GridConfig`: Configuration for grid size and tile dimensions

#### `ui/world/grid/coordinates.rs`
**Responsibility**: Coordinate system conversions
- `grid_to_world`: Convert grid (x, y, z) to world position
- `world_to_grid`: Convert world position to grid coordinates
- `grid_to_isometric`: Convert grid to isometric screen coordinates

### 3. Map Generation (`ui/world/generation/`)

#### `ui/world/generation/generator.rs`
**Responsibility**: Procedural map generation logic
- `MapGenerator`: Trait for different generation algorithms
- `DefaultMapGenerator`: Implementation using noise-based generation

#### `ui/world/generation/biome_rules.rs`
**Responsibility**: Biome placement rules
- `BiomeRules`: Defines constraints for realistic biome placement
- `evaluate_biome`: Determines appropriate biome for given conditions

#### `ui/world/generation/systems.rs`
**Responsibility**: Map generation execution
- `generate_map_system`: One-shot system to generate the initial map

### 4. Camera System (`ui/world/camera/`)

#### `ui/world/camera/components.rs`
**Responsibility**: Camera state
- `IsometricCamera`: Marker component
- `CameraState`: Zoom level and position constraints

#### `ui/world/camera/controls.rs`
**Responsibility**: Input handling for camera
- `keyboard_camera_system`: WASD/Arrow key panning
- `mouse_camera_system`: Mouse/trackpad input handling
- `zoom_system`: Handle zoom in/out

#### `ui/world/camera/constraints.rs`
**Responsibility**: Camera boundary enforcement
- `apply_camera_constraints_system`: Keep camera within map bounds

## Data Flow

```
1. Startup Phase:
   GridConfig → MapGenerator → GridMap → Tile Entities

2. Runtime Phase:
   Input → Camera Systems → Camera Transform
   Tile Entities → Rendering
```

## Key Design Decisions

### 1. Coordinate System
- **Grid Coordinates**: (x, y, z) where:
  - x: West to East
  - y: North to South  
  - z: Elevation (for future use)
- **Isometric Projection**: 2:1 ratio (width:height)

### 2. Entity Architecture
- Each tile is an individual entity
- Tiles have position, biome, and visual components
- Grid resource maintains tile entity references

### 3. Biome Generation Algorithm
- **Noise-based**: Use Perlin/Simplex noise for natural patterns
- **Constraints**:
  - Water tiles form connected bodies
  - Mountains appear in ranges
  - Coasts only adjacent to water
  - Deserts avoid direct water adjacency

### 4. Camera System
- Orthographic projection for isometric view
- Constrained to map boundaries with padding
- Smooth movement with acceleration/deceleration

### 5. Rendering Approach
- Simple colored squares for initial implementation
- Each biome has a distinct color:
  - Plain: Light Green (#90EE90)
  - Forest: Dark Green (#228B22)
  - Coast: Sandy (#F4A460)
  - Water: Blue (#4682B4)
  - Desert: Yellow (#F0E68C)
  - Mountain: Gray (#808080)

## Module Structure

```
src/
├── main.rs
├── lib.rs
└── ui/
    ├── mod.rs
    ├── components.rs
    ├── systems.rs
    ├── styles.rs
    └── world/
        ├── mod.rs
        ├── tiles/
        │   ├── mod.rs
        │   ├── components.rs
        │   └── systems.rs
        ├── grid/
        │   ├── mod.rs
        │   ├── components.rs
        │   └── coordinates.rs
        ├── generation/
        │   ├── mod.rs
        │   ├── generator.rs
        │   ├── biome_rules.rs
        │   └── systems.rs
        └── camera/
            ├── mod.rs
            ├── components.rs
            ├── controls.rs
            └── constraints.rs
```

## Plugin Architecture

Each major system will be a Bevy plugin:
- `TilePlugin`: Tile rendering and management
- `GridPlugin`: Grid structure and coordinates
- `MapGenerationPlugin`: Procedural generation
- `IsometricCameraPlugin`: Camera controls and constraints

## Performance Considerations

1. **Tile Pooling**: Reuse tile entities when possible
2. **Frustum Culling**: Only render visible tiles
3. **Chunk System**: Group tiles for efficient updates (future)
4. **Static Batching**: Batch similar tiles (future)

## Future Extensibility

The modular design allows for:
- Additional biome types
- Elevation-based rendering (z-axis)
- Tile interactions
- Sprite-based rendering
- Chunk-based infinite maps
- Multiplayer synchronization

## Implementation Order

1. Grid system and coordinate conversions
2. Basic tile components and spawning
3. Simple colored tile rendering
4. Camera system with controls
5. Procedural map generation
6. Biome placement rules
7. Visual polish and optimizations

## Step-by-Step Implementation Plan

### Phase 1: Grid Foundation
**Goal**: Establish the coordinate system and grid structure

1. **Create Grid Components** (`ui/world/grid/components.rs`)
   - Define `GridMap` resource to store grid dimensions and tile entity references
   - Define `GridConfig` struct with map width, height, and tile size settings
   - Keep it minimal: just the data structures, no logic

2. **Implement Coordinate Conversions** (`ui/world/grid/coordinates.rs`)
   - `grid_to_world(x, y, z)` - Convert grid coordinates to world position
   - `world_to_grid(pos)` - Convert world position back to grid coordinates  
   - `grid_to_isometric(x, y)` - Convert to 2:1 isometric screen coordinates
   - Use simple math: iso_x = (x - y) * tile_width/2, iso_y = (x + y) * tile_height/2

### Phase 2: Tile System
**Goal**: Create visible tile entities on screen

3. **Define Tile Components** (`ui/world/tiles/components.rs`)
   - `TilePosition` - Store grid x, y, z coordinates
   - `TileBiome` - Enum with 6 biome types (Plain, Forest, Coast, Water, Desert, Mountain)
   - `Tile` - Empty marker component to identify tile entities
   - Each component is just data, no behavior

4. **Build Tile Spawning System** (`ui/world/tiles/systems.rs`)
   - `spawn_tile_system` - Creates one entity per grid position
   - For each tile: spawn a colored square sprite at isometric position
   - Use placeholder colors from the design doc (e.g., #90EE90 for plains)
   - Start with a small fixed grid (e.g., 10x10) for testing

### Phase 3: Camera Setup
**Goal**: View and navigate the isometric map

5. **Create Camera Components** (`ui/world/camera/components.rs`)
   - `IsometricCamera` - Marker component for the camera entity
   - `CameraState` - Store zoom level and position constraints
   - Setup orthographic camera with proper scale for isometric view

6. **Implement Basic Camera Controls** (`ui/world/camera/controls.rs`)
   - `keyboard_camera_system` - WASD or arrow keys for panning
   - Simple translation: move camera transform based on input
   - Fixed movement speed, no acceleration for now
   - Skip mouse controls and zoom initially

### Phase 4: Map Generation
**Goal**: Procedurally generate tile biomes

7. **Create Map Generator** (`ui/world/generation/generator.rs`)
   - `MapGenerator` trait with single method: `generate(width, height) -> Vec<TileBiome>`
   - `DefaultMapGenerator` - Simple noise-based implementation
   - Use basic Perlin noise to assign biomes based on thresholds
   - No complex biome rules initially, just noise values to biome mapping

8. **Build Generation System** (`ui/world/generation/systems.rs`)
   - `generate_map_system` - Runs once at startup
   - Calls generator to get biome data
   - Updates existing tiles with generated biomes
   - Triggers visual update for tile colors

### Phase 5: Integration
**Goal**: Wire everything together

9. **Create Plugins and Update Main** 
   - `GridPlugin` - Inserts GridConfig resource
   - `TilePlugin` - Adds tile spawning systems
   - `IsometricCameraPlugin` - Spawns camera, adds control systems
   - `MapGenerationPlugin` - Adds generation system
   - Update `main.rs` to add all plugins
   - Ensure proper system ordering with labels

## Implementation Notes

- Each phase produces visible results that can be tested
- Keep systems simple - no optimization or advanced features
- Use Bevy 0.16's direct component spawning (no bundles)
- Test after each phase before moving to the next
- Total scope: ~500-700 lines of code across all files