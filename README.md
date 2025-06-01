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

## License

[Add license information]