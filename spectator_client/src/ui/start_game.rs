use bevy::prelude::*;

use super::NORMAL_BUTTON_COLOR;

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct StartGameButton;

pub fn spawn_start_game_button(mut commands: Commands) {
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|commands| {
            commands
                .spawn((
                    Button,
                    StartGameButton,
                    Node {
                        width: Val::Px(250.0),
                        height: Val::Px(75.0),
                        border: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(Color::BLACK),
                    BorderRadius::MAX,
                    BackgroundColor(NORMAL_BUTTON_COLOR),
                ))
                .with_child((
                    Text::new("Start Game"),
                    TextFont {
                        font_size: 30.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
        });
}
