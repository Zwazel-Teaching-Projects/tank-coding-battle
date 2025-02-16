use bevy::prelude::*;
use shared::{
    asset_handling::config::ClientConfigSystemParam,
    networking::messages::{
        message_container::{
            MessageContainer, MessageErrorTrigger, MessageTarget, NetworkMessageType,
        },
        message_data::{
            message_error_types::ErrorMessageTypes, start_game_config::StartGameConfig,
        },
        message_queue::ImmediateOutMessageQueue,
    },
};

use crate::game_state::MyGameState;

use super::{interaction_system::ButtonPressedTrigger, NORMAL_BUTTON_COLOR};

#[derive(Debug, Component, Default, Reflect)]
#[reflect(Component)]
pub struct StartGameButton;

pub fn spawn_start_game_button(mut commands: Commands) {
    commands
        .spawn((
            StateScoped(MyGameState::SettingUp),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
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
                .observe(button_pressed)
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

fn button_pressed(
    _: Trigger<ButtonPressedTrigger>,
    mut message_queue: Query<&mut ImmediateOutMessageQueue>,
    mut state: ResMut<NextState<MyGameState>>,
    client_config: ClientConfigSystemParam,
) {
    let client_config = client_config.client_config();

    state.set(MyGameState::GameToldToStart);

    if let Ok(mut message_queue) = message_queue.get_single_mut() {
        message_queue.push_back(MessageContainer::new(
            MessageTarget::ToLobbyDirectly,
            NetworkMessageType::StartGame(StartGameConfig {
                fill_empty_slots_with_dummies: client_config.fill_empty_slots_with_dummies,
            }),
        ));
    }
}

pub fn start_game_error_handling(
    trigger: Trigger<MessageErrorTrigger>,
    mut state: ResMut<NextState<MyGameState>>,
) {
    match &(**trigger.event()) {
        ErrorMessageTypes::LobbyNotReadyToStart(message) => {
            error!("Failed to start game: {}", message);
            state.set(MyGameState::SettingUp);
        }
        _ => {}
    }
}
