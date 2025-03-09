use bevy::prelude::*;
use shared::game::{
    collision_handling::components::CollisionLayer,
    flag::{FlagBaseMarker, FlagCarrier, FlagMarker, FlagState},
    player_handling::TankBodyMarker,
};

use super::triggers::FlagGotPickedUpTrigger;

// TODO: Send out network message for flag picked up(?)
pub fn flag_picked_up(
    trigger: Trigger<FlagGotPickedUpTrigger>,
    mut flags: Query<(&mut FlagState, &mut CollisionLayer, &FlagMarker)>,
    mut flag_bases: Query<
        (&mut CollisionLayer, &mut FlagBaseMarker),
        (Without<FlagMarker>, Without<TankBodyMarker>),
    >,
    mut commands: Commands,
) {
    let _lobby_entity = trigger.entity();
    let flag_entity = trigger.flag;
    let picker_entity = trigger.carrier;

    let (mut flag_state, mut collision_layer, flag_marker) =
        flags.get_mut(flag_entity).expect("Flag not found");
    let (mut _flag_base_collision_layer, mut flag_base_marker) = flag_bases
        .get_mut(flag_marker.base)
        .expect("Flag base not found");

    flag_base_marker.flag_in_base = false;

    *flag_state = FlagState::Carried(picker_entity);
    *collision_layer = CollisionLayer::flag_base(); // Can only collide with flag bases

    commands
        .entity(picker_entity)
        .insert(FlagCarrier { flag: flag_entity });
}
