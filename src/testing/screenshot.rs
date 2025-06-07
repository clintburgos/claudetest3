use bevy::prelude::*;
use bevy::render::view::screenshot::Capturing;

/// Component to track screenshot status in UI
#[derive(Component)]
pub struct ScreenshotIndicator;

/// Timer for hiding screenshot indicator
#[derive(Resource)]
pub struct ScreenshotIndicatorTimer(pub Timer);

impl Default for ScreenshotIndicatorTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(2.0, TimerMode::Once))
    }
}

/// Setup screenshot indicator UI
pub fn setup_screenshot_indicator(mut commands: Commands) {
    // Initialize timer resource
    commands.init_resource::<ScreenshotIndicatorTimer>();

    // Create screenshot indicator (initially hidden)
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                bottom: Val::Px(10.0),
                right: Val::Px(10.0),
                padding: UiRect::all(Val::Px(8.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            Visibility::Hidden,
            ScreenshotIndicator,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("📸 Taking screenshot..."),
                TextFont {
                    font_size: 14.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

/// Update screenshot indicator visibility based on screenshot state
pub fn update_screenshot_indicator(
    capturing_query: Query<Entity, With<Capturing>>,
    mut indicator_query: Query<&mut Visibility, With<ScreenshotIndicator>>,
    mut timer: ResMut<ScreenshotIndicatorTimer>,
    time: Res<Time>,
) {
    // Check if any screenshots are being captured
    let is_capturing = !capturing_query.is_empty();

    if is_capturing {
        // Show indicator and reset timer
        timer.0.reset();
        for mut visibility in indicator_query.iter_mut() {
            *visibility = Visibility::Visible;
        }
    } else {
        // Update timer
        timer.0.tick(time.delta());

        // Hide indicator after timer expires
        if timer.0.finished() {
            for mut visibility in indicator_query.iter_mut() {
                *visibility = Visibility::Hidden;
            }
        }
    }
}
