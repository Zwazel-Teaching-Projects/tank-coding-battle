use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::triggers::CollidedWithTrigger,
        flag::{FlagMarker, FlagState},
    },
    networking::lobby_management::InTeam,
};

pub fn handle_collision_with_flag(
    trigger: Trigger<CollidedWithTrigger>,
    mut flags: Query<(&Transform, &mut FlagState, &InTeam), With<FlagMarker>>,
) {
    let flag_entity = trigger.entity();
    let (flag_transform, mut flag_state, in_team) =
        flags.get_mut(flag_entity).expect("Flag not found");

    println!("Flag {:?} collided with {:?}", flag_entity, trigger.entity);
}
