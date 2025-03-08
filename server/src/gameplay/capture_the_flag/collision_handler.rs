use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::triggers::CollidedWithTrigger,
        flag::{FlagBaseMarker, FlagCarrier, FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::lobby_management::{InLobby, InTeam},
};

use super::triggers::{FlagGotDroppedTrigger, FlagGotPickedUpTrigger, ResetFlagTrigger};

pub fn handle_collision_with_flag(
    trigger: Trigger<CollidedWithTrigger>,
    flags: Query<(&FlagState, &InTeam, &InLobby), With<FlagMarker>>,
    tanks: Query<&InTeam, With<TankBodyMarker>>,
    mut commands: Commands,
) {
    let flag_entity = trigger.entity();
    let carrier_entity = trigger.entity;
    let (flag_state, in_team, in_lobby) = flags.get(flag_entity).expect("Flag not found");

    let carrier_in_team = tanks.get(carrier_entity).expect("Tank not found");
    let carrier_is_in_flag_team = carrier_in_team.0 == in_team.0;

    match *flag_state {
        FlagState::InBase => {
            if !carrier_is_in_flag_team {
                commands.trigger_targets(
                    FlagGotPickedUpTrigger {
                        carrier: carrier_entity,
                        flag: flag_entity,
                    },
                    **in_lobby,
                );
            } else {
                warn!("Flag {:?} collided with a tank from the same team while in base, this should never happen", flag_entity);
            }
        }
        FlagState::Carried(_entity) => {
            unimplemented!("We should never be able to collide with a carried flag")
        }
        FlagState::Dropped => {
            if carrier_is_in_flag_team {
                commands.trigger_targets(ResetFlagTrigger, flag_entity);
            } else {
                commands.trigger_targets(
                    FlagGotPickedUpTrigger {
                        carrier: carrier_entity,
                        flag: flag_entity,
                    },
                    **in_lobby,
                );
            }
        }
    }
}

pub fn handle_collision_with_flag_base(
    trigger: Trigger<CollidedWithTrigger>,
    flag_base: Query<(&FlagBaseMarker, &InTeam)>,
    flags: Query<(&FlagState, &InTeam), With<FlagMarker>>,
    players: Query<(&InTeam, &FlagCarrier, &InLobby), With<TankBodyMarker>>,
    mut commands: Commands,
) {
    let my_flag_base_entity = trigger.entity();
    let collider_entity = trigger.entity;

    // Optimally, the flag base would be my own flag base, and we are carrying the enemy flag.
    let (my_flag_base_marker, my_flag_base_in_team) = flag_base
        .get(my_flag_base_entity)
        .expect("Flag base not found");
    if let Ok((player_in_team, player_carrying_flag, player_in_lobby)) =
        players.get(collider_entity)
    {
        if player_in_team.0 == my_flag_base_in_team.0 {
            // We collided with our own flag base
            if my_flag_base_marker.flag_in_base {
                // The flag is in the base, we can score a point

                let (enemy_flag_state, enemy_flag_in_team) = flags
                    .get(player_carrying_flag.flag)
                    .expect("Flag not found");

                if enemy_flag_in_team.0 == my_flag_base_in_team.0 {
                    // The flag i'm carrying is the same team as my flag base, we can't score a point
                    warn!("Player {:?} collided with its own flag base while carrying the same team's flag, this should never happen", collider_entity);
                } else {
                    match *enemy_flag_state {
                        FlagState::Carried(carrier_entity) => {
                            // The enemy flag is carried by a player from the other team, we can score a point
                            if carrier_entity == collider_entity {
                                // The player that collided with the flag base is carrying the flag, we can score a point
                                // TODO: Send out network message for point scored(?) + actually score the point. or end game. or anything.
                                commands.trigger_targets(
                                    FlagGotDroppedTrigger {
                                        carrier: carrier_entity,
                                        flag: player_carrying_flag.flag,
                                    },
                                    **player_in_lobby,
                                );
                                commands
                                    .trigger_targets(ResetFlagTrigger, player_carrying_flag.flag);
                            } else {
                                // The player that collided with the flag base is not carrying the flag, we can't score a point
                                warn!("Player {:?} collided with its own flag base while the enemy flag was carried by another player, this should never happen", collider_entity);
                            }
                        }
                        _ => {
                            warn!("Player {:?} collided with its own flag base while the carried flag was not carried, this should never happen", collider_entity);
                        }
                    }
                }
            } else {
                // Our flag is not in our base, we can't score a point. this is expected behavior. Do nothing.
            }
        } else {
            warn!(
                "Player {:?} collided with enemy flag base, this should never happen",
                collider_entity
            );
        }
    } else {
        warn!(
            "Collider {:?} is not a player, this should never happen",
            collider_entity
        );
    }
}
