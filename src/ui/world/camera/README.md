# Camera Module

Isometric camera controls and view management.

## Files

### `mod.rs`
- **Purpose**: Module exports and IsometricCameraPlugin
- **Plugin**: `IsometricCameraPlugin` - Registers camera systems

### `components.rs`
- **Purpose**: Camera-related components
- **Components**:
  - `IsometricCamera` - Marker for the main camera
  - `CameraState` - Zoom level, velocity, constraints

### `controls.rs`
- **Purpose**: Input handling systems
- **Systems**:
  - `keyboard_camera_system` - WASD/Arrow key movement
  - `mouse_camera_system` - Mouse/trackpad controls
  - `zoom_system` - Zoom in/out handling

### `constraints.rs`
- **Purpose**: Camera boundary enforcement
- **Systems**:
  - `apply_camera_constraints_system` - Keep camera in bounds
- **Functions**:
  - `calculate_bounds` - Determine valid camera area

## Controls

### Keyboard
- **WASD/Arrows**: Pan camera
- **Q**: Zoom in
- **E**: Zoom out

### Mouse/Trackpad
- **Scroll wheel**: Zoom
- **Two-finger drag**: Pan (trackpad)
- **Pinch**: Zoom (trackpad)

## Camera Properties

- Orthographic projection
- Isometric angle (no rotation needed)
- Smooth movement with momentum
- Constrained to map boundaries
- Zoom limits (0.5x to 2.0x)