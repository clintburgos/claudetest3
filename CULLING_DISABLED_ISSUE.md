# Culling Disabled Issue

## Problem
When view culling is disabled, NO tiles render at all. This is because:

1. The `view_culling_system` returns early when `culling_config.enabled` is false
2. This prevents ANY tile spawning from occurring
3. The `spawn_all_tiles_system` was added to handle this case, but it runs AFTER view_culling_system

## Root Cause
In `/src/ui/world/tiles/view_culling.rs`:
```rust
// Skip if culling is disabled
if !culling_config.enabled {
    return;  // This causes NO tiles to spawn!
}
```

## Solution Implemented
1. Created `spawn_all_tiles_system` to spawn all tiles when culling is disabled
2. Added it to the system chain in `MapGenerationPlugin`:
```rust
.add_systems(
    Update,
    (
        view_culling_system,
        spawn_all_tiles::spawn_all_tiles_system,
    )
        .chain()
        .in_set(WorldSystems::TileUpdate)
        .run_if(in_state(GameState::Playing)),
)
```

## Current Status
- The spawn_all_tiles_system is implemented and compiles
- It checks if culling is disabled and spawns all tiles
- System ordering ensures it runs after view_culling_system

## Testing
Created several test programs:
- `no_culling_test.rs` - Starts with culling disabled
- `test_culling_toggle.rs` - Toggles culling after 3 seconds
- Toggle culling manually with the C key in any program

## Usage
Press C to toggle culling on/off during gameplay. When disabled, all 40,000 tiles will be spawned at once.