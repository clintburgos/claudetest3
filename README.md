# Bevy Isometric World Project

A Bevy 0.16 project featuring a complex UI system with an isometric tile-based world map.

## Project Structure

```
src/
├── main.rs           # Application entry point and setup
├── lib.rs            # Module declarations
└── ui/               # UI systems and components
    ├── README.md     # UI module documentation
    └── world/        # Isometric world implementation
        └── README.md # World system documentation
```

## Quick Links

- [Master Index](./INDEX.md) - Navigation hub for all documentation
- [Bevy Resources](./BEVY_RESOURCES.md) - Bevy 0.16 reference guide
- [Claude Guide](./CLAUDE.md) - AI assistant development guide
- [Design Document](./docs/ISOMETRIC_MAP_DESIGN.md) - Isometric world architecture

## Getting Started

```bash
# Run the application
cargo run

# Run with release optimizations
cargo run --release

# Check compilation
cargo check

# Run tests
cargo test
```

## Development

Before committing:
```bash
cargo fmt      # Format code
cargo clippy   # Lint
cargo test     # Run tests
```

## Features

- **UI System**: Modular component-based UI with Bevy 0.16
- **Isometric World**: Procedurally generated tile-based world map
- **Camera Controls**: Pan and zoom with keyboard/mouse
- **Biome System**: Multiple terrain types with realistic placement
- **Debug Logging**: Comprehensive logging system with automatic screenshots

## Debug Logging

The application includes a comprehensive logging system that captures:
- All keyboard and mouse input events
- System events and performance metrics (FPS)
- Automatic screenshots every second

### Log Location

Logs are stored in `logs/session_<timestamp>/` directories containing:
- `log.txt` - Main log file with all events
- `screenshot_<timestamp>.png` - Screenshots captured every second

### Log Format

```
[timestamp_ms] Frame # | CATEGORY | message | data
```

Example:
```
[1749309362865] Frame 59 | KEYPRESS | Key W pressed | data: keycode: KeyW
[1749309362865] Frame 59 | SCREENSHOT | Screenshot captured | data: path: logs/session_1749309361/screenshot_1749309362865.png
```

### Using Logs for Debugging

1. Each run creates a new session directory
2. Screenshots help visualize the state at any point in time
3. Input events can be correlated with screenshots to reproduce issues
4. Performance metrics help identify optimization opportunities

## License

[Add license information]