use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::triggers::CollidedWithTrigger,
        flag::{FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::lobby_management::InTeam,
};

pub fn handle_collision_with_flag(
    trigger: Trigger<CollidedWithTrigger>,
    mut flags: Query<(&Transform, &mut FlagState, &InTeam), With<FlagMarker>>,
    tanks: Query<&InTeam, With<TankBodyMarker>>,
) {
    let flag_entity = trigger.entity();
    let carrier_entity = trigger.entity;
    let (flag_transform, mut flag_state, in_team) =
        flags.get_mut(flag_entity).expect("Flag not found");

    let carrier_in_team = tanks.get(carrier_entity).expect("Tank not found");

    match *flag_state {
        FlagState::InBase => {
            if carrier_in_team.0 != in_team.0 {
                *flag_state = FlagState::Carried(carrier_entity);
            } else {
                warn!("Flag {:?} collided with a tank from the same team while in base, this should be ignored and never happen", flag_entity);
            }
        }
        FlagState::Carried(entity) => {
            unimplemented!("We should never be able to collide with a carried flag")
        }
        FlagState::Dropped => todo!(),
    }

    println!("Flag {:?} collided with {:?}", flag_entity, trigger.entity);
}
