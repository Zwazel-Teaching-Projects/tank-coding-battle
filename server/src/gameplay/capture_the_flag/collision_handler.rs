use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::triggers::CollidedWithTrigger,
        flag::{FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::lobby_management::{InLobby, InTeam},
};

use super::triggers::{FlagGotPickedUpTrigger, ResetFlagTrigger};

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
                        picker: carrier_entity,
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
                        picker: carrier_entity,
                        flag: flag_entity,
                    },
                    **in_lobby,
                );
            }
        }
    }
}
