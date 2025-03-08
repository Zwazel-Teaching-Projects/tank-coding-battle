use bevy::prelude::*;
use shared::{
    asset_handling::maps::MarkerType,
    game::{
        collision_handling::components::WantedTransform,
        flag::{FlagMarker, FlagState},
    },
    networking::lobby_management::{InLobby, InTeam, MyLobby},
};

use super::triggers::ResetFlagTrigger;

pub fn reset_flag(
    trigger: Trigger<ResetFlagTrigger>,
    mut flags: Query<(
        &mut WantedTransform,
        &mut Transform,
        &FlagMarker,
        &InTeam,
        &InLobby,
        &mut FlagState,
    )>,
    lobby: Query<&MyLobby>,
) {
    let flag_entity = trigger.entity();

    let (mut wanted_transform, mut transform, flag_marker, in_team, in_lobby, mut flag_state) =
        flags.get_mut(flag_entity).expect("Flag not found");
    let my_lobby = lobby.get(in_lobby.0).expect("Lobby not found");

    let map_config = my_lobby.map_config.as_ref().expect("Map config not found");
    let map = &map_config.map;

    let flag_base = map
        .markers
        .iter()
        .find(|marker| {
            if let MarkerType::FlagBase { flag_number } = marker.kind {
                return flag_marker.0 == flag_number && marker.group == in_team.0;
            }
            false
        })
        .expect("Flag base not found");

    let marker_position = map
        .get_real_world_position_of_tile(flag_base.tile.clone())
        .expect("Failed to get real world position of tile");

    *wanted_transform = WantedTransform(Transform::from_translation(marker_position));
    *transform = Transform::from_translation(marker_position);
    *flag_state = FlagState::InBase;
}
