use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::triggers::CollidedWithTrigger,
        flag::{FlagBaseMarker, FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::lobby_management::{InLobby, InTeam},
};

use super::triggers::{FlagGotDroppedTrigger, FlagGotPickedUpTrigger, ResetFlagTrigger};

/// Handles the collision between a flag and a tank.
pub fn handle_collision_with_flag(
    trigger: Trigger<CollidedWithTrigger>,
    flags: Query<(&FlagState, &InTeam, &InLobby), With<FlagMarker>>,
    tanks: Query<&InTeam, With<TankBodyMarker>>,
    mut commands: Commands,
) {
    let flag_entity = trigger.entity();
    let collider_entity = trigger.entity;
    let (flag_state, in_team, in_lobby) = flags.get(flag_entity).expect("Flag not found");

    if let Ok(carrier_in_team) = tanks.get(collider_entity) {
        let carrier_is_in_flag_team = carrier_in_team.0 == in_team.0;

        match *flag_state {
            FlagState::InBase => {
                if !carrier_is_in_flag_team {
                    commands.trigger_targets(
                        FlagGotPickedUpTrigger {
                            carrier: collider_entity,
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
                            carrier: collider_entity,
                            flag: flag_entity,
                        },
                        **in_lobby,
                    );
                }
            }
        }
    } else {
        // The collider is not a tank. We don't care about this collision.
    }
}

pub fn handle_collision_with_flag_base(
    trigger: Trigger<CollidedWithTrigger>,
    flag_base: Query<(&FlagBaseMarker, &InTeam)>,
    flags: Query<(&FlagState, &InTeam, &InLobby), With<FlagMarker>>,
    mut commands: Commands,
) {
    let my_flag_base_entity = trigger.entity(); // Should always be the flag base
    let collider_entity = trigger.entity; // Should always be the carried flag

    // Optimally, the flag base would be my own flag base, and we are carrying the enemy flag.
    let (my_flag_base_marker, my_flag_base_in_team) = flag_base
        .get(my_flag_base_entity)
        .expect("Flag base not found");

    if !my_flag_base_marker.flag_in_base {
        // the flag is not in the base, so we don't care about this collision. As the team can only score when the flag is in the base.
        return;
    }

    if let Ok((collided_flag_state, collided_flag_in_team, flag_in_lobby)) =
        flags.get(collider_entity)
    {
        if my_flag_base_marker.my_flag == collider_entity {
            warn!(
                "Flag {:?} collided with its own flag base, this should never happen",
                collider_entity
            );
            return;
        }

        if collided_flag_in_team.0 == my_flag_base_in_team.0 {
            warn!(
                "Flag {:?} collided with a flag base from the same team, this should never happen",
                collider_entity
            );
            return;
        }

        match *collided_flag_state {
            FlagState::Carried(carrier_entity) => {
                commands.trigger_targets(
                    FlagGotDroppedTrigger {
                        carrier: carrier_entity,
                        flag: collider_entity,
                    },
                    **flag_in_lobby,
                );
                commands.trigger_targets(ResetFlagTrigger, collider_entity);
            }
            _ => {
                warn!("Flag {:?} collided with a flag base while the flag was not carried, this should never happen", collider_entity);
            }
        }
    } else {
        warn!(
            "Collider {:?} is not a Flag, this should never happen",
            collider_entity
        );
    }
}
