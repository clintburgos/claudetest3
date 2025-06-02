# Camera Zoom Bug Analysis

## Issue Summary

The camera zoom system has an inverted calculation in the view culling system, causing:
1. **Zooming out makes map edges recede** - The entire map fits within the viewport when zoomed out too far
2. **Zooming in makes the map disappear** - Tiles are culled when the visible area becomes too small

## Root Cause

The bug is in `/src/ui/world/tiles/view_culling.rs` in the `calculate_visible_bounds` function:

```rust
// CURRENT (INCORRECT):
let visible_width = window.width() / camera_scale;
let visible_height = window.height() / camera_scale;
```

This should be:

```rust
// CORRECT:
let visible_width = window.width() * camera_scale;
let visible_height = window.height() * camera_scale;
```

## Why This Is Wrong

### Camera System Design
- `CameraState.zoom`: 
  - `1.0` = default view
  - `>1.0` = zoomed IN (closer view, see less)
  - `<1.0` = zoomed OUT (farther view, see more)
- `Transform.scale` is set to match `zoom` value

### Current Behavior (Inverted)
When **zooming IN** (scale > 1.0):
- `visible_area = window_size / scale` → SMALLER visible area
- But zooming in should show MORE detail, not less area
- Result: Tiles disappear because visible area is too small

When **zooming OUT** (scale < 1.0):
- `visible_area = window_size / scale` → LARGER visible area
- But zooming out should show LESS detail, more overview
- Result: Entire map fits in view, edges recede

### Expected Behavior
The visible area calculation should work opposite to the current implementation:
- Zooming IN → smaller world area visible (but rendered larger)
- Zooming OUT → larger world area visible (but rendered smaller)

## Test Results

Running the debug test shows:

```
=== CURRENT BEHAVIOR (INVERTED) ===
Very zoomed in (zoom = 0.25):
  Visible area: 5120x2880 pixels
  Visible tiles: 160.0x90.0

Very zoomed out (zoom = 4):
  Visible area: 320x180 pixels
  Visible tiles: 10.0x5.6
```

This is backwards! When zoomed in, we see MORE tiles instead of fewer.

## Fix

The fix is simple - change the division to multiplication in `calculate_visible_bounds`:

```rust
// In src/ui/world/tiles/view_culling.rs, lines 80-81
let visible_width = window.width() * camera_scale;
let visible_height = window.height() * camera_scale;
```

## Alternative Approach

If the current behavior is intentional (where scale represents "how much of the world fits in view"), then:
1. Rename variables for clarity (e.g., `view_scale` instead of `zoom`)
2. Update documentation to reflect this
3. Invert the zoom controls (Q/E keys and mouse wheel)

However, the current naming and documentation suggest the fix above is the correct approach.