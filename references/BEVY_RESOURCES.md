# Bevy 0.16 Resources & References

## Official Resources

### Core Documentation
- **Official Website**: https://bevyengine.org/
- **API Documentation**: https://docs.rs/bevy/0.16/bevy/
- **Bevy Book**: https://bevyengine.org/learn/book/introduction/
- **GitHub Repository**: https://github.com/bevyengine/bevy
- **Release Notes**: https://bevyengine.org/news/bevy-0-16/

### Official Examples
- **Examples Repository**: https://github.com/bevyengine/bevy/tree/v0.16.0/examples
- **Example Browser**: https://bevyengine.org/examples/

## Migration Guides

### From 0.15 to 0.16
- **Official Migration Guide**: https://bevyengine.org/learn/migration-guides/0-15-to-0-16/
- **Breaking Changes**: Check CHANGELOG.md in the Bevy repository

### Key Breaking Changes in 0.16
- UI bundles deprecated (NodeBundle, ButtonBundle, etc.)
- Style merged into Node component
- Text API completely redesigned
- Required Components pattern introduced
- Many component renamings

## Community Resources

### Learning Materials
- **Bevy Cheatbook**: https://bevy-cheatbook.github.io/
- **Bevy Assets**: https://bevyengine.org/assets/
- **Bevy Tutorial Series**: Various YouTube channels and blogs

### Community Hubs
- **Discord Server**: https://discord.gg/bevy
- **Reddit**: https://www.reddit.com/r/bevy/
- **GitHub Discussions**: https://github.com/bevyengine/bevy/discussions

## Key Concepts & APIs

### UI System (0.16)
```rust
// Node-based UI (no more bundles)
commands.spawn(Node {
    width: Val::Percent(100.0),
    height: Val::Px(50.0),
    ..default()
});

// Text API
commands.spawn((
    Text::new("Hello"),
    TextFont { font_size: 30.0, ..default() },
    TextColor(Color::WHITE),
));

// Buttons
commands.spawn((
    Button,
    Node { ..default() },
    BackgroundColor(Color::BLUE),
));
```

### ECS Fundamentals
- **Entities**: Unique identifiers
- **Components**: Data attached to entities
- **Systems**: Functions that process components
- **Resources**: Global data
- **Commands**: Deferred world mutations

### Core Plugins
- `DefaultPlugins`: Standard set of plugins
- `MinimalPlugins`: Bare minimum for headless apps
- `WindowPlugin`: Window configuration
- `AssetPlugin`: Asset loading system
- `RenderPlugin`: Rendering pipeline

## Common Patterns

### State Management
```rust
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    #[default]
    Menu,
    Playing,
    Paused,
}

app.init_state::<GameState>()
   .add_systems(OnEnter(GameState::Menu), setup_menu)
   .add_systems(OnExit(GameState::Menu), cleanup_menu);
```

### Asset Loading
```rust
#[derive(Resource)]
struct GameAssets {
    font: Handle<Font>,
    texture: Handle<Image>,
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(GameAssets {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        texture: asset_server.load("textures/icon.png"),
    });
}
```

### Input Handling
```rust
fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if keyboard.pressed(KeyCode::KeyW) {
        // Move forward
    }
    if mouse.just_pressed(MouseButton::Left) {
        // Fire weapon
    }
}
```

## Performance Tips

### Optimization Strategies
1. **Use Changed/Added filters**: `Query<&Transform, Changed<Transform>>`
2. **Batch operations**: Minimize archetype moves
3. **Profile with Tracy**: Enable `trace` feature
4. **Parallelize systems**: Use `.par_iter()` for queries
5. **Optimize assets**: Compress textures, use atlases

### Debug Features
```toml
[dependencies]
bevy = { version = "0.16", features = ["trace", "debug_asset_server"] }
```

## Ecosystem Crates

### Essential Extensions
- **bevy_egui**: Dear ImGui integration
- **bevy_rapier**: Physics engine
- **bevy_kira_audio**: Advanced audio
- **bevy_asset_loader**: Asset loading states
- **bevy_inspector_egui**: Runtime inspector

### UI Libraries
- **bevy_ui_navigation**: Keyboard/gamepad navigation
- **bevy_cosmic_edit**: Text editing widget
- **bevy_lunex**: Alternative UI system

### Graphics Extensions
- **bevy_hanabi**: Particle system
- **bevy_atmosphere**: Sky rendering
- **bevy_water**: Water simulation
- **bevy_vfx_bag**: Visual effects collection

## Development Tools

### IDE Support
- **VS Code**: rust-analyzer + Bevy snippets
- **IntelliJ/CLion**: Rust plugin
- **Zed**: Built-in Rust support

### Debugging
- **bevy_inspector_egui**: Runtime entity inspector
- **bevy_prototype_debug_lines**: Debug line drawing
- **bevy_screen_diagnostics**: FPS and performance overlay

### Build Configuration
```toml
# Faster builds
[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3

# Release with debug info
[profile.release]
debug = true

# Maximum optimization
[profile.release]
lto = "thin"
```

## Platform-Specific

### WASM/Web
- Use `wasm-bindgen` features
- Configure canvas in index.html
- Handle browser-specific input

### Mobile
- iOS: Configure Info.plist
- Android: Configure AndroidManifest.xml
- Touch input handling

### Console
- Limited official support
- Community efforts ongoing

## Best Practices

### Architecture
1. **Use plugins**: Modularize functionality
2. **Marker components**: Zero-sized type tags
3. **Event-driven**: Communicate via events
4. **State machines**: Manage game flow
5. **Asset handles**: Don't store assets directly

### Code Organization
```
src/
├── main.rs           # App setup
├── lib.rs            # Plugin definitions
├── systems/          # System functions
├── components/       # Component definitions
├── resources/        # Resource types
├── events/           # Event types
└── ui/              # UI-specific code
```

## Troubleshooting

### Common Issues
1. **Panic on spawn**: Missing required components
2. **Black screen**: Check camera setup
3. **Performance**: Enable release optimizations
4. **Asset not found**: Check paths and working directory
5. **System order**: Use labels and ordering constraints

### Getting Help
1. Search existing issues on GitHub
2. Ask on Discord (include minimal example)
3. Post on Reddit with [Help] tag
4. Check Bevy Cheatbook troubleshooting section

## Learning Path

### Beginner
1. Official Bevy Book
2. Examples (start with `hello_world`)
3. Make Pong or Breakout
4. Learn ECS patterns

### Intermediate
1. Study complex examples
2. Create custom systems
3. Implement UI layouts
4. Add audio and particles

### Advanced
1. Custom render pipelines
2. Write Bevy plugins
3. Optimize performance
4. Contribute to Bevy

## Stay Updated

- **Blog**: https://bevyengine.org/news/
- **Twitter/X**: @BevyEngine
- **Mastodon**: @bevyengine@mastodon.social
- **YouTube**: Search "Bevy Engine"
- **Newsletter**: Bevy community newsletters

---

*Last updated for Bevy 0.16.0*