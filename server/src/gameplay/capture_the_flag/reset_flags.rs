use bevy::{ecs::entity::EntityHashSet, prelude::*};
use shared::{
    game::{
        collision_handling::components::{CollisionLayer, WantedTransform},
        flag::{FlagBaseMarker, FlagMarker, FlagState},
    },
    networking::{
        lobby_management::{InLobby, InTeam, MyLobby},
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::flag_event_data::FlagSimpleEventDataWrapper,
            message_queue::OutMessageQueue,
        },
    },
};

use super::triggers::ResetFlagTrigger;

pub fn reset_flag(
    trigger: Trigger<ResetFlagTrigger>,
    mut my_lobby: Query<(&MyLobby, &mut OutMessageQueue)>,
    mut flags: Query<
        (
            &mut WantedTransform,
            &mut Transform,
            &FlagMarker,
            &mut FlagState,
            &mut CollisionLayer,
            &InLobby,
            &InTeam,
        ),
        Without<FlagBaseMarker>,
    >,
    mut bases: Query<(&mut FlagBaseMarker, &Transform)>,
) {
    let flag_entity = trigger.entity();

    let (
        mut wanted_transform,
        mut transform,
        flag_marker,
        mut flag_state,
        mut collision_layer,
        in_lobby,
        in_team,
    ) = flags.get_mut(flag_entity).expect("Flag not found");

    let (lobby, mut lobby_message_queue) = my_lobby.get_mut(**in_lobby).expect("Lobby not found");

    lobby_message_queue.push_back(MessageContainer::new(
        MessageTarget::ToEveryone,
        NetworkMessageType::FlagReturnedInBase(FlagSimpleEventDataWrapper {
            flag_id: flag_entity,
        }),
    ));

    if let Some(map_config) = &lobby.map_config {
        let team_members = &map_config
            .get_team(&in_team)
            .expect("Failed to get team")
            .players;

        let (mut flag_base_marker, flag_base_transform) = bases
            .get_mut(flag_marker.base)
            .expect("Flag base not found");

        flag_base_marker.flag_in_base = true;
        *wanted_transform = WantedTransform(*flag_base_transform);
        *transform = *flag_base_transform;
        *flag_state = FlagState::InBase;

        *collision_layer =
            CollisionLayer::flag().with_ignore(EntityHashSet::from_iter(team_members.clone()));
    } else {
        error!("Failed to get map config");
    }
}
