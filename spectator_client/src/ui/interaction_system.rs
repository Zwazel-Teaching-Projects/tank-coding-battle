use bevy::{color::palettes::css::RED, prelude::*};

use super::{HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PRESSED_BUTTON_COLOR};

#[derive(Event)]
pub struct ButtonPressedTrigger;

pub fn interaction_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (entity, interaction, mut color, mut border_color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON_COLOR.into();
                border_color.0 = RED.into();

                commands.trigger_targets(ButtonPressedTrigger, entity);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON_COLOR.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON_COLOR.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}
