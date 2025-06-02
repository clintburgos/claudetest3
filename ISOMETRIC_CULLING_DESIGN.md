# Isometric View Culling Design Document

## Document Information
- **Author**: Claude
- **Date**: December 2024
- **Project**: Bevy 0.16 Isometric Grid Game
- **Related Files**:
  - `src/ui/world/tiles/view_culling.rs` - Current implementation
  - `src/ui/world/grid/coordinates.rs` - Coordinate transformation functions
  - `src/constants.rs` - Culling constants
  - `examples/debug_isometric_culling.rs` - Debug visualization tool

## Problem Statement

The current view culling system in the Bevy isometric game has a fundamental mismatch between the camera's view and the tile culling logic:

1. **Current Issues:**
   - Tiles at map edges disappear when zooming out
   - The culling uses rectangular bounds in grid space, but the isometric view creates a diamond-shaped visible area
   - Arbitrary zoom thresholds (< 0.3) are used as a workaround
   - The world-to-grid coordinate conversion can produce out-of-bounds values when the camera view extends beyond the map

2. **Root Cause:**
   - The isometric map forms a diamond shape in world space
   - The camera sees a rectangular area in screen/world space
   - The current culling only checks 4 corners of the rectangle, missing tiles that are visible in the diamond corners

## Technical Background

### Understanding Isometric Coordinates (For Engineers New to Isometric)

#### What is Isometric Projection?
Isometric projection is a method of visually representing 3D objects in 2D. In games, it creates a "2.5D" effect where:
- The world appears to be viewed from a 45-degree angle
- Objects have depth without true 3D perspective
- Classic examples: SimCity 2000, Diablo 2, Age of Empires

#### Coordinate Systems Explained

1. **Grid Space (Logical Space)**
   - Integer coordinates (x, y) representing tile positions
   - Range: (0, 0) to (width-1, height-1)
   - Forms a regular square grid
   - Think of this as a chess board where each tile has an (x,y) position
   ```
   Grid Space (5x5 example):
   (0,4) (1,4) (2,4) (3,4) (4,4)
   (0,3) (1,3) (2,3) (3,3) (4,3)
   (0,2) (1,2) (2,2) (3,2) (4,2)
   (0,1) (1,1) (2,1) (3,1) (4,1)
   (0,0) (1,0) (2,0) (3,0) (4,0)
   ```

2. **World Space (After Isometric Transform)**
   - Float coordinates after applying isometric transformation
   - The square grid becomes a diamond shape
   - **Key formulas** (from `coordinates.rs`):
     ```rust
     // Grid to World transformation
     world_x = (grid_x - grid_y) * tile_size * 0.5
     world_y = -(grid_x + grid_y) * tile_size * 0.25  // Note the negative!
     world_z = grid_z * tile_size * 0.5
     ```
   - **Why these formulas?**
     - `(grid_x - grid_y)`: Moving right in grid increases X, moving up decreases X
     - `(grid_x + grid_y)`: Moving right OR up both increase Y
     - `* 0.5` and `* 0.25`: Create the 2:1 diamond aspect ratio
     - Negative Y: Bevy uses Y-up, but we want Y to increase downward on screen

3. **Visual Representation**
   ```
   Grid Space:          World Space (Isometric):
   
   2 [#][#][#]              [#]
   1 [#][#][#]           [#]   [#]
   0 [#][#][#]        [#]   [#]   [#]
     0  1  2           [#]   [#]
                         [#]
   
   Square → Diamond transformation
   ```

#### Critical Math Relationships

1. **Grid (0,0) → World (0,0)**
   - The origin stays at the origin

2. **Grid corners in world space (for 200x200 map, tile_size=64):**
   - (0,0) → (0, 0) - Bottom corner of diamond
   - (199,0) → (6368, -3184) - Right corner
   - (0,199) → (-6368, -3184) - Left corner  
   - (199,199) → (0, -6368) - Top corner

3. **World bounds for 200x200 map:**
   - X: -6368 to +6368 (width = 12736)
   - Y: -6368 to 0 (height = 6368)
   - Forms a diamond centered at (0, -3184)

#### Understanding the Inverse Transform (World to Grid)

The world_to_grid function is critical but can be confusing. Here's the derivation:

**Given:**
```
world_x = (grid_x - grid_y) * tile_size * 0.5
world_y = -(grid_x + grid_y) * tile_size * 0.25
```

**Solving for grid_x and grid_y:**
```
Step 1: Rearrange the equations
world_x / (tile_size * 0.5) = grid_x - grid_y    ... (1)
-world_y / (tile_size * 0.25) = grid_x + grid_y   ... (2)

Step 2: Add equations (1) and (2)
(world_x / (tile_size * 0.5)) + (-world_y / (tile_size * 0.25)) = 2 * grid_x
grid_x = [(world_x / (tile_size * 0.5)) + (-world_y / (tile_size * 0.25))] / 2

Step 3: Subtract equation (1) from (2)
(-world_y / (tile_size * 0.25)) - (world_x / (tile_size * 0.5)) = 2 * grid_y
grid_y = [(-world_y / (tile_size * 0.25)) - (world_x / (tile_size * 0.5))] / 2
```

**Important:** When world coordinates are outside the diamond map bounds, this can give negative or very large grid coordinates!

#### The Culling Problem Visualized

```
Camera View (Rectangle) vs Map (Diamond):

        Camera at min zoom
   +-----------------------+
   |                       |
   |    ◊                 |
   |   ◊ ◊   Diamond      |
   |  ◊ ◊ ◊  Map          |
   |   ◊ ◊                |
   |    ◊                 |
   |                       |
   +-----------------------+

Problem: Rectangle corners extend beyond diamond!
```

When zoomed out:
- Camera sees a rectangle in world space
- Map exists as a diamond in world space
- Rectangle corners are outside the diamond
- world_to_grid() returns invalid coordinates for those corners
- Current code uses min/max of corners → misses edge tiles!

### Current Implementation Issues

The current `calculate_visible_bounds` function:
1. Calculates the visible rectangle in world space
2. Converts only 4 corners to grid space
3. Takes min/max of these corners
4. This misses tiles that are visible but outside the rectangular bounds in grid space

## Proposed Solution

### Core Concept

Instead of converting world-space corners to grid coordinates, we need to find all grid tiles whose world-space positions intersect with the camera's visible rectangle.

### Algorithm Overview

1. Calculate the camera's visible rectangle in world space (existing)
2. Determine the bounding box of the entire map in world space
3. Find the intersection of these two rectangles
4. Iterate through all potentially visible grid tiles and test if their world position is within the visible rectangle
5. Use spatial optimization to avoid checking every tile

## Implementation Steps

### Step 1: Create Helper Functions

Create new helper functions in `src/ui/world/grid/coordinates.rs`:

```rust
/// Get the world-space bounding box of a single tile
/// 
/// IMPORTANT: This calculates the diamond-shaped bounds of an isometric tile!
/// Each tile is a diamond (rhombus) in world space, NOT a square.
/// 
/// Example for tile (1,0) with tile_size=64:
/// - Center: world_x = (1-0)*64*0.5 = 32, world_y = -(1+0)*64*0.25 = -16
/// - Diamond vertices:
///   - Top: (32, -16 + 16) = (32, 0)
///   - Right: (32 + 32, -16) = (64, -16)
///   - Bottom: (32, -16 - 16) = (32, -32)
///   - Left: (32 - 32, -16) = (0, -16)
pub fn get_tile_world_bounds(x: i32, y: i32, tile_size: f32) -> (Vec3, Vec3) {
    let center = grid_to_world(x, y, 0, tile_size);
    
    // For a diamond tile in isometric view:
    // - Width is tile_size (distance from left vertex to right vertex)
    // - Height is tile_size * 0.5 (distance from top vertex to bottom vertex)
    let half_width = tile_size * 0.5;
    let half_height = tile_size * 0.25;
    
    // AABB (Axis-Aligned Bounding Box) that contains the diamond
    let min = Vec3::new(center.x - half_width, center.y - half_height, 0.0);
    let max = Vec3::new(center.x + half_width, center.y + half_height, 0.0);
    
    (min, max)
}

/// Check if a tile's world bounds intersect with a rectangle
/// 
/// This performs an AABB intersection test between:
/// - The tile's bounding box (which contains its diamond shape)
/// - The camera's visible rectangle
/// 
/// NOTE: This is conservative - it may return true for tiles whose
/// bounding box intersects but whose actual diamond doesn't.
/// This is fine for culling (better to render too much than too little).
pub fn tile_intersects_rect(
    tile_x: i32, 
    tile_y: i32, 
    tile_size: f32,
    rect_min: Vec2,
    rect_max: Vec2
) -> bool {
    let (tile_min, tile_max) = get_tile_world_bounds(tile_x, tile_y, tile_size);
    
    // AABB intersection test: rectangles DON'T intersect if:
    // - tile is completely to the left of rect (tile_max.x < rect_min.x)
    // - tile is completely to the right of rect (tile_min.x > rect_max.x)
    // - tile is completely below rect (tile_max.y < rect_min.y)
    // - tile is completely above rect (tile_min.y > rect_max.y)
    // 
    // They DO intersect if none of these are true (hence the NOT)
    !(tile_max.x < rect_min.x || 
      tile_min.x > rect_max.x || 
      tile_max.y < rect_min.y || 
      tile_min.y > rect_max.y)
}

/// More accurate diamond intersection test (optional optimization)
/// 
/// This checks if the actual diamond shape intersects the rectangle,
/// not just the bounding box. More expensive but more accurate.
pub fn tile_diamond_intersects_rect(
    tile_x: i32,
    tile_y: i32,
    tile_size: f32,
    rect_min: Vec2,
    rect_max: Vec2
) -> bool {
    let center = grid_to_world(tile_x, tile_y, 0, tile_size);
    
    // Diamond vertices
    let vertices = [
        Vec2::new(center.x, center.y + tile_size * 0.25),  // Top
        Vec2::new(center.x + tile_size * 0.5, center.y),   // Right
        Vec2::new(center.x, center.y - tile_size * 0.25),  // Bottom
        Vec2::new(center.x - tile_size * 0.5, center.y),   // Left
    ];
    
    // Check if any vertex is inside the rectangle
    for v in &vertices {
        if v.x >= rect_min.x && v.x <= rect_max.x &&
           v.y >= rect_min.y && v.y <= rect_max.y {
            return true;
        }
    }
    
    // Check if rectangle center is inside diamond (simplified)
    let rect_center = (rect_min + rect_max) * 0.5;
    // ... diamond containment test (omitted for brevity)
    
    // Check if edges intersect (omitted for brevity)
    
    false
}
```

### Step 2: Rewrite calculate_visible_bounds

Replace the current implementation in `src/ui/world/tiles/view_culling.rs`:

```rust
fn calculate_visible_bounds(
    camera_transform: &Transform,
    window: &Window,
    grid_config: &GridConfig,
    base_buffer: i32,
) -> (i32, i32, i32, i32) {
    let camera_scale = camera_transform.scale.x;
    let camera_scale = camera_scale.max(0.001);
    
    // Calculate visible world area
    let visible_width = window.width() / camera_scale;
    let visible_height = window.height() / camera_scale;
    
    let cam_x = camera_transform.translation.x;
    let cam_y = camera_transform.translation.y;
    
    // Visible rectangle in world space
    let visible_min = Vec2::new(
        cam_x - visible_width * 0.5,
        cam_y - visible_height * 0.5
    );
    let visible_max = Vec2::new(
        cam_x + visible_width * 0.5,
        cam_y + visible_height * 0.5
    );
    
    // Calculate the world bounds of the entire map to optimize our search
    let map_world_bounds = calculate_map_world_bounds(grid_config);
    
    // Find grid search bounds by checking extremes
    let search_bounds = calculate_grid_search_bounds(
        visible_min,
        visible_max,
        &map_world_bounds,
        grid_config
    );
    
    // Now find actual visible tiles within search bounds
    let (min_x, min_y, max_x, max_y) = find_visible_tiles(
        search_bounds,
        visible_min,
        visible_max,
        grid_config,
        base_buffer
    );
    
    (min_x, min_y, max_x, max_y)
}
```

### Step 3: Implement Supporting Functions

Add these functions to the view_culling.rs file:

```rust
/// Calculate the world-space bounding box of the entire map
fn calculate_map_world_bounds(grid_config: &GridConfig) -> (Vec2, Vec2) {
    // Check all four corners of the map
    let corners = [
        (0, 0),
        (grid_config.width - 1, 0),
        (0, grid_config.height - 1),
        (grid_config.width - 1, grid_config.height - 1),
    ];
    
    let mut min = Vec2::new(f32::MAX, f32::MAX);
    let mut max = Vec2::new(f32::MIN, f32::MIN);
    
    for (x, y) in corners {
        let world_pos = grid_to_world(x, y, 0, grid_config.tile_size);
        min.x = min.x.min(world_pos.x - grid_config.tile_size * 0.5);
        max.x = max.x.max(world_pos.x + grid_config.tile_size * 0.5);
        min.y = min.y.min(world_pos.y - grid_config.tile_size * 0.25);
        max.y = max.y.max(world_pos.y + grid_config.tile_size * 0.25);
    }
    
    (min, max)
}

/// Calculate grid bounds to search for visible tiles
fn calculate_grid_search_bounds(
    visible_min: Vec2,
    visible_max: Vec2,
    map_world_bounds: &(Vec2, Vec2),
    grid_config: &GridConfig
) -> (i32, i32, i32, i32) {
    // Intersect visible area with map bounds
    let search_min = Vec2::new(
        visible_min.x.max(map_world_bounds.0.x),
        visible_min.y.max(map_world_bounds.0.y)
    );
    let search_max = Vec2::new(
        visible_max.x.min(map_world_bounds.1.x),
        visible_max.y.min(map_world_bounds.1.y)
    );
    
    // Convert to approximate grid bounds
    // For isometric, we need to check a wider range
    let buffer = 2; // Small buffer for safety
    
    // Estimate grid bounds (this is still approximate but conservative)
    let grid_min_x = 0_i32.saturating_sub(buffer);
    let grid_max_x = (grid_config.width - 1).saturating_add(buffer);
    let grid_min_y = 0_i32.saturating_sub(buffer);
    let grid_max_y = (grid_config.height - 1).saturating_add(buffer);
    
    (grid_min_x, grid_min_y, grid_max_x, grid_max_y)
}

/// Find tiles that are actually visible
fn find_visible_tiles(
    search_bounds: (i32, i32, i32, i32),
    visible_min: Vec2,
    visible_max: Vec2,
    grid_config: &GridConfig,
    base_buffer: i32
) -> (i32, i32, i32, i32) {
    let (search_min_x, search_min_y, search_max_x, search_max_y) = search_bounds;
    
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    
    // Check each tile in the search range
    for y in search_min_y..=search_max_y {
        for x in search_min_x..=search_max_x {
            // Skip out-of-bounds tiles
            if x < 0 || x >= grid_config.width || y < 0 || y >= grid_config.height {
                continue;
            }
            
            // Check if this tile is visible
            if tile_intersects_rect(x, y, grid_config.tile_size, visible_min, visible_max) {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }
    
    // Handle case where no tiles are visible
    if min_x == i32::MAX {
        return (0, 0, 0, 0);
    }
    
    // Apply buffer
    min_x = (min_x - base_buffer).max(0);
    max_x = (max_x + base_buffer).min(grid_config.width - 1);
    min_y = (min_y - base_buffer).max(0);
    max_y = (max_y + base_buffer).min(grid_config.height - 1);
    
    (min_x, min_y, max_x, max_y)
}
```

### Step 4: Optimize the Search Algorithm

For large maps, checking every tile can be expensive. Implement a smarter search:

```rust
/// Optimized search using the diamond shape of isometric maps
fn find_visible_tiles_optimized(
    visible_min: Vec2,
    visible_max: Vec2,
    grid_config: &GridConfig,
    base_buffer: i32
) -> (i32, i32, i32, i32) {
    // For isometric grids, we can use the fact that:
    // - Moving along X in grid space moves diagonally SE in world space
    // - Moving along Y in grid space moves diagonally SW in world space
    
    // Start from the center and work outward
    let center_x = grid_config.width / 2;
    let center_y = grid_config.height / 2;
    
    let mut min_x = center_x;
    let mut max_x = center_x;
    let mut min_y = center_y;
    let mut max_y = center_y;
    
    // Expand outward until we've found all visible tiles
    let mut found_any = false;
    
    for radius in 0..((grid_config.width.max(grid_config.height) / 2) + 1) {
        let mut found_this_ring = false;
        
        // Check the ring at this radius
        for offset in -radius..=radius {
            // Check four sides of the ring
            let positions = [
                (center_x + offset, center_y - radius),
                (center_x + offset, center_y + radius),
                (center_x - radius, center_y + offset),
                (center_x + radius, center_y + offset),
            ];
            
            for (x, y) in positions {
                if x < 0 || x >= grid_config.width || y < 0 || y >= grid_config.height {
                    continue;
                }
                
                if tile_intersects_rect(x, y, grid_config.tile_size, visible_min, visible_max) {
                    min_x = min_x.min(x);
                    max_x = max_x.max(x);
                    min_y = min_y.min(y);
                    max_y = max_y.max(y);
                    found_this_ring = true;
                    found_any = true;
                }
            }
        }
        
        // If we've found tiles before but not in this ring, we can stop
        if found_any && !found_this_ring && radius > 10 {
            break;
        }
    }
    
    // Apply buffer
    min_x = (min_x - base_buffer).max(0);
    max_x = (max_x + base_buffer).min(grid_config.width - 1);
    min_y = (min_y - base_buffer).max(0);
    max_y = (max_y + base_buffer).min(grid_config.height - 1);
    
    (min_x, min_y, max_x, max_y)
}
```

### Step 5: Remove Threshold-Based Code

Remove all zoom threshold checks (like `if camera_scale < 0.3`) from the calculate_visible_bounds function.

### Step 6: Update Dynamic Buffer Calculation

Update the dynamic buffer to be more intelligent:

```rust
// Dynamic buffer based on zoom level and tile visibility
let dynamic_buffer = if camera_scale > 1.0 {
    // Zoomed in: larger buffer for smooth panning
    (base_buffer as f32 * (1.0 + camera_scale.ln())).ceil() as i32
} else {
    // Zoomed out: smaller buffer since we see more tiles
    (base_buffer as f32 * camera_scale.sqrt()).max(1.0).ceil() as i32
};
```

### Step 7: Testing

Create comprehensive tests:

1. **Unit Tests**: Test the helper functions with known inputs
2. **Integration Tests**: Test the full culling system at various zoom levels
3. **Visual Tests**: Create debug overlays showing which tiles are considered visible

## Alternative Approach: Rasterization

For very large maps, an even more efficient approach would be to rasterize the diamond shape:

1. Project the four corners of each tile to screen space
2. Use a 2D rasterization algorithm to determine which tiles overlap the screen
3. This would be more complex but could handle any projection type

## Performance Considerations

1. **Caching**: Cache the map's world bounds since they don't change
2. **Spatial Index**: For very large maps (1000x1000+), consider a spatial index like a quadtree
3. **LOD System**: At very low zoom levels, consider using level-of-detail tiles

## Migration Plan

1. Implement helper functions with tests
2. Create the new calculate_visible_bounds alongside the old one
3. Add a feature flag to switch between implementations
4. Test thoroughly with various map sizes and zoom levels
5. Remove the old implementation once verified

## Success Criteria

1. No tiles disappear when zooming out
2. No arbitrary zoom thresholds in the code
3. Performance remains acceptable (< 1ms for culling calculation)
4. All edge cases handled (very small maps, very large maps, extreme zoom levels)

## Future Enhancements

1. Support for rotating cameras
2. Support for non-rectangular viewports
3. Frustum culling for 3D elements on tiles
4. Predictive spawning based on camera velocity

## Detailed Implementation Checklist

### Pre-Implementation Setup
- [ ] Review current implementation in `src/ui/world/tiles/view_culling.rs`
- [ ] Understand coordinate systems in `src/ui/world/grid/coordinates.rs`
- [ ] Review camera system in `src/ui/world/camera/`
- [ ] Check current constants in `src/constants.rs` under `culling` module
- [ ] Run `examples/debug_isometric_culling.rs` to see current behavior

### Implementation Tasks

#### Phase 1: Helper Functions
- [ ] Add `get_tile_world_bounds()` to coordinates.rs
- [ ] Add `tile_intersects_rect()` to coordinates.rs
- [ ] Write unit tests for both functions
- [ ] Ensure functions handle edge cases (negative coordinates, out-of-bounds)

#### Phase 2: Core Algorithm
- [ ] Back up current `calculate_visible_bounds()` function
- [ ] Implement `calculate_map_world_bounds()`
- [ ] Implement `calculate_grid_search_bounds()`
- [ ] Implement `find_visible_tiles()`
- [ ] Test with small map (20x20) first
- [ ] Test with large map (200x200)

#### Phase 3: Optimization
- [ ] Profile the basic implementation
- [ ] Implement `find_visible_tiles_optimized()` if needed
- [ ] Consider early-exit conditions
- [ ] Add performance metrics/logging

#### Phase 4: Cleanup
- [ ] Remove all `camera_scale < 0.3` type checks
- [ ] Remove the threshold-based entire map inclusion
- [ ] Update dynamic buffer calculation
- [ ] Clean up debug logging

#### Phase 5: Testing & Validation
- [ ] Test zoom levels: 0.084 (min), 0.5, 1.0, 2.0, 5.625 (max)
- [ ] Test camera positions: center, all four corners
- [ ] Test map sizes: 20x20, 50x50, 200x200, 500x500
- [ ] Verify no tiles disappear at any zoom level
- [ ] Check performance (target: <1ms culling time)

### Post-Implementation
- [ ] Update documentation in CLAUDE.md
- [ ] Remove old implementation
- [ ] Update any examples that depend on culling behavior
- [ ] Performance comparison report

## Code References and Resources

### Bevy Documentation
- [Bevy 0.16 Coordinates](https://bevyengine.org/learn/book/2d/coordinates/)
- [Bevy Transform Component](https://docs.rs/bevy/0.16.0/bevy/transform/components/struct.Transform.html)
- [Bevy Camera](https://docs.rs/bevy/0.16.0/bevy/render/camera/struct.Camera.html)

### Project-Specific References
1. **Coordinate System** (from `src/ui/world/grid/coordinates.rs`):
   ```rust
   // Grid to World transformation
   world_x = (x - y) * tile_size * 0.5
   world_y = -(x + y) * tile_size * 0.25
   world_z = z * tile_size * 0.5
   ```

2. **Current Culling Constants** (from `src/constants.rs`):
   ```rust
   pub const DEFAULT_BUFFER_TILES: i32 = 5;
   pub const DEFAULT_TILES_PER_FRAME: usize = 5000;
   pub const MIN_DYNAMIC_BUFFER: f32 = 2.0;
   ```

3. **Camera State** (from `src/ui/world/camera/components.rs`):
   - `zoom`: Current zoom level (scale factor)
   - `min_zoom`: 0.084 (calculated dynamically)
   - `max_zoom`: 5.625 (calculated dynamically)

### Mathematical References
- [AABB Intersection Test](https://developer.mozilla.org/en-US/docs/Games/Techniques/2D_collision_detection#axis-aligned_bounding_box)
- [Isometric Projection Math](https://en.wikipedia.org/wiki/Isometric_projection)

## Common Pitfalls to Avoid

1. **Integer Overflow**: Use `saturating_add/sub` for grid coordinates
2. **Float Precision**: Be careful with float comparisons near boundaries
3. **Off-by-One Errors**: Grid indices are 0-based, width/height are counts
4. **Performance**: Don't check all 40,000 tiles for a 200x200 map
5. **Edge Cases**: Handle maps smaller than the viewport

## Debug Tools and Verification

1. **Visual Debug Mode**: Add overlay showing:
   - Camera's visible rectangle in world space
   - Which tiles are being checked
   - Which tiles pass the intersection test

2. **Logging**: Add configurable logging for:
   - Number of tiles checked vs spawned
   - Time taken for culling calculation
   - Grid bounds vs actual spawned bounds

3. **Test Commands**: 
   ```bash
   # Run existing debug tool
   cargo run --example debug_isometric_culling
   
   # Run with different map sizes
   cargo run --example test_zoom_edges_simple
   ```

## Questions for Stakeholders

Before implementation, clarify:
1. Is the 5000 tiles/frame spawn rate acceptable?
2. Should we support non-square maps?
3. Is camera rotation planned for the future?
4. What's the maximum expected map size?
5. Should we add a config option for the culling algorithm?

## Definition of Done

- [ ] All tiles visible at appropriate zoom levels
- [ ] No arbitrary thresholds in code
- [ ] Performance within targets
- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Code reviewed
- [ ] Documentation updated
- [ ] No regressions in existing functionality

## Troubleshooting Guide

### Common Issues and Solutions

1. **Tiles still disappearing at edges**
   - Check: Is the search bounds calculation too restrictive?
   - Check: Are you using the correct signs in world_to_grid?
   - Debug: Log the tile bounds for edge tiles and camera rect

2. **Performance is poor**
   - Check: Are you iterating over too many tiles?
   - Solution: Implement spatial partitioning or use the optimized search
   - Debug: Add timing logs to measure culling time

3. **Tiles pop in/out during movement**
   - Check: Is the buffer size appropriate?
   - Check: Is the dynamic buffer calculation working?
   - Solution: Increase buffer or add hysteresis

4. **Map appears offset**
   - Check: Is the camera centered correctly?
   - Check: Are the world bounds calculations correct?
   - Debug: Draw debug rectangles for world bounds

### Example Calculations (For Verification)

Given: 200x200 map, tile_size = 64, camera at (0, -3200)

1. **World bounds of entire map:**
   ```
   Bottom-left (0,0):     world = (0, 0)
   Bottom-right (199,0):  world = (6368, -3184)
   Top-left (0,199):      world = (-6368, -3184)
   Top-right (199,199):   world = (0, -6368)
   
   Map bounds: X(-6368 to 6368), Y(-6368 to 0)
   ```

2. **Camera at minimum zoom (0.084):**
   ```
   Window: 1280x720
   Visible size: 1280/0.084 x 720/0.084 = 15238 x 8571
   
   Visible rect:
   Left: 0 - 15238/2 = -7619
   Right: 0 + 15238/2 = 7619
   Bottom: -3200 - 8571/2 = -7486
   Top: -3200 + 8571/2 = 1086
   
   This rect FULLY CONTAINS the diamond map!
   ```

3. **Which tiles should be visible:**
   - At min zoom: ALL tiles (0,0) to (199,199)
   - The rect extends beyond the map, so every tile's center is inside

### Verification Tests

Add these assertions to your tests:

```rust
// Test 1: At minimum zoom, all corners visible
#[test]
fn test_all_corners_visible_at_min_zoom() {
    let grid_config = GridConfig { width: 200, height: 200, tile_size: 64.0 };
    let camera_scale = 0.084;
    let window_size = (1280.0, 720.0);
    
    let bounds = calculate_visible_bounds(...);
    
    assert_eq!(bounds, (0, 0, 199, 199));
}

// Test 2: Specific tile visibility
#[test] 
fn test_edge_tile_visibility() {
    // For each edge tile, verify it's included when it should be visible
    let edge_tiles = [(0,0), (199,0), (0,199), (199,199)];
    
    for (x, y) in edge_tiles {
        let world_pos = grid_to_world(x, y, 0, 64.0);
        // Verify this position would be inside camera rect at various zooms
    }
}
```