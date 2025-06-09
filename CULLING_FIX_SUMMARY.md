# View Culling Fix Summary

## Problem
When zooming out, tiles at the edges of the screen were being culled even though they were visible. When culling was disabled, these tiles would suddenly appear, proving they should have been rendered all along.

## Root Cause
The culling algorithm was too aggressive in removing tiles:
1. **Insufficient buffer**: Default buffer was only 5 tiles
2. **Wrong buffer calculation**: When zoomed out, the buffer was being REDUCED instead of increased
3. **No margin on visible bounds**: The visible rectangle calculation didn't account for isometric tile overlap
4. **Buffer not passed correctly**: The dynamic buffer wasn't being used in find_visible_tiles

## Fixes Applied

### 1. Increased Default Buffer
```rust
// Before: DEFAULT_BUFFER_TILES: i32 = 5
// After:  DEFAULT_BUFFER_TILES: i32 = 10
```

### 2. Fixed Dynamic Buffer Calculation
```rust
// Before: Reduced buffer when zoomed out
(base_buffer as f32 * camera_scale.sqrt()).max(1.0).ceil() as i32

// After: Increased buffer when zoomed out
let zoom_factor = 1.0 / camera_scale;
(base_buffer as f32 * zoom_factor.sqrt()).max(base_buffer as f32).ceil() as i32
```

### 3. Added Margin to Visible Bounds
```rust
// Added extra margin to account for isometric tile overlap
let margin = grid_config.tile_size * 2.0;
let visible_min = Vec2::new(
    cam_x - visible_width * 0.5 - margin, 
    cam_y - visible_height * 0.5 - margin
);
```

### 4. Fixed Buffer Parameter
The dynamic_buffer is now correctly passed to find_visible_tiles instead of base_buffer.

## Results
- At zoom 1.0: Visible tiles expanded from (73,73,127,127) to (62,62,138,138)
- At extreme zoom out: Properly renders entire map (0,0)-(199,199)
- Dynamic buffer scales appropriately: 10 at zoom 1.0, up to 26 at extreme zoom out
- No more visible gaps at screen edges when culling is enabled

## Verification
Run `cargo run --bin gradual_zoom_out` and observe:
1. No blank edges appear during zoom out
2. Tiles fill the entire visible screen area
3. Toggling culling (press C) shows minimal difference in visible tiles