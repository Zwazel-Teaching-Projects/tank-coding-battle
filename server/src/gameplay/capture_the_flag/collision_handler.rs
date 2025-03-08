use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::{components::CollisionLayer, triggers::CollidedWithTrigger},
        flag::{FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::lobby_management::InTeam,
};

pub fn handle_collision_with_flag(
    trigger: Trigger<CollidedWithTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer, &InTeam), With<FlagMarker>>,
    tanks: Query<&InTeam, With<TankBodyMarker>>,
) {
    let flag_entity = trigger.entity();
    let carrier_entity = trigger.entity;
    let (mut flag_state, mut collision_layer, in_team) =
        flags.get_mut(flag_entity).expect("Flag not found");

    let carrier_in_team = tanks.get(carrier_entity).expect("Tank not found");

    match *flag_state {
        FlagState::InBase => {
            if carrier_in_team.0 != in_team.0 {
                *flag_state = FlagState::Carried(carrier_entity);
                *collision_layer = CollisionLayer::none();
            } else {
                warn!("Flag {:?} collided with a tank from the same team while in base, this should never happen", flag_entity);
            }
        }
        FlagState::Carried(_entity) => {
            unimplemented!("We should never be able to collide with a carried flag")
        }
        FlagState::Dropped => todo!(),
    }
}
