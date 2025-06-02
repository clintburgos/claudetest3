use super::components::*;
use bevy::prelude::*;

/// Type alias for button interaction query
type InteractiveButtonQuery<'w, 's> = Query<
    'w,
    's,
    (
        &'static Interaction,
        &'static mut BackgroundColor,
        &'static InteractiveButton,
    ),
    (Changed<Interaction>, With<Button>),
>;

pub fn handle_button_interactions(mut interaction_query: InteractiveButtonQuery) {
    for (interaction, mut color, button) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.5, 0.5, 0.5));
                handle_button_action(button.action);
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

fn handle_button_action(action: ButtonAction) {
    match action {
        ButtonAction::Navigate(index) => {
            info!("Navigating to section {}", index);
        }
        ButtonAction::OpenDialog => {
            info!("Opening dialog");
        }
        ButtonAction::CloseDialog => {
            info!("Closing dialog");
        }
        ButtonAction::Custom(id) => {
            info!("Custom action {}", id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::app::App;
    use bevy::ecs::system::RunSystemOnce;

    #[test]
    fn test_handle_button_action_navigate() {
        // Just verify it doesn't panic
        handle_button_action(ButtonAction::Navigate(5));
    }

    #[test]
    fn test_handle_button_action_open_dialog() {
        handle_button_action(ButtonAction::OpenDialog);
    }

    #[test]
    fn test_handle_button_action_close_dialog() {
        handle_button_action(ButtonAction::CloseDialog);
    }

    #[test]
    fn test_handle_button_action_custom() {
        handle_button_action(ButtonAction::Custom(42));
    }

    #[test]
    fn test_handle_button_interactions_pressed() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a button with pressed interaction
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            Button,
            InteractiveButton {
                action: ButtonAction::OpenDialog,
            },
        ));

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");
    }

    #[test]
    fn test_handle_button_interactions_hovered() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a button with hover interaction
        let button = app
            .world_mut()
            .spawn((
                Interaction::Hovered,
                BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
                Button,
                InteractiveButton {
                    action: ButtonAction::CloseDialog,
                },
            ))
            .id();

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that background color changed
        let bg_color = app.world().get::<BackgroundColor>(button).unwrap();
        assert_eq!(bg_color.0, Color::srgb(0.4, 0.4, 0.4));
    }

    #[test]
    fn test_handle_button_interactions_none() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a button with no interaction
        let button = app
            .world_mut()
            .spawn((
                Interaction::None,
                BackgroundColor(Color::srgb(1.0, 1.0, 1.0)),
                Button,
                InteractiveButton {
                    action: ButtonAction::Navigate(0),
                },
            ))
            .id();

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run");

        // Check that background color changed
        let bg_color = app.world().get::<BackgroundColor>(button).unwrap();
        assert_eq!(bg_color.0, Color::srgb(0.3, 0.3, 0.3));
    }

    #[test]
    fn test_handle_button_interactions_multiple_buttons() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create multiple buttons with different interactions
        app.world_mut().spawn((
            Interaction::Pressed,
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            Button,
            InteractiveButton {
                action: ButtonAction::Custom(1),
            },
        ));

        app.world_mut().spawn((
            Interaction::Hovered,
            BackgroundColor(Color::srgb(0.0, 0.0, 0.0)),
            Button,
            InteractiveButton {
                action: ButtonAction::Custom(2),
            },
        ));

        app.world_mut()
            .run_system_once(handle_button_interactions)
            .expect("System should run with multiple buttons");
    }
}
