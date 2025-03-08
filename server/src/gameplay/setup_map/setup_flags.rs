use bevy::{ecs::entity::EntityHashSet, prelude::*};
use shared::{
    asset_handling::maps::MarkerType,
    game::{
        collision_handling::components::{Collider, CollisionLayer, WantedTransform},
        flag::{FlagBaseMarker, FlagMarker, FlagState},
    },
    networking::lobby_management::{InLobby, InTeam, MyLobby},
};

use crate::gameplay::capture_the_flag::triggers::InitAllFlagsTrigger;

const FLAG_BASE_HALF_SIZE: Vec3 = Vec3::new(0.25, 0.5, 0.25);
const FLAG_HALF_SIZE: Vec3 = Vec3::new(0.25, 0.5, 0.25);

pub fn setup_flags(
    trigger: Trigger<InitAllFlagsTrigger>,
    mut commands: Commands,
    mut my_lobby: Query<&mut MyLobby>,
) {
    let lobby_id = trigger.entity();
    let mut lobby = my_lobby.get_mut(lobby_id).expect("Lobby not found");

    let mut new_bases = Vec::new();
    let mut new_flags = Vec::new();
    if let Some(map_config) = &lobby.map_config {
        let team_names = map_config.get_team_names();
        let map = &map_config.map;
        map.markers.iter().for_each(|marker| {
            if let MarkerType::FlagBase { flag_number } = marker.kind {
                let marker_position = map
                    .get_real_world_position_of_tile(marker.tile.clone())
                    .expect("Failed to get real world position of tile");

                let my_team = &marker.group;
                let team_members = &map_config
                    .get_team(my_team)
                    .expect("Failed to get team")
                    .players;
                let enemy_team_members = team_names
                    .iter()
                    .filter(|team_name| *team_name != my_team)
                    .flat_map(|team_name| {
                        map_config
                            .get_team(team_name)
                            .expect("Failed to get team")
                            .players
                            .clone()
                    })
                    .collect::<Vec<_>>();

                // Create flag base entity
                let new_base = commands
                    .spawn((
                        Name::new(format!("FlagBase_{}_{}", my_team, flag_number)),
                        WantedTransform(Transform::from_translation(marker_position)),
                        Collider {
                            half_size: FLAG_BASE_HALF_SIZE,
                            max_slope: 0.0,
                        },
                        CollisionLayer::flag_base()
                            // Never interact with other flag bases, only with flags or my own flag base (to deliver enemy flag)
                            .with_ignore(EntityHashSet::from_iter(enemy_team_members.clone())),
                        InTeam(my_team.clone()),
                        InLobby(lobby_id),
                    ))
                    .id();
                new_bases.push(new_base);

                // Create flag entity
                let new_flag = commands
                    .spawn((
                        Name::new(format!("Flag_{}_{}", my_team, flag_number)),
                        InTeam(my_team.clone()),
                        InLobby(lobby_id),
                        FlagMarker { base: new_base },
                        FlagState::InBase,
                        WantedTransform(Transform::from_translation(marker_position)),
                        Collider {
                            half_size: FLAG_HALF_SIZE,
                            max_slope: 0.0,
                        },
                        // At start, it's considered to be in base, so teammembers should not collide with it
                        CollisionLayer::flag()
                            .with_ignore(EntityHashSet::from_iter(team_members.clone())),
                    ))
                    .id();
                new_flags.push(new_flag);

                commands.entity(new_base).insert(FlagBaseMarker {
                    flag_in_base: true,
                    my_flag: new_flag,
                });
            }
        });
    }
    for flag in new_flags {
        lobby.flags.push(flag);
    }
    for base in new_bases {
        lobby.flag_bases.push(base);
    }
}
