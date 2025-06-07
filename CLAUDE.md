# Claude AI Assistant Guide

This document provides context and guidelines for Claude when working on this Bevy project.

## 📚 Quick Reference

**Master Index**: See [INDEX.md](./INDEX.md) for all available documentation.

**Key Resources**:
- [BEVY_RESOURCES.md](./BEVY_RESOURCES.md) - Bevy 0.16 documentation and guides
- [CHEATSHEET.md](./CHEATSHEET.md) - Quick code snippets *(planned)*
- [API_REFERENCE.md](./API_REFERENCE.md) - API quick lookup *(planned)*

## 🎯 Project Overview

**Project**: Complex Bevy 0.16 Graphical Interface
**Language**: Rust
**Framework**: Bevy 0.16
**Purpose**: Building a sophisticated UI-heavy application

### Current State
- ✅ Basic Bevy 0.16 setup complete
- ✅ UI module structure created
- ✅ Complex layout with header, sidebar, and content area
- ✅ Interactive button system
- 🚧 Additional UI components needed

## 🛠️ Development Guidelines

### Bevy 0.16 Specifics
1. **No More Bundles**: Use direct component spawning
   ```rust
   // ❌ Old way
   commands.spawn(NodeBundle { ... });
   
   // ✅ New way
   commands.spawn(Node { ... });
   ```

2. **Text API**: Use new component-based approach
   ```rust
   commands.spawn((
       Text::new("Hello"),
       TextFont { font_size: 30.0, ..default() },
       TextColor(Color::WHITE),
   ));
   ```

3. **Style Merged into Node**: No separate Style component
   ```rust
   Node {
       width: Val::Percent(100.0),
       height: Val::Px(50.0),
       ..default()
   }
   ```

### Code Style
- Use `cargo fmt` before committing
- Follow Rust naming conventions
- Prefer composition over inheritance
- Use marker components for entity identification
- Group related systems in modules

### Project Structure
```
src/
├── main.rs          # App setup and main systems
├── lib.rs           # Module declarations
└── ui/              # UI components
    ├── mod.rs       # UI module exports
    ├── components.rs # Component definitions
    ├── systems.rs    # UI systems
    └── styles.rs     # Style constants and helpers
```

## 🚀 Common Tasks

### Adding a New UI Component
1. Define component in `ui/components.rs`
2. Create spawn function in appropriate module
3. Add interaction system if needed
4. Update styles in `ui/styles.rs`

### Running the Project
```bash
# Development build
cargo run

# Release build
cargo run --release

# Check compilation
cargo check

# Run tests
cargo test
```

### Debugging
- Enable Bevy's debug features in Cargo.toml
- Use `bevy_inspector_egui` for runtime inspection
- Add `.after(LogPlugin::default())` to see system ordering
- Check logs in `logs/session_<timestamp>/` for detailed debugging info

## 📋 Lint and Type Check Commands

**IMPORTANT**: Run these before committing:
```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Type check
cargo check
```

## 🔍 Debug Logging System

The application includes a comprehensive logging system that automatically captures:

### What's Logged
- **Keypresses**: All keyboard input with press/release states
- **Mouse Events**: Clicks with position and movement (throttled)
- **Performance**: FPS metrics logged every second
- **Screenshots**: Automatic captures every second
- **Game Events**: Custom events can be added via LogEvent

### Log Location
```
logs/
└── session_<timestamp>/
    ├── log.txt                    # Main event log
    └── screenshot_<timestamp>.png # Visual captures
```

### Interpreting Logs
```
[timestamp_ms] Frame # | CATEGORY | message | data
```

Examples:
- `[1749309362865] Frame 59 | KEYPRESS | Key W pressed | data: keycode: KeyW`
- `[1749309362865] Frame 59 | SCREENSHOT | Screenshot captured | data: path: logs/...`
- `[1749309362865] Frame 59 | PERFORMANCE | FPS: 60.8 | data: delta_time: 16.439ms`

### Using for Debugging
1. Find the session directory for your run
2. Open `log.txt` to see the event timeline
3. Cross-reference with screenshots to see visual state
4. Use timestamps to correlate events with visual changes

## 🐛 Known Issues & Workarounds

### Issue: ChildBuilder Import
**Problem**: `ChildBuilder` type not directly accessible in Bevy 0.16
**Solution**: Use inline closures with `with_children` instead of separate functions

### Issue: Slow Compilation
**Solution**: Already configured with optimized dev profile in Cargo.toml

## 🔄 Migration Notes

When upgrading Bevy versions:
1. Check [official migration guide](https://bevyengine.org/learn/migration-guides/)
2. Update Cargo.toml dependency
3. Fix breaking changes (usually UI and Text API)
4. Test all interactive components

## 📝 Documentation Standards

When adding new features:
1. Add inline documentation for public items
2. Update relevant .md files in docs/
3. Include examples in documentation
4. Update INDEX.md if adding new doc files

## 🎨 UI Development Patterns

### Component Hierarchy
```
Root Node (100% x 100%)
├── Header (100% x 60px)
│   └── Title Text
├── Content Area (100% x flex)
    ├── Sidebar (200px x 100%)
    │   └── Button List
    └── Main Panel (flex x 100%)
        └── Content
```

### Interactive Elements
- All buttons should have hover/pressed states
- Use `Interaction` component for mouse events
- Provide visual feedback for all interactions
- Keep consistent styling via `ui/styles.rs`

## 🔧 Performance Considerations

1. **Batch UI Updates**: Modify multiple UI elements in single system
2. **Use Changed Filters**: Only process modified components
3. **Minimize State Checks**: Cache state when possible
4. **Profile First**: Don't optimize without profiling

## 📚 Learning Resources

See [BEVY_RESOURCES.md](./BEVY_RESOURCES.md) for:
- Official documentation
- Community tutorials
- Example projects
- Discord/forums for help

## 🚦 Git Workflow

1. **Before Committing**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```

2. **Commit Messages**: Follow conventional commits
   - `feat:` New features
   - `fix:` Bug fixes
   - `docs:` Documentation changes
   - `style:` Code style changes
   - `refactor:` Code refactoring
   - `test:` Test additions/changes
   - `chore:` Build/tool changes

## 🎯 Next Steps

Current priorities:
1. Implement additional UI components
2. Add state management system
3. Create data binding system
4. Implement theming support
5. Add animation system

---

*Last updated: 2024-12-31 | Bevy 0.16*