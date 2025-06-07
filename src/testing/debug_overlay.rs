use bevy::prelude::*;
use crate::ui::world::camera::components::{IsometricCamera, CameraState};
use crate::ui::world::tiles::components::Tile;

#[derive(Resource)]
pub struct DebugOverlayEnabled(pub bool);

impl Default for DebugOverlayEnabled {
    fn default() -> Self {
        Self(false)
    }
}

#[derive(Component)]
pub struct DebugOverlay;

pub fn setup_debug_overlay(mut commands: Commands) {
    commands.init_resource::<DebugOverlayEnabled>();
}

pub fn toggle_debug_overlay(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut debug_enabled: ResMut<DebugOverlayEnabled>,
    mut visibility_query: Query<&mut Visibility, With<DebugOverlay>>,
    mut commands: Commands,
    existing_overlay: Query<Entity, With<DebugOverlay>>,
) {
    if keyboard.just_pressed(KeyCode::F8) {
        debug_enabled.0 = !debug_enabled.0;
        
        if debug_enabled.0 {
            info!("Debug overlay enabled");
            if existing_overlay.is_empty() {
                spawn_debug_overlay(&mut commands);
            }
            for mut visibility in visibility_query.iter_mut() {
                *visibility = Visibility::Visible;
            }
        } else {
            info!("Debug overlay disabled");
            for mut visibility in visibility_query.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn spawn_debug_overlay(commands: &mut Commands) {
    // Debug info panel
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(70.0),
            left: Val::Px(10.0),
            width: Val::Px(300.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DebugOverlay,
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Debug Info"),
            TextFont {
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::WHITE),
            DebugOverlay,
        ));
    });
}

pub fn update_debug_info(
    debug_enabled: Res<DebugOverlayEnabled>,
    camera_query: Query<(&Transform, &CameraState), With<IsometricCamera>>,
    tile_query: Query<&Transform, With<Tile>>,
    mut text_query: Query<&mut Text, With<DebugOverlay>>,
    time: Res<Time>,
) {
    if !debug_enabled.0 {
        return;
    }
    
    if let Ok((cam_transform, camera_state)) = camera_query.single() {
        let visible_tiles = tile_query.iter().count();
        let fps = 1.0 / time.delta_secs();
        
        for mut text in text_query.iter_mut() {
            text.0 = format!(
                "Debug Info\n\
                FPS: {:.1}\n\
                Camera Pos: ({:.1}, {:.1})\n\
                Zoom: {:.2}\n\
                Visible Tiles: {}\n\
                Press F8 to toggle",
                fps,
                cam_transform.translation.x,
                cam_transform.translation.y,
                camera_state.zoom,
                visible_tiles
            );
        }
    }
}