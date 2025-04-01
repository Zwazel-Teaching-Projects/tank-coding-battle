use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::components::CollisionLayer,
        flag::{FlagBaseMarker, FlagCarrier, FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::{
        lobby_management::MyLobby,
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::flag_event_data::FlagEventDataWrapper,
            message_queue::OutMessageQueue,
        },
    },
};

use crate::networking::handle_clients::lib::MyNetworkClient;

use super::triggers::FlagGotPickedUpTrigger;

pub fn flag_picked_up(
    trigger: Trigger<FlagGotPickedUpTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer, &FlagMarker)>,
    mut flag_bases: Query<
        (&mut CollisionLayer, &mut FlagBaseMarker),
        (Without<FlagMarker>, Without<TankBodyMarker>),
    >,
    mut commands: Commands,
    mut lobby: Query<&mut OutMessageQueue, (With<MyLobby>, Without<MyNetworkClient>)>,
) {
    let lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;
    let picker_entity = trigger.carrier;

    let (mut flag_state, mut collision_layer, flag_marker) =
        flags.get_mut(flag_entity).expect("Flag not found");
    let (mut _flag_base_collision_layer, mut flag_base_marker) = flag_bases
        .get_mut(flag_marker.base)
        .expect("Flag base not found");
    let mut lobby_message_queue = lobby
        .get_mut(lobby_entity)
        .expect("Message queue not found");

    lobby_message_queue.push_back(MessageContainer::new(
        MessageTarget::ToEveryone,
        NetworkMessageType::FlagGotPickedUp(FlagEventDataWrapper {
            flag_id: flag_entity,
            carrier_id: picker_entity,
        }),
    ));

    flag_base_marker.flag_in_base = false;

    *flag_state = FlagState::Carried(picker_entity);
    *collision_layer = CollisionLayer::flag_base(); // Can only collide with flag bases

    commands
        .entity(picker_entity)
        .insert(FlagCarrier { flag: flag_entity });
}
