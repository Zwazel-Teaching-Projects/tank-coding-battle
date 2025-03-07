use bevy::prelude::*;
use shared::{
    asset_handling::maps::MarkerType,
    game::flag::FlagMarker,
    networking::lobby_management::{InLobby, InTeam, MyLobby},
};

use super::triggers::ResetAllFlagsTrigger;

pub fn reset_flags(
    trigger: Trigger<ResetAllFlagsTrigger>,
    mut commands: Commands,
    lobby: Query<&MyLobby>,
    query: Query<(&InTeam, &InLobby, &FlagMarker)>,
) {
    let lobby_id = trigger.entity();
    let my_lobby = lobby.get(lobby_id).expect("Lobby not found");

    if let Some(map_config) = &my_lobby.map_config {
        map_config.map.markers.iter().for_each(|marker| {
            if let MarkerType::FlagBase { flag_number } = marker.kind {
                query.iter().for_each(|(team, in_lobby, flag_marker)| {
                    if in_lobby.0 == lobby_id && team.0 == marker.group {
                        // Reset
                        println!("Resetting flag {}", flag_number);
                        // found flag entity for the flag base, no need to further iterate
                        return;
                    }
                });
            }
        });
    }
}
