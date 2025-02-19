use bevy::prelude::*;
use shared::{
    asset_handling::config::ServerConfigSystemParam,
    networking::{
        lobby_management::{
            lobby_management::LobbyManagementSystemParam, AwaitingFirstContact,
            PlayerWantsToJoinLobbyTrigger,
        },
        messages::{
            message_container::{
                FirstContactTrigger, MessageContainer, MessageTarget, NetworkMessageType,
            },
            message_data::{first_contact::ClientType, message_error_types::ErrorMessageTypes},
            message_queue::ImmediateOutMessageQueue,
        },
    },
};

use crate::networking::handle_clients::lib::{ClientDisconnectedTrigger, MyNetworkClient};

pub fn handle_awaiting_first_contact(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut AwaitingFirstContact)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in clients.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            warn!("Client {:?} timed out waiting for first contact", entity);
            commands.trigger(ClientDisconnectedTrigger(entity));
        }
    }
}

// Proof of concept for handling a message using an observer
// We can even make targeted ones and only trigger for specific clients!
pub fn handle_first_contact_message(
    trigger: Trigger<FirstContactTrigger>,
    mut commands: Commands,
    mut lobby_management: LobbyManagementSystemParam,
    mut clients: Query<(Entity, &mut MyNetworkClient, &mut ImmediateOutMessageQueue)>,
    server_config: ServerConfigSystemParam,
) {
    let server_config = server_config.server_config();
    let message = &trigger.message;
    let sender = trigger.sender.unwrap();

    // Update the client's state
    if let Ok((client_entity, mut client, mut message_queue)) = clients.get_mut(sender) {
        client.name = Some(message.bot_name.clone());
        if let Some(assigned_spawn_point) = message.bot_assigned_spawn_point {
            client.assigned_spawn_point = Some(assigned_spawn_point);
        }

        commands
            .entity(client_entity)
            .insert(message.client_type.clone());

        match message.client_type {
            ClientType::Player => {
                if let Some(tank_type) = &message.tank_type {
                    commands.entity(client_entity).insert(tank_type.clone());
                } else {
                    error!("Player client did not specify a tank type");
                    message_queue.push_back(MessageContainer::new(
                        MessageTarget::Client(client_entity),
                        NetworkMessageType::MessageError(ErrorMessageTypes::InvalidFirstContact(
                            "Player client did not specify a tank type".to_string(),
                        )),
                    ));

                    return;
                }
            }
            _ => {}
        }
    }

    // get or insert lobby
    match lobby_management.get_or_insert_lobby_entity(
        &message.lobby_name,
        message.map_name.as_deref(),
        &mut commands,
        server_config,
    ) {
        Ok(lobby_entity) => {
            commands.trigger(PlayerWantsToJoinLobbyTrigger {
                player: sender,
                lobby: lobby_entity,
                player_type: message.client_type.clone(),
                team_name: message.team_name.clone(),
                player_name: message.bot_name.clone(),
            });
        }
        Err(e) => {
            error!(
                "Error getting or inserting lobby entity: {:?} for sender: {:?}",
                e, sender
            );
        }
    }
}
