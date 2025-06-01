# UI Module

This module contains all user interface components and systems for the application.

## Structure

```
ui/
├── mod.rs         # Module exports and plugin registration
├── components.rs  # UI component definitions
├── systems.rs     # UI interaction systems
├── styles.rs      # Style constants and helpers
└── world/         # Isometric world subsystem
```

## Responsibilities

### `mod.rs`
- **Purpose**: Module organization and exports
- **Exports**: Public components, systems, and styles
- **Plugin**: Registers `UiPlugin` if applicable

### `components.rs`
- **Purpose**: Define UI-specific components
- **Components**:
  - `MainMenu` - Main menu marker
  - `Sidebar` - Sidebar panel marker
  - `ContentPanel` - Main content area marker
  - `Header` - Header bar marker
  - `InteractiveButton` - Button with action handling
  - `ButtonAction` - Enum of possible button actions

### `systems.rs`
- **Purpose**: Handle UI interactions and updates
- **Systems**:
  - `handle_button_interactions` - Process button clicks/hovers
  - Additional UI update systems

### `styles.rs`
- **Purpose**: Centralized styling configuration
- **Contents**:
  - Layout constants (dimensions, padding)
  - Color scheme (`UiColors`)
  - Helper functions for common styles

## Usage

```rust
use crate::ui::{InteractiveButton, ButtonAction, UiColors};

// Spawn a button
commands.spawn((
    Button,
    Node { /* ... */ },
    InteractiveButton {
        action: ButtonAction::OpenDialog,
    },
));
```

## Submodules

- [`world/`](./world/README.md) - Isometric world rendering and interaction