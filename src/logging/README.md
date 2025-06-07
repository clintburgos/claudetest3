# Logging Module

This module provides comprehensive logging and debugging capabilities for the Bevy application, capturing all user interactions, system events, and periodic screenshots to help debug and recreate issues.

## Structure

- `mod.rs` - Main module file that exports the LoggingPlugin
- `components.rs` - Defines log data structures (LogEntry, LogCategory, LogBuffer, LogEvent)
- `systems.rs` - Contains systems for capturing various events (keypresses, mouse, game events)
- `writer.rs` - Handles writing logs to files with proper formatting
- `screenshot.rs` - Manages automatic screenshot capture every second

## Features

### Event Categories
- **Keypress** - All keyboard input events (press/release)
- **MouseClick** - Mouse button events with position
- **MouseMove** - Mouse movement (throttled to reduce spam)
- **GameEvent** - Custom game-specific events
- **SystemEvent** - System-level events
- **PerformanceMetric** - FPS and frame timing data
- **StateChange** - Game state transitions
- **Screenshot** - Automatic screenshot capture events
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

The logging system is automatically initialized when the LoggingPlugin is added to the app. Each run creates a new session directory under `logs/session_<timestamp>/` containing:
- `log.txt` - The main log file with all events
- `screenshot_<timestamp>.png` - Screenshots captured every second

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

Logs are stored in session directories `logs/session_<timestamp>/` and can be used to:
- Debug issues by seeing exact sequence of events with visual context
- Recreate bugs by replaying user inputs alongside screenshots
- Analyze performance patterns
- Track game state changes
- Visually verify UI state at any moment

## Integration Points

The logging system integrates with:
- Bevy's input system for capturing all user inputs
- Game state systems for tracking state changes
- Performance monitoring for FPS tracking
- Custom game events through the LogEvent system
- Bevy's screenshot system for periodic visual capture

## Screenshot Capture

Screenshots are automatically captured every 1 second and saved to the session directory with timestamps. Each screenshot event is logged in the main log file, making it easy to correlate visual state with user actions and system events.