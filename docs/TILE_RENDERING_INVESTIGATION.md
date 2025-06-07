# Tile Rendering Investigation Report

## Problem Statement

When moving the camera around the isometric map, black background areas appear where tiles should be visible. This is particularly noticeable at map edges and when zoomed out.

## Investigation Timeline

### Phase 1: Setting Up Testing Infrastructure

**Objective**: Create tools to programmatically control the camera and capture screenshots for analysis.

**Challenges**:
1. Bevy 0.16 changed the screenshot API from `ScreenshotManager` to an observer-based system
2. OS-level screenshots were capturing the wrong window

**Solution**:
```rust
// Bevy 0.16 screenshot implementation
commands
    .spawn(Screenshot::primary_window())
    .observe(save_to_disk(filename));
```

### Phase 2: Initial Camera Movement Tests

**Discovery**: Camera positions like (-3000, 3000) resulted in completely black screenshots.

**Investigation Steps**:
1. Added debug logging to view culling system
2. Found warning: "No visible tiles found! visible_rect=(-3640,2640)-(-2360,3360)"
3. Realized the visible rectangle was outside tile bounds

### Phase 3: Understanding the Coordinate System

**Key Findings**:
1. The `grid_to_world` function negates Y coordinates:
   ```rust
   Vec3::new(world_x, -world_y, world_z)
   ```

2. For a 200x200 map with tile_size=64:
   - Grid (0,0) → World (0, 0)
   - Grid (199,199) → World (0, -6368)
   - Map exists entirely in negative Y space

3. Camera at (0, 3000) was looking at empty space above the map

### Phase 4: Camera Constraints Investigation

**Problem**: Even with `DisableCameraConstraints`, camera was being clamped to (0, -3200) instead of test positions.

**Root Cause**: Camera constraints were applied during initial setup before the component could take effect.

**Attempted Solutions**:
1. Adding `DisableCameraConstraints` after camera spawn
2. Deferred camera positioning
3. Creating custom camera setup without default plugin

### Phase 5: View Culling Analysis

**Bug Found**: View culling was using `camera_transform.scale.x` instead of `camera_state.zoom`

**Fix Applied**:
```rust
// Before
let camera_scale = camera_transform.scale.x;

// After  
let camera_scale = camera_zoom; // Passed as parameter
```

**Impact**: This was causing incorrect visible bounds calculation, especially when zoomed.

## Current State

### What's Working:
1. **Testing Infrastructure**: Full suite of debugging tools operational
2. **Screenshot System**: Bevy 0.16 observer-based screenshots working
3. **Coordinate Understanding**: Clear mapping of grid to world space
4. **Camera Freedom**: Can position camera anywhere for testing
5. **Center Rendering**: Tiles render correctly when camera is centered

### What's Not Working:
1. **Edge Coverage**: Black areas appear at map edges
2. **Buffer Calculation**: Current buffer may be insufficient at boundaries
3. **Corner Cases**: Extreme positions show rendering gaps

## Technical Details

### Visible Bounds Calculation
The system calculates visible tiles through:
1. Camera position and zoom determine visible world rectangle
2. World rectangle is mapped to grid coordinates
3. Buffer tiles are added around visible area
4. Tiles within bounds are spawned/culled

### Edge Case Example
Camera at (-6000, -3000):
- Visible tiles calculated: (0,162,27,199)
- Some of these tiles don't render
- Results in black triangular areas

## Hypotheses for Remaining Issues

1. **Buffer Calculation**: Dynamic buffer may be too small at edges
   ```rust
   // Current logic
   dynamic_buffer = if camera_scale > 1.0 {
       (base_buffer as f32 * (1.0 + camera_scale.ln())).ceil() as i32
   } else {
       (base_buffer as f32 * camera_scale.sqrt()).max(1.0).ceil() as i32
   };
   ```

2. **Tile Intersection**: Conservative AABB test may miss edge tiles

3. **Spawn Timing**: Tiles may not spawn quickly enough when moving to edges

## Recommendations

1. **Increase Edge Buffer**: Add special handling for map boundaries
2. **Pre-spawn Border Tiles**: Keep edge tiles always loaded
3. **Review Culling Logic**: Ensure tiles slightly outside view aren't culled
4. **Profile Performance**: Check if spawn rate limiting causes gaps

## Test Coverage

Created comprehensive test suite covering:
- 20 different camera positions
- Multiple zoom levels (0.1 to 3.0)
- All map edges and corners
- Transitions between positions

This provides reproducible test cases for validating fixes.

## Conclusion

The investigation successfully identified multiple issues in the tile rendering pipeline and created tools for ongoing debugging. While center rendering works correctly, edge cases need additional refinement to ensure complete tile coverage at all camera positions.