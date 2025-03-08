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
    mut player_collision_layer: Query<&mut CollisionLayer, Without<FlagMarker>>,
) {
    let _lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;
    let carrier_entity = trigger.carrier;

    let (mut flag_state, mut collision_layer) = flags.get_mut(flag_entity).expect("Flag not found");
    *collision_layer = CollisionLayer::flag();
    collision_layer.ignore.clear(); // Can be picked up by everyone again
    *flag_state = FlagState::Dropped;

    // Player no longer carries the flag, update the collision layer so we can't collide with the flag base
    let mut player_collision_layer = player_collision_layer
        .get_mut(carrier_entity)
        .expect("Player not found");
    player_collision_layer.remove_layer(CollisionLayer::FLAG_BASE);
}
