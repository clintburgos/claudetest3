//! Info Panel - Displays information about selected tiles
//!
//! This module creates a side panel that shows details about
//! the currently selected tile, including coordinates and biome type.

use super::components::*;
use crate::game::GameState;
use crate::ui::world::tiles::{SelectedTile, TileBiome, TilePosition};
use bevy::prelude::*;

/// Plugin for the tile information panel
pub struct InfoPanelPlugin;

impl Plugin for InfoPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_info_panel)
            .add_systems(OnExit(GameState::Playing), despawn_info_panel)
            .add_systems(
                Update,
                update_tile_info.run_if(in_state(GameState::Playing)),
            );
    }
}

/// Spawns the info panel UI
fn spawn_info_panel(mut commands: Commands) {
    // Info panel container - right side of screen
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                top: Val::Px(70.0), // Below top bar
                width: Val::Px(250.0),
                height: Val::Px(300.0),
                padding: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.1, 0.1, 0.1, 0.9)),
            InfoPanel,
        ))
        .with_children(|parent| {
            // Panel title
            parent.spawn((
                Text::new("Tile Information"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Separator
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(2.0),
                    margin: UiRect::vertical(Val::Px(5.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.5, 0.5, 0.5, 0.5)),
            ));

            // No selection text (default)
            parent.spawn((
                Text::new("No tile selected"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::srgba(0.7, 0.7, 0.7, 1.0)),
                TileInfoText,
            ));

            // Coordinates section
            parent.spawn((
                Text::new("Coordinates: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                CoordinatesText,
            ));

            // Biome section
            parent.spawn((
                Text::new("Biome: --"),
                TextFont {
                    font_size: 16.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                BiomeText,
            ));
        });
}

/// Type aliases for text queries
type InfoTextQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<TileInfoText>,
        Without<CoordinatesText>,
        Without<BiomeText>,
    ),
>;
type CoordsTextQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<CoordinatesText>,
        Without<TileInfoText>,
        Without<BiomeText>,
    ),
>;
type BiomeTextQuery<'w, 's> = Query<
    'w,
    's,
    &'static mut Text,
    (
        With<BiomeText>,
        Without<TileInfoText>,
        Without<CoordinatesText>,
    ),
>;

/// Updates tile information display based on selected tile
fn update_tile_info(
    selected_tile: Res<SelectedTile>,
    tile_query: Query<(&TilePosition, &TileBiome)>,
    mut info_text_query: InfoTextQuery,
    mut coords_text_query: CoordsTextQuery,
    mut biome_text_query: BiomeTextQuery,
) {
    // Only update if selection has changed
    if !selected_tile.is_changed() {
        return;
    }
    if let Ok(mut info_text) = info_text_query.single_mut() {
        if let Some(entity) = selected_tile.entity {
            // Update main info text
            info_text.0 = "Tile Selected".to_string();

            // Get tile data
            if let Ok((position, biome)) = tile_query.get(entity) {
                // Update coordinates
                if let Ok(mut coords_text) = coords_text_query.single_mut() {
                    coords_text.0 = format!("Coordinates: ({}, {})", position.x, position.y);
                }

                // Update biome with colored text
                if let Ok(mut biome_text) = biome_text_query.single_mut() {
                    biome_text.0 = format!("Biome: {}", biome_to_string(biome));
                }
            }
        } else {
            // No selection
            info_text.0 = "No tile selected".to_string();

            if let Ok(mut coords_text) = coords_text_query.single_mut() {
                coords_text.0 = "Coordinates: --".to_string();
            }

            if let Ok(mut biome_text) = biome_text_query.single_mut() {
                biome_text.0 = "Biome: --".to_string();
            }
        }
    }
}

/// Convert biome enum to display string
fn biome_to_string(biome: &TileBiome) -> &'static str {
    match biome {
        TileBiome::Plain => "Plains",
        TileBiome::Forest => "Forest",
        TileBiome::Coast => "Coast",
        TileBiome::Water => "Water",
        TileBiome::Desert => "Desert",
        TileBiome::Mountain => "Mountain",
    }
}

/// Despawns the info panel when leaving the playing state
fn despawn_info_panel(mut commands: Commands, panel_query: Query<Entity, With<InfoPanel>>) {
    for entity in panel_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_biome_to_string() {
        assert_eq!(biome_to_string(&TileBiome::Plain), "Plains");
        assert_eq!(biome_to_string(&TileBiome::Forest), "Forest");
        assert_eq!(biome_to_string(&TileBiome::Coast), "Coast");
        assert_eq!(biome_to_string(&TileBiome::Water), "Water");
        assert_eq!(biome_to_string(&TileBiome::Desert), "Desert");
        assert_eq!(biome_to_string(&TileBiome::Mountain), "Mountain");
    }

    #[test]
    fn test_spawn_info_panel() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        app.world_mut()
            .run_system_once(spawn_info_panel)
            .expect("System should run");

        // Check that info panel was created
        let panel_count = app
            .world_mut()
            .query::<&InfoPanel>()
            .iter(&app.world())
            .count();
        assert_eq!(panel_count, 1, "Should create info panel");

        // Check that text components were created
        let info_text = app
            .world_mut()
            .query::<&TileInfoText>()
            .iter(&app.world())
            .count();
        assert_eq!(info_text, 1, "Should create tile info text");

        let coords_text = app
            .world_mut()
            .query::<&CoordinatesText>()
            .iter(&app.world())
            .count();
        assert_eq!(coords_text, 1, "Should create coordinates text");

        let biome_text = app
            .world_mut()
            .query::<&BiomeText>()
            .iter(&app.world())
            .count();
        assert_eq!(biome_text, 1, "Should create biome text");
    }

    #[test]
    fn test_despawn_info_panel() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create info panel
        let panel_entity = app.world_mut().spawn(InfoPanel).id();

        app.world_mut()
            .run_system_once(despawn_info_panel)
            .expect("System should run");

        // Check that info panel was removed
        assert!(!app.world().entities().contains(panel_entity));
    }

    #[test]
    fn test_update_tile_info_no_selection() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.init_resource::<SelectedTile>();

        // Create text components
        app.world_mut().spawn((Text::new("Selected"), TileInfoText));
        app.world_mut()
            .spawn((Text::new("Coords"), CoordinatesText));
        app.world_mut().spawn((Text::new("Biome"), BiomeText));

        app.world_mut()
            .run_system_once(update_tile_info)
            .expect("System should run");

        // Check that texts were updated to show no selection
        let info_text = app
            .world_mut()
            .query::<(&Text, &TileInfoText)>()
            .iter(&app.world())
            .next()
            .unwrap()
            .0;
        assert_eq!(info_text.0, "No tile selected");
    }

    #[test]
    fn test_update_tile_info_with_selection() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a selected tile
        let tile_entity = app
            .world_mut()
            .spawn((TilePosition::ground(5, 7), TileBiome::Forest))
            .id();

        // Set selected tile
        let mut selected = SelectedTile::default();
        selected.entity = Some(tile_entity);
        selected.position = Some(TilePosition::ground(5, 7));
        app.insert_resource(selected);

        // Create text components
        app.world_mut()
            .spawn((Text::new("No selection"), TileInfoText));
        app.world_mut().spawn((Text::new("--"), CoordinatesText));
        app.world_mut().spawn((Text::new("--"), BiomeText));

        app.world_mut()
            .run_system_once(update_tile_info)
            .expect("System should run");

        // Check that texts were updated
        let info_text = app
            .world_mut()
            .query::<(&Text, &TileInfoText)>()
            .iter(&app.world())
            .next()
            .unwrap()
            .0;
        assert_eq!(info_text.0, "Tile Selected");

        let coords_text = app
            .world_mut()
            .query::<(&Text, &CoordinatesText)>()
            .iter(&app.world())
            .next()
            .unwrap()
            .0;
        assert_eq!(coords_text.0, "Coordinates: (5, 7)");

        let biome_text = app
            .world_mut()
            .query::<(&Text, &BiomeText)>()
            .iter(&app.world())
            .next()
            .unwrap()
            .0;
        assert_eq!(biome_text.0, "Biome: Forest");
    }
}
