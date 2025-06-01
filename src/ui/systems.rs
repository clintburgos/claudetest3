use super::components::*;
use bevy::prelude::*;

pub fn handle_button_interactions(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &InteractiveButton),
        (Changed<Interaction>, With<Button>),
    >,
) {
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
