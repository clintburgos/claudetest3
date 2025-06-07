# Debugging Tools and Investigation Summary

This document summarizes the debugging tools created and investigations performed to diagnose and fix tile rendering issues in the isometric game.

## Overview

The main issue investigated was tiles not covering the visible area when the camera moves to certain positions, resulting in black background showing through. This was particularly problematic at map edges and when zoomed out.

## Debugging Tools Created

### 1. Screenshot Testing System (`/src/testing/`)

A comprehensive testing framework that allows programmatic control and screenshot capture:

**Key Files:**
- `/src/testing/mod.rs` - Main testing plugin with F1-F12 keyboard shortcuts
- `/src/testing/camera_controls.rs` - Camera test controls (F5-F7 for reset/min/max zoom)
- `/src/testing/debug_overlay.rs` - Debug overlay showing FPS, camera position, zoom, visible tiles
- `/src/testing/screenshot.rs` - Screenshot functionality using Bevy 0.16's observer API

**Features:**
- F1: Take screenshot
- F2: Toggle debug overlay
- F3: Toggle tile borders
- F4: Toggle performance stats
- F5: Reset camera
- F6: Set minimum zoom
- F7: Set maximum zoom
- F8: Toggle biome colors
- F9: Cycle test scenarios
- F10: Toggle tile labels
- F11: Toggle grid lines
- F12: Reload shaders

**Screenshot Implementation:**
```rust
commands
    .spawn(Screenshot::primary_window())
    .observe(save_to_disk(filename));
```

### 2. Automated Tile Coverage Test (`/src/bin/test_tile_coverage.rs`)

An automated test that moves the camera to 20 different positions and takes screenshots to verify tile coverage.

**Test Positions:**
- Center at various zoom levels (1.0, 0.3, 3.0)
- Edge positions (top, bottom, left, right)
- Corner positions
- Zoomed out edge positions (most likely to show gaps)
- Intermediate positions
- Full map view (zoom 0.1)

**Key Features:**
- Automatic camera movement between test positions
- Screenshot capture at each position
- Timer-based progression (1.5 seconds per position)
- Outputs to `tile_coverage_test/` directory

### 3. Fixed Coverage Test (`/src/bin/test_tile_coverage_fixed.rs`)

An improved version that:
- Bypasses the default camera plugin to avoid constraint issues
- Adds `DisableCameraConstraints` component from the start
- Uses proper isometric coordinates for test positions
- Accounts for the map's actual world space bounds

**Coordinate System Insights:**
- 200x200 map with tile_size=64
- Map center at world(0, -3200)
- World bounds: X: -6368 to 6368, Y: -6368 to 0
- All map tiles exist in negative Y space

### 4. Python Analysis Script (`analyze_tile_coverage.py`)

A script to analyze screenshots for black areas:
```python
# Detects black pixels (RGB < 10,10,10)
# Excludes expected UI regions
# Outputs percentage of black area
```

### 5. Map Bounds Calculator (`check_map_bounds.rs`)

A standalone utility to calculate world space bounds for the isometric map:
```rust
// Converts grid coordinates to world space
// Shows that Y coordinates are negated
// Helps understand camera positioning
```

## Issues Discovered

### 1. Camera Constraint System
- Initial issue: Camera constraints prevented testing edge positions
- Solution: Added `DisableCameraConstraints` component
- Finding: Constraints were being applied before the component could be added

### 2. Transform.scale vs CameraState.zoom
- Bug: View culling used `camera_transform.scale.x` instead of `camera_state.zoom`
- Fix: Updated `calculate_visible_bounds` to accept zoom parameter
- File: `/src/ui/world/tiles/view_culling.rs`

### 3. Isometric Coordinate System
- Discovery: World Y coordinates are negated in `grid_to_world`
- Impact: Camera at (0, 3000) looks at empty space above the map
- All tiles have Y ≤ 0 in world space

### 4. Edge Rendering Issues
- Current status: Tiles don't fully cover viewport at map edges
- Example: Camera at (-6000, -3000) shows black areas
- Culling system finds tiles but coverage is incomplete

## Key Code Sections Modified

### 1. View Culling (`/src/ui/world/tiles/view_culling.rs`)
```rust
// Fixed zoom parameter usage
fn calculate_visible_bounds(
    camera_transform: &Transform,
    camera_zoom: f32,  // Added parameter
    window: &Window,
    grid_config: &GridConfig,
    base_buffer: i32,
) -> (i32, i32, i32, i32)
```

### 2. Camera Components (`/src/ui/world/camera/components.rs`)
```rust
// Added component to disable constraints for testing
#[derive(Component)]
pub struct DisableCameraConstraints;
```

### 3. Camera Constraints (`/src/ui/world/camera/constraints.rs`)
```rust
// Modified query to exclude cameras with DisableCameraConstraints
Query<(&mut Transform, &CameraState), 
    (With<IsometricCamera>, Without<DisableCameraConstraints>)>
```

## Test Results

### Working Correctly:
- ✅ Center camera positions at all zoom levels
- ✅ Screenshot capture system
- ✅ Debug overlay information
- ✅ Camera movement without constraints
- ✅ Coordinate system understanding

### Issues Found:
- ❌ Black areas at map edges (e.g., top_left_corner)
- ❌ Incomplete tile coverage when camera near map boundaries
- ⚠️ Buffer tile calculation may need adjustment

## Next Steps

1. **Investigate buffer tile calculation** - The dynamic buffer based on zoom may not be sufficient at edges
2. **Review tile intersection logic** - Check if `tile_intersects_rect` is too conservative
3. **Examine tile spawning** - Ensure tiles are actually being spawned for calculated visible bounds
4. **Consider pre-spawning edge tiles** - May need special handling for map boundaries

## Usage

To run the debugging tools:

```bash
# Run the main game with testing features
cargo run

# Run automated tile coverage test
cargo run --bin test_tile_coverage

# Run fixed coverage test (recommended)
cargo run --bin test_tile_coverage_fixed

# Analyze screenshots (requires PIL)
python analyze_tile_coverage.py
```

## Conclusion

The debugging tools successfully identified that:
1. The Bevy 0.16 screenshot API works via observers
2. Camera constraints were interfering with edge testing
3. The isometric coordinate system places all tiles in negative Y space
4. Edge rendering has gaps that need to be addressed

These tools provide a solid foundation for continued debugging and fixing the tile coverage issues.