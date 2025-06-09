# Isometric Culling Fix

## Problem
When zooming out, tiles at the edges of the screen were not being rendered, leaving blank areas. This was because the culling algorithm was treating the visible area as a simple rectangle, but in isometric projection, the visible area in grid space is actually a rotated diamond.

## Solution
Created a proper isometric culling algorithm that:

1. **Converts screen corners to grid space**: Takes the four corners of the screen and converts them to grid coordinates to find the actual diamond-shaped area
2. **Dynamic buffer scaling**: Increases buffer significantly when zoomed out
3. **Diagonal buffer**: Adds extra buffer to account for the diamond shape of the visible area

## Implementation

### New File: `isometric_culling.rs`
```rust
pub fn calculate_isometric_visible_tiles(
    camera_pos: Vec3,
    window_size: Vec2,
    camera_zoom: f32,
    tile_size: f32,
    grid_width: i32,
    grid_height: i32,
    buffer: i32,
) -> (i32, i32, i32, i32)
```

This function:
- Converts screen corners to world space
- Converts world space corners to grid coordinates
- Finds the bounding box of these grid coordinates
- Applies a zoom-scaled buffer
- Adds diagonal buffer for isometric projection

### Key Improvements

1. **Buffer scaling formula**:
   ```rust
   let zoom_buffer = if scale < 1.0 {
       let zoom_factor = 1.0 / scale;
       (buffer as f32 * zoom_factor.sqrt() * 2.0).ceil() as i32
   } else {
       buffer
   };
   ```

2. **Diagonal buffer**:
   ```rust
   let diagonal_buffer = (zoom_buffer as f32 * 1.5).ceil() as i32;
   ```

3. **Increased base buffer**: From 5 to 10 tiles

## Results

Before fix:
- Zoom 1.0: visible tiles (73,73)-(127,127) = 55x55 tiles
- Zoom 0.95: visible tiles (72,72)-(128,128) = 57x57 tiles

After fix:
- Zoom 1.0: visible tiles (54,54)-(146,146) = 93x93 tiles
- Zoom 0.95: visible tiles (25,25)-(175,175) = 151x151 tiles
- Zoom 0.2: visible tiles (0,0)-(199,199) = entire map

The culling now properly ensures that all visible tiles are rendered with no gaps at the edges, while still optimizing performance by not rendering truly off-screen tiles.