use bevy::prelude::*;
use shared::game::{
    collision_handling::components::CollisionLayer,
    flag::{FlagMarker, FlagState},
};

use super::triggers::FlagGotDroppedTrigger;

// TODO: Send out network message for flag dropped(?)
pub fn flag_dropped(
    trigger: Trigger<FlagGotDroppedTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer), With<FlagMarker>>,
) {
    let _lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;

    let (mut flag_state, mut collision_layer) = flags.get_mut(flag_entity).expect("Flag not found");
    *collision_layer = CollisionLayer::flag();
    collision_layer.ignore.clear(); // Can be picked up by everyone
    *flag_state = FlagState::Dropped;
}
