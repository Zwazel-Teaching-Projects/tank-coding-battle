use bevy::prelude::*;
use shared::game::{collision_handling::components::CollisionLayer, flag::FlagState};

use super::triggers::FlagGotPickedUpTrigger;

// TODO: Send out network message for flag picked up(?)
pub fn flag_picked_up(
    trigger: Trigger<FlagGotPickedUpTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer)>,
) {
    let _lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;
    let picker_entity = trigger.picker;

    let (mut flag_state, mut collision_layer) = flags.get_mut(flag_entity).expect("Flag not found");

    *flag_state = FlagState::Carried(picker_entity);
    *collision_layer = CollisionLayer::none(); // Can not collide with anything while carried
}
