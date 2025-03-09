use bevy::prelude::*;
use shared::game::{
    collision_handling::components::CollisionLayer,
    flag::{FlagCarrier, FlagMarker, FlagState},
};

use super::triggers::FlagGotDroppedTrigger;

// TODO: Send out network message for flag dropped(?)
pub fn flag_dropped(
    trigger: Trigger<FlagGotDroppedTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer), With<FlagMarker>>,
    mut commands: Commands,
) {
    let _lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;
    let carrier_entity = trigger.carrier;

    let (mut flag_state, mut flag_collision_layer) =
        flags.get_mut(flag_entity).expect("Flag not found");
    *flag_collision_layer = CollisionLayer::flag();
    flag_collision_layer.ignore.clear(); // Can be picked up by everyone again
    *flag_state = FlagState::Dropped;

    commands.entity(carrier_entity).remove::<FlagCarrier>();
}
