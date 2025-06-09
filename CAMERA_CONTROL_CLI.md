# Camera Control CLI Tools

This document describes the command-line tools for controlling camera movements in the Bevy game.

## Overview

Two binary programs have been created to script camera movements:

1. **camera_script** - Flexible scripting tool with command-line arguments
2. **camera_tour** - Predefined camera tours

Both programs execute camera movements and automatically exit when complete.

## camera_script

A flexible command-line tool for scripting custom camera movements.

### Usage

```bash
cargo run --bin camera_script -- [actions...]
```

### Available Actions

- `--pan, -p <dx> <dy>` - Pan camera by dx, dy world units
- `--zoom, -z <factor>` - Zoom by factor (2.0 = zoom in, 0.5 = zoom out)
- `--set-zoom, -sz <level>` - Set absolute zoom level
- `--wait, -w <seconds>` - Wait for specified seconds
- `--center, -c <x> <y>` - Center camera on tile coordinates

### Examples

```bash
# Pan right 100 units, wait 2 seconds, then zoom in
cargo run --bin camera_script -- --pan 100 0 --wait 2 --zoom 2

# Center on tile (50, 50), wait, then zoom out
cargo run --bin camera_script -- --center 50 50 --wait 1 --zoom 0.5

# Complex sequence
cargo run --bin camera_script -- --center 100 100 --wait 1 --zoom 0.5 --wait 1 --pan -500 -500 --wait 1 --set-zoom 2.0
```

## camera_tour

Simpler tool with predefined camera tours.

### Usage

```bash
cargo run --bin camera_tour -- <tour_type>
```

### Available Tours

- `overview` - Zoom out to see entire map, then zoom back in
- `corners` - Visit all four corners of the map
- `zoom` - Demonstrate different zoom levels
- `edges` - Scroll along the edges of the map

### Examples

```bash
# Run the overview tour
cargo run --bin camera_tour -- overview

# Visit all corners
cargo run --bin camera_tour -- corners

# Demonstrate zoom levels
cargo run --bin camera_tour -- zoom
```

## Implementation Details

Both tools:
- Skip the main menu and go directly to the game
- Use reduced logging for cleaner output
- Execute actions with timers
- Exit automatically using `std::process::exit(0)` after completion

The tools are useful for:
- Automated testing of camera functionality
- Creating demo videos
- Debugging camera-related issues
- Performance testing at different zoom levels