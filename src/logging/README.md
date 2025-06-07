# Logging Module

This module provides comprehensive logging and debugging capabilities for the Bevy application, capturing all user interactions and system events to help debug and recreate issues.

## Structure

- `mod.rs` - Main module file that exports the LoggingPlugin
- `components.rs` - Defines log data structures (LogEntry, LogCategory, LogBuffer, LogEvent)
- `systems.rs` - Contains systems for capturing various events (keypresses, mouse, game events)
- `writer.rs` - Handles writing logs to files with proper formatting

## Features

### Event Categories
- **Keypress** - All keyboard input events (press/release)
- **MouseClick** - Mouse button events with position
- **MouseMove** - Mouse movement (throttled to reduce spam)
- **GameEvent** - Custom game-specific events
- **SystemEvent** - System-level events
- **PerformanceMetric** - FPS and frame timing data
- **StateChange** - Game state transitions
- **Custom** - User-defined categories

### Log Format
```
[timestamp_ms] Frame # | CATEGORY | message | data
```

Example:
```
[1704134400000] Frame 120 | KEYPRESS | Key W pressed | data: keycode: KeyW
[1704134400016] Frame 121 | MOUSE_CLICK | Left mouse button pressed | data: position: (640.5, 360.2)
[1704134401000] Frame 180 | PERFORMANCE | FPS: 59.8 | data: delta_time: 16.722ms
```

## Usage

The logging system is automatically initialized when the LoggingPlugin is added to the app. Logs are written to the `logs/` directory with timestamps.

### Adding Custom Log Events

```rust
// Send a custom log event
log_events.send(LogEvent {
    category: LogCategory::GameEvent,
    message: "Player spawned".to_string(),
    data: Some("position: (100, 200)".to_string()),
});
```

### Reading Logs

Logs are stored in `logs/debug_log_<timestamp>.txt` and can be used to:
- Debug issues by seeing exact sequence of events
- Recreate bugs by replaying user inputs
- Analyze performance patterns
- Track game state changes

## Integration Points

The logging system integrates with:
- Bevy's input system for capturing all user inputs
- Game state systems for tracking state changes
- Performance monitoring for FPS tracking
- Custom game events through the LogEvent system