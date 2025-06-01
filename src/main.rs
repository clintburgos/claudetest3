use bevy::prelude::*;
use claudetest3::ui;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Complex UI".to_string(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ui::world::WorldPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .run();
}

fn setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn(Camera2d);

    // Root UI container
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        })
        .with_children(|parent| {
            // Header
            parent
                .spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Px(60.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Complex Bevy UI"),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            // Main content area
            parent
                .spawn(Node {
                    width: Val::Percent(100.0),
                    flex_grow: 1.0,
                    flex_direction: FlexDirection::Row,
                    ..default()
                })
                .with_children(|parent| {
                    // Sidebar
                    parent
                        .spawn((
                            Node {
                                width: Val::Px(200.0),
                                height: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(10.0)),
                                flex_direction: FlexDirection::Column,
                                row_gap: Val::Px(10.0),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
                        ))
                        .with_children(|parent| {
                            // Sidebar buttons
                            for i in 0..5 {
                                parent
                                    .spawn((
                                        Button,
                                        Node {
                                            width: Val::Percent(100.0),
                                            height: Val::Px(40.0),
                                            justify_content: JustifyContent::Center,
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                    ))
                                    .with_children(|parent| {
                                        parent.spawn((
                                            Text::new(format!("Option {}", i + 1)),
                                            TextFont {
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::WHITE),
                                        ));
                                    });
                            }
                        });

                    // Main panel
                    parent
                        .spawn((
                            Node {
                                flex_grow: 1.0,
                                height: Val::Percent(100.0),
                                padding: UiRect::all(Val::Px(20.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
                        ))
                        .with_children(|parent| {
                            parent.spawn((
                                Text::new("Main Content Area"),
                                TextFont {
                                    font_size: 24.0,
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                });
        });
}

fn ui_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.4, 0.4, 0.4));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
        }
    }
}
