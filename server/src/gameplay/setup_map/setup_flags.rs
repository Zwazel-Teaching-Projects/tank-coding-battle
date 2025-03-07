use bevy::{ecs::entity::EntityHashSet, prelude::*};
use shared::{
    asset_handling::maps::MarkerType,
    game::{
        collision_handling::components::{Collider, CollisionLayer, WantedTransform},
        flag::{FlagMarker, FlagState},
    },
    networking::lobby_management::{InLobby, InTeam, MyLobby},
};

use crate::gameplay::capture_the_flag::triggers::InitAllFlagsTrigger;

pub fn setup_flags(
    trigger: Trigger<InitAllFlagsTrigger>,
    mut commands: Commands,
    mut my_lobby: Query<&mut MyLobby>,
) {
    let lobby_id = trigger.entity();
    let mut lobby = my_lobby.get_mut(lobby_id).expect("Lobby not found");

    let mut new_flags = Vec::new();
    if let Some(map_config) = &lobby.map_config {
        let map = &map_config.map;
        map.markers.iter().for_each(|marker| {
            if let MarkerType::FlagBase { flag_number } = marker.kind {
                let marker_position = map
                    .get_real_world_position_of_tile(marker.tile.clone())
                    .expect("Failed to get real world position of tile");

                let team = &marker.group;
                let team_members = &map_config
                    .get_team(team)
                    .expect("Failed to get team")
                    .players;

                // Create flag entity
                let new_flag = commands
                    .spawn((
                        Name::new(format!("Flag_{}_{}", team, flag_number)),
                        InTeam(team.clone()),
                        InLobby(lobby_id),
                        FlagMarker(flag_number),
                        FlagState::InBase,
                        WantedTransform(Transform::from_translation(marker_position)),
                        Collider {
                            half_size: Vec3::new(0.25, 0.5, 0.25),
                            max_slope: 0.0,
                        },
                        // At start, it's considered to be in base, so teammembers should not collide with it
                        CollisionLayer::flag()
                            .with_ignore(EntityHashSet::from_iter(team_members.clone())),
                    ))
                    .id();
                new_flags.push(new_flag);
            }
        });
    }
    for flag in new_flags {
        lobby.flags.push(flag);
    }
}
