use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::components::CollisionLayer,
        flag::{FlagCarrier, FlagMarker, FlagState},
    },
    networking::messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_data::flag_event_data::FlagEventDataWrapper,
        message_queue::OutMessageQueue,
    },
};

use super::{triggers::FlagGotDroppedTrigger, MyLobby};

pub fn flag_dropped(
    trigger: Trigger<FlagGotDroppedTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer), With<FlagMarker>>,
    mut lobby_queue: Query<&mut OutMessageQueue, With<MyLobby>>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;
    let carrier_entity = trigger.carrier;

    let (mut flag_state, mut flag_collision_layer) =
        flags.get_mut(flag_entity).expect("Flag not found");
    *flag_collision_layer = CollisionLayer::flag();
    flag_collision_layer.ignore.clear(); // Can be picked up by everyone again
    *flag_state = FlagState::Dropped;

    let mut lobby_queue = lobby_queue
        .get_mut(lobby_entity)
        .expect("Message queue not found");
    lobby_queue.push_back(MessageContainer::new(
        MessageTarget::ToEveryone,
        NetworkMessageType::FlagGotDropped(FlagEventDataWrapper {
            flag_id: flag_entity,
            carrier_id: carrier_entity,
        }),
    ));

    commands.entity(carrier_entity).remove::<FlagCarrier>();
}
