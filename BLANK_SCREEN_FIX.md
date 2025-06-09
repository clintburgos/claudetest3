# Blank Screen Issues and Solutions

## Problem Summary
The game sometimes shows a blank screen, particularly:
- On startup when skipping the menu
- After large zoom changes
- During rapid camera movements
- When taking screenshots immediately after state changes

## Root Causes

### 1. Tile Spawning Rate Limit
- Current limit: 5000 tiles per frame
- Full map has 40,000 tiles
- At minimum zoom, it takes 8 frames to spawn all tiles
- This creates a visible "pop-in" effect

### 2. Frame Timing Issues
- Camera setup happens in `OnEnter(GameState::Playing)`
- Tile culling runs in `Update` 
- There's a 1-2 frame delay before tiles appear

### 3. Screenshot Timing
- Screenshots captured during state transitions show blank screens
- The logging system takes screenshots automatically on certain events

## Immediate Fixes

### 1. Increase Tile Spawn Rate
Edit `/src/constants.rs`:
```rust
pub const DEFAULT_TILES_PER_FRAME: usize = 10000; // Increased from 5000
```

### 2. Add Initial Tile Spawn
Add a system that runs once on startup to spawn initial tiles immediately:
```rust
.add_systems(OnEnter(GameState::Playing), spawn_initial_tiles.after(setup_camera))
```

### 3. Add Loading State
Instead of going directly to Playing, add a brief Loading state:
```rust
GameState::MainMenu -> GameState::Loading -> GameState::Playing
```

### 4. Delay Screenshot Capture
Add a timer before taking screenshots to ensure tiles are spawned.

## Long-term Solutions

### 1. Predictive Tile Loading
Pre-spawn tiles in the direction of camera movement.

### 2. Level-of-Detail System
Use lower detail tiles when zoomed out far.

### 3. Async Tile Loading
Load tiles in background threads to prevent frame drops.

### 4. Frustum Culling Optimization
Better predict which tiles will be visible next frame.

## Testing the Fix

1. Run with increased spawn rate:
   ```bash
   cargo run --bin claudetest3
   ```

2. Test rapid zoom changes:
   ```bash
   cargo run --bin camera_script -- --set-zoom 0.084 --wait 1 --set-zoom 5.0
   ```

3. Test startup timing:
   ```bash
   cargo run --bin camera_tour -- overview
   ```