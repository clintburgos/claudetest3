//! Tile Interaction Systems - Mouse hovering and selection
//!
//! This module handles user interaction with tiles:
//! - Hover detection using raycasting
//! - Click selection with visual feedback
//! - State management for highlighted/selected tiles
//!
//! # Design Notes
//! - Uses Bevy's picking/raycasting for accurate tile detection
//! - Maintains single selection (only one tile selected at a time)
//! - Visual feedback through color modulation

use super::components::{Tile, TileBiome, TileHighlighted, TilePosition, TileSelected};
use crate::game::GameState;
use crate::ui::world::camera::IsometricCamera;
use crate::ui::world::grid::coordinates::screen_to_grid;
use crate::ui::world::grid::{GridConfig, GridMap};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

/// Resource to track the currently hovered tile
#[derive(Resource, Default, Debug)]
pub struct HoveredTile {
    pub entity: Option<Entity>,
    pub position: Option<TilePosition>,
}

/// Resource to track the currently selected tile
#[derive(Resource, Default, Debug)]
pub struct SelectedTile {
    pub entity: Option<Entity>,
    pub position: Option<TilePosition>,
}

/// System to detect which tile is under the mouse cursor
pub fn tile_hover_detection_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<IsometricCamera>>,
    grid_config: Res<GridConfig>,
    grid_map: Res<GridMap>,
    mut hovered_tile: ResMut<HoveredTile>,
    mut commands: Commands,
    tile_query: Query<Entity, With<TileHighlighted>>,
) {
    let Ok(window) = windows.single() else {
        return;
    };

    let Some(cursor_position) = window.cursor_position() else {
        // Clear hover if cursor is not in window
        clear_hover(&mut hovered_tile, &mut commands, &tile_query);
        return;
    };

    let Ok((camera, camera_transform)) = camera_query.single() else {
        return;
    };

    // Convert screen coordinates to world coordinates
    let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) else {
        clear_hover(&mut hovered_tile, &mut commands, &tile_query);
        return;
    };

    // Convert world coordinates to grid coordinates
    let grid_pos = screen_to_grid(world_pos, grid_config.tile_size);

    // Check if the grid position has a tile
    if let Some(entity) = grid_map.get_tile(grid_pos.x, grid_pos.y) {
        // Update hover state if different tile
        if hovered_tile.entity != Some(entity) {
            // Remove highlight from previous tile
            if let Some(prev_entity) = hovered_tile.entity {
                commands.entity(prev_entity).remove::<TileHighlighted>();
            }

            // Add highlight to new tile
            commands.entity(entity).insert(TileHighlighted);

            // Update resource
            hovered_tile.entity = Some(entity);
            hovered_tile.position = Some(TilePosition::ground(grid_pos.x, grid_pos.y));
        }
    } else {
        // Clear hover if no tile at position
        clear_hover(&mut hovered_tile, &mut commands, &tile_query);
    }
}

/// Helper function to clear hover state
fn clear_hover(
    hovered_tile: &mut ResMut<HoveredTile>,
    commands: &mut Commands,
    tile_query: &Query<Entity, With<TileHighlighted>>,
) {
    if let Some(entity) = hovered_tile.entity {
        if let Ok(entity) = tile_query.get(entity) {
            commands.entity(entity).remove::<TileHighlighted>();
        }
    }
    hovered_tile.entity = None;
    hovered_tile.position = None;
}

/// System to handle tile selection on mouse click
pub fn tile_selection_system(
    mouse_input: Res<ButtonInput<MouseButton>>,
    hovered_tile: Res<HoveredTile>,
    mut selected_tile: ResMut<SelectedTile>,
    mut commands: Commands,
    selected_query: Query<Entity, With<TileSelected>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // Clear previous selection
    for entity in selected_query.iter() {
        commands.entity(entity).remove::<TileSelected>();
    }

    // Select the hovered tile if any
    if let Some(entity) = hovered_tile.entity {
        commands.entity(entity).insert(TileSelected);
        selected_tile.entity = Some(entity);
        selected_tile.position = hovered_tile.position;
    } else {
        // Clear selection if clicking on empty space
        selected_tile.entity = None;
        selected_tile.position = None;
    }
}

/// Type alias for tile visual update query
type TileVisualQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static MeshMaterial2d<ColorMaterial>,
        &'static TileBiome,
        Has<TileHighlighted>,
        Has<TileSelected>,
    ),
    With<Tile>,
>;

/// System to update visual appearance of highlighted and selected tiles
pub fn tile_highlight_visual_system(
    tiles: TileVisualQuery,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (material_handle, biome, is_highlighted, is_selected) in tiles.iter() {
        if let Some(material) = materials.get_mut(&material_handle.0) {
            // Calculate the target color based on current state
            let mut target_color = biome.color();

            // Apply highlight effect
            if is_highlighted {
                target_color = target_color.lighter(0.3);
            }

            // Apply selection effect (can stack with highlight)
            if is_selected {
                let linear = target_color.to_linear();
                target_color = Color::linear_rgba(
                    linear.red * 0.8,
                    linear.green * 0.9,
                    linear.blue * 1.2,
                    linear.alpha,
                );
            }

            // Only update material if color is different
            if material.color != target_color {
                material.color = target_color;
            }
        }
    }
}

// Note: tile_selection_visual_system has been merged into tile_highlight_visual_system
// to properly handle color restoration from the base biome color

/// Plugin that adds tile interaction systems
pub struct TileInteractionPlugin;

impl Plugin for TileInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredTile>()
            .init_resource::<SelectedTile>()
            .add_systems(
                Update,
                (
                    tile_hover_detection_system,
                    tile_selection_system,
                    tile_highlight_visual_system,
                )
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::world::grid::GridConfig;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_hovered_tile_resource_default() {
        let hovered = HoveredTile::default();
        assert_eq!(hovered.entity, None);
        assert_eq!(hovered.position, None);
    }

    #[test]
    fn test_selected_tile_resource_default() {
        let selected = SelectedTile::default();
        assert_eq!(selected.entity, None);
        assert_eq!(selected.position, None);
    }

    #[test]
    fn test_tile_hover_detection_no_window() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<HoveredTile>();
        app.insert_resource(GridConfig::default());
        app.insert_resource(GridMap::new(10, 10));

        // Run system without window - should handle gracefully
        app.add_systems(Update, tile_hover_detection_system);
        app.update();

        let hovered = app.world().resource::<HoveredTile>();
        assert_eq!(hovered.entity, None);
    }

    #[test]
    fn test_tile_selection_system_no_hover() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<HoveredTile>();
        app.init_resource::<SelectedTile>();
        app.init_resource::<ButtonInput<MouseButton>>();

        // Simulate mouse click
        let mut mouse_input = app.world_mut().resource_mut::<ButtonInput<MouseButton>>();
        mouse_input.press(MouseButton::Left);

        app.add_systems(Update, tile_selection_system);
        app.update();

        let selected = app.world().resource::<SelectedTile>();
        assert_eq!(selected.entity, None);
    }

    #[test]
    fn test_tile_selection_system_with_hover() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a test entity
        let test_entity = app
            .world_mut()
            .spawn((Tile, TilePosition::ground(5, 5)))
            .id();

        // Set up hover state
        app.insert_resource(HoveredTile {
            entity: Some(test_entity),
            position: Some(TilePosition::ground(5, 5)),
        });
        app.init_resource::<SelectedTile>();
        app.init_resource::<ButtonInput<MouseButton>>();

        // Simulate mouse click
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.add_systems(Update, tile_selection_system);
        app.update();

        // Check that tile was selected
        let selected = app.world().resource::<SelectedTile>();
        assert_eq!(selected.entity, Some(test_entity));
        assert_eq!(selected.position, Some(TilePosition::ground(5, 5)));

        // Check that component was added
        assert!(app.world().get::<TileSelected>(test_entity).is_some());
    }

    // Note: Visual tests have been removed since we switched from sprites to mesh materials
    // The visual system now works with ColorMaterial assets instead of sprite colors

    #[test]
    fn test_clear_hover_removes_component() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a highlighted tile
        let entity = app.world_mut().spawn((Tile, TileHighlighted)).id();

        app.insert_resource(HoveredTile {
            entity: Some(entity),
            position: Some(TilePosition::ground(0, 0)),
        });

        // Run the clear hover logic inline since we can't easily test the helper function
        let _ = app.world_mut().run_system_once(
            |mut hovered_tile: ResMut<HoveredTile>, mut commands: Commands| {
                if let Some(entity) = hovered_tile.entity {
                    commands.entity(entity).remove::<TileHighlighted>();
                }
                hovered_tile.entity = None;
                hovered_tile.position = None;
            },
        );

        // Check that highlight was removed
        assert!(app.world().get::<TileHighlighted>(entity).is_none());
    }

    #[test]
    fn test_tile_interaction_plugin() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, TileInteractionPlugin));

        // Check resources were initialized
        assert!(app.world().get_resource::<HoveredTile>().is_some());
        assert!(app.world().get_resource::<SelectedTile>().is_some());
    }

    #[test]
    fn test_multiple_selection_clears_previous() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create two tiles
        let entity1 = app.world_mut().spawn((Tile, TileSelected)).id();
        let entity2 = app.world_mut().spawn(Tile).id();

        // Set up hover state for entity2
        app.insert_resource(HoveredTile {
            entity: Some(entity2),
            position: Some(TilePosition::ground(1, 1)),
        });
        app.init_resource::<SelectedTile>();
        app.init_resource::<ButtonInput<MouseButton>>();

        // Simulate mouse click
        app.world_mut()
            .resource_mut::<ButtonInput<MouseButton>>()
            .press(MouseButton::Left);

        app.add_systems(Update, tile_selection_system);
        app.update();

        // Check that entity1 no longer has TileSelected
        assert!(app.world().get::<TileSelected>(entity1).is_none());
        // Check that entity2 now has TileSelected
        assert!(app.world().get::<TileSelected>(entity2).is_some());
    }
}
