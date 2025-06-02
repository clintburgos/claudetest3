//! Performance monitoring overlay that displays FPS and entity count
//!
//! This module provides a real-time performance overlay that shows:
//! - Current FPS
//! - Frame time in milliseconds
//! - Total entity count
//! - Tile entity count
//! - View culling status

use crate::constants::ui::performance::*;
use crate::ui::{
    styles::UiColors,
    world::tiles::{SpawnedTiles, ViewCullingConfig},
};
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;

/// Marker component for the performance overlay root
#[derive(Component)]
pub struct PerformanceOverlay;

/// Marker component for FPS text
#[derive(Component)]
pub struct FpsText;

/// Marker component for entity count text
#[derive(Component)]
pub struct EntityCountText;

/// Marker component for tile count text
#[derive(Component)]
pub struct TileCountText;

/// Marker component for culling status text
#[derive(Component)]
pub struct CullingStatusText;

/// Spawn the performance overlay UI
pub fn spawn_performance_overlay(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(MARGIN),
                right: Val::Px(MARGIN),
                padding: UiRect::all(Val::Px(PADDING)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(ROW_GAP),
                ..default()
            },
            BackgroundColor(UiColors::BACKGROUND.with_alpha(BACKGROUND_ALPHA)),
            PerformanceOverlay,
        ))
        .with_children(|parent| {
            // FPS text
            parent.spawn((
                Text::new("FPS: --"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                TextColor(UiColors::TEXT_PRIMARY),
                FpsText,
            ));

            // Entity count text
            parent.spawn((
                Text::new("Entities: --"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                TextColor(UiColors::TEXT_PRIMARY),
                EntityCountText,
            ));

            // Tile count text
            parent.spawn((
                Text::new("Tiles: --"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                TextColor(UiColors::TEXT_PRIMARY),
                TileCountText,
            ));

            // Culling status text
            parent.spawn((
                Text::new("Culling: --"),
                TextFont {
                    font_size: FONT_SIZE,
                    ..default()
                },
                TextColor(UiColors::TEXT_PRIMARY),
                CullingStatusText,
            ));
        });
}

/// Despawn the performance overlay
pub fn despawn_performance_overlay(
    mut commands: Commands,
    overlay_query: Query<Entity, With<PerformanceOverlay>>,
) {
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }
}

/// Update FPS display
pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut fps_text_query: Query<&mut Text, With<FpsText>>,
) {
    if let Ok(mut text) = fps_text_query.single_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                text.0 = format!("FPS: {:.0}", value);
            }
        }
    }
}

/// Update entity count display
pub fn update_entity_count_text(
    mut entity_count_query: Query<&mut Text, With<EntityCountText>>,
    all_entities: Query<Entity>,
) {
    if let Ok(mut text) = entity_count_query.single_mut() {
        let count = all_entities.iter().count();
        text.0 = format!("Entities: {}", count);
    }
}

/// Update tile count display
pub fn update_tile_count_text(
    mut tile_count_query: Query<&mut Text, With<TileCountText>>,
    spawned_tiles: Res<SpawnedTiles>,
) {
    if let Ok(mut text) = tile_count_query.single_mut() {
        text.0 = format!("Tiles: {}", spawned_tiles.count());
    }
}

/// Update culling status display
pub fn update_culling_status_text(
    mut culling_text_query: Query<&mut Text, With<CullingStatusText>>,
    culling_config: Res<ViewCullingConfig>,
) {
    if let Ok(mut text) = culling_text_query.single_mut() {
        text.0 = format!(
            "Culling: {} (buffer: {})",
            if culling_config.enabled { "ON" } else { "OFF" },
            culling_config.buffer_tiles
        );
    }
}

/// Plugin that adds the performance overlay
pub struct PerformanceOverlayPlugin;

impl Plugin for PerformanceOverlayPlugin {
    fn build(&self, app: &mut App) {
        use crate::game::GameState;

        // Add frame time diagnostics if not already added
        if !app.is_plugin_added::<FrameTimeDiagnosticsPlugin>() {
            app.add_plugins(FrameTimeDiagnosticsPlugin::default());
        }

        app.add_systems(OnEnter(GameState::Playing), spawn_performance_overlay)
            .add_systems(OnExit(GameState::Playing), despawn_performance_overlay)
            .add_systems(
                Update,
                (
                    update_fps_text,
                    update_entity_count_text,
                    update_tile_count_text,
                    update_culling_status_text,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_spawn_performance_overlay() {
        let mut app = App::new();

        app.world_mut()
            .run_system_once(spawn_performance_overlay)
            .expect("System should run");

        // Check overlay was spawned
        let overlay_count = app
            .world_mut()
            .query::<&PerformanceOverlay>()
            .iter(app.world())
            .count();
        assert_eq!(overlay_count, 1);

        // Check text components were spawned
        assert_eq!(
            app.world_mut()
                .query::<&FpsText>()
                .iter(app.world())
                .count(),
            1
        );
        assert_eq!(
            app.world_mut()
                .query::<&EntityCountText>()
                .iter(app.world())
                .count(),
            1
        );
        assert_eq!(
            app.world_mut()
                .query::<&TileCountText>()
                .iter(app.world())
                .count(),
            1
        );
        assert_eq!(
            app.world_mut()
                .query::<&CullingStatusText>()
                .iter(app.world())
                .count(),
            1
        );
    }

    #[test]
    fn test_despawn_performance_overlay() {
        let mut app = App::new();

        // First spawn the overlay
        app.world_mut()
            .run_system_once(spawn_performance_overlay)
            .expect("System should run");

        // Then despawn it
        app.world_mut()
            .run_system_once(despawn_performance_overlay)
            .expect("System should run");

        // Check everything was despawned
        assert_eq!(
            app.world_mut()
                .query::<&PerformanceOverlay>()
                .iter(app.world())
                .count(),
            0
        );
        assert_eq!(
            app.world_mut()
                .query::<&FpsText>()
                .iter(app.world())
                .count(),
            0
        );
    }
}
