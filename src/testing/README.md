# Testing Module

This module provides interactive testing capabilities for the Bevy Isometric World application. It's only included in debug builds to help verify the program works correctly.

## Features

### Keyboard Controls
- **F1** - Show help/controls in console
- **F5** - Reset camera to center position
- **F6** - Zoom out to minimum (see entire map)
- **F7** - Zoom in to maximum (detailed view)
- **F8** - Toggle debug overlay
- **F9** - Cycle through test scenarios
- **F12** - Screenshot reminder (use OS tools)

### Debug Overlay
When enabled (F8), displays:
- Current FPS
- Camera position
- Current zoom level
- Number of visible tiles

### Test Scenarios
Cycle through with F9:
1. **Normal** - Default view
2. **MaxZoom** - Maximum zoom level
3. **MinZoom** - Minimum zoom level
4. **EdgeOfMap** - Camera at map edge
5. **CenterOfMap** - Camera centered

## Usage

### Running Interactive Tests
```bash
./test_interactive.sh
```

### Running Visual Verification
```bash
./run_visual_tests.sh
```

### Taking Screenshots
- **macOS**: Cmd+Shift+4 (area) or Cmd+Shift+3 (full screen)
- **Windows**: Win+Shift+S or PrtScn
- **Linux**: PrtScn or use screenshot tools

## Files

- `mod.rs` - Main testing plugin and input handling
- `camera_controls.rs` - Camera test controls (F5-F7, F9)
- `debug_overlay.rs` - Debug information overlay (F8)

## Verifying Functionality

1. **Camera Movement**: Use arrow keys or WASD to pan around
2. **Camera Zoom**: Use mouse wheel or Q/E keys
3. **Tile Interaction**: Hover and click on tiles
4. **UI Elements**: Check header and panels display correctly
5. **Performance**: Monitor FPS in debug overlay

## Adding New Tests

To add new test functionality:
1. Add new keyboard handler in `handle_test_inputs`
2. Create new test scenario in `TestScenario` enum
3. Update help text in F1 handler
4. Document in this README