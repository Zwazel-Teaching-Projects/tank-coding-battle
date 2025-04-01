use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::components::CollisionLayer,
        flag::{FlagCarrier, FlagState},
    },
    networking::{
        lobby_management::{lobby_management::LobbyManagementSystemParam, MyLobby},
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::entity_data::EntityDataWrapper,
            message_queue::OutMessageQueue,
        },
    },
};

use crate::{
    gameplay::capture_the_flag::triggers::FlagGotDroppedTrigger,
    networking::handle_clients::lib::MyNetworkClient,
};

#[derive(Debug, Reflect, Event)]
pub struct ClientDiedTrigger;

pub fn client_died(
    trigger: Trigger<ClientDiedTrigger>,
    lobby: LobbyManagementSystemParam,
    mut lobby_message_queue: Query<&mut OutMessageQueue, (With<MyLobby>, Without<MyNetworkClient>)>,
    mut player: Query<(&mut CollisionLayer, Option<&FlagCarrier>)>,
    flags: Query<&FlagState>,
    mut commands: Commands,
) {
    let player_entity = trigger.entity();
    let (lobby_entity, _, _) = lobby
        .get_lobby_of_player(player_entity)
        .expect("Lobby not found");
    let mut lobby_message_queue = lobby_message_queue
        .get_mut(lobby_entity)
        .expect("Message queue not found");

    lobby_message_queue.push_back(MessageContainer::new(
        MessageTarget::ToEveryone,
        NetworkMessageType::PlayerDied(EntityDataWrapper::new(player_entity)),
    ));

    let (mut player_collision_layer, flag_carrier) =
        player.get_mut(player_entity).expect("Player not found");
    *player_collision_layer = CollisionLayer::none(); // Player can't collide with anything

    // Drop flag if player was carrying one
    if let Some(flag_carrier) = flag_carrier {
        let flag_state = flags.get(flag_carrier.flag).expect("Flag not found");
        if let FlagState::Carried(carrier_entity) = *flag_state {
            if carrier_entity == player_entity {
                commands.trigger_targets(
                    FlagGotDroppedTrigger {
                        flag: flag_carrier.flag,
                        carrier: carrier_entity,
                    },
                    lobby_entity,
                );
            }
        }
    }
}
