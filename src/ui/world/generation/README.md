# Generation Module

Procedural map generation with realistic biome placement.

## Files

### `mod.rs`
- **Purpose**: Module exports and MapGenerationPlugin
- **Plugin**: `MapGenerationPlugin` - Triggers map generation

### `generator.rs`
- **Purpose**: Map generation trait and implementations
- **Trait**: `MapGenerator` - Interface for generation algorithms
- **Structs**: `DefaultMapGenerator` - Noise-based generation

### `biome_rules.rs`
- **Purpose**: Rules for realistic biome placement
- **Functions**:
  - `evaluate_biome` - Determine biome from conditions
  - `is_valid_placement` - Check biome constraints

### `systems.rs`
- **Purpose**: Systems that execute map generation
- **Systems**:
  - `generate_map_system` - One-shot generation on startup

## Generation Algorithm

1. **Elevation Map**: Perlin noise for height values
2. **Moisture Map**: Secondary noise for wet/dry areas
3. **Biome Selection**: Based on elevation + moisture
4. **Constraint Check**: Ensure valid adjacencies
5. **Water Border**: Surround map with water tiles

## Biome Rules

- **Water**: Low elevation or map border
- **Coast**: Adjacent to water, low elevation
- **Plain**: Medium elevation, medium moisture
- **Forest**: Medium elevation, high moisture
- **Desert**: Medium elevation, low moisture
- **Mountain**: High elevation