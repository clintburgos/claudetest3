use bevy::prelude::*;
use claudetest3::game::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, setup_debug_ui)
        .add_systems(OnEnter(GameState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(GameState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(GameState::Playing), spawn_playing_ui)
        .add_systems(Update, debug_ui_entities)
        .add_systems(Update, handle_spacebar)
        .run();
}

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct PlayingUI;

#[derive(Component)]
struct DebugText;

fn setup_debug_ui(mut commands: Commands) {
    // Debug text to show entity count
    commands.spawn((
        Text::new("Press SPACE to toggle between MainMenu and Playing states\nUI Entities: 0"),
        TextFont {
            font_size: 20.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        DebugText,
    ));
}

fn spawn_main_menu(mut commands: Commands) {
    info!("Spawning main menu UI");
    
    // Main menu with black background
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("MAIN MENU"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

fn despawn_main_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    info!("Despawning main menu UI");
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spawn_playing_ui(mut commands: Commands) {
    info!("Spawning playing UI");
    
    // Playing state UI - should be visible
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 1.0, 0.0, 0.5)),
        Text::new("PLAYING STATE"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::WHITE),
        PlayingUI,
    ));
}

fn debug_ui_entities(
    mut text_query: Query<&mut Text, With<DebugText>>,
    main_menu_query: Query<Entity, With<MainMenuUI>>,
    playing_query: Query<Entity, With<PlayingUI>>,
    all_ui_query: Query<Entity, With<Node>>,
    state: Res<State<GameState>>,
) {
    if let Ok(mut text) = text_query.single_mut() {
        let main_menu_count = main_menu_query.iter().count();
        let playing_count = playing_query.iter().count();
        let total_ui = all_ui_query.iter().count();
        
        text.0 = format!(
            "Press SPACE to toggle between MainMenu and Playing states\n\
             Current State: {:?}\n\
             MainMenu UI entities: {}\n\
             Playing UI entities: {}\n\
             Total UI entities: {}",
            state.get(),
            main_menu_count,
            playing_count,
            total_ui
        );
    }
}

fn handle_spacebar(
    keyboard: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        match current_state.get() {
            GameState::MainMenu => {
                info!("Transitioning to Playing state");
                next_state.set(GameState::Playing);
            }
            GameState::Playing => {
                info!("Transitioning to MainMenu state");
                next_state.set(GameState::MainMenu);
            }
            _ => {}
        }
    }
}