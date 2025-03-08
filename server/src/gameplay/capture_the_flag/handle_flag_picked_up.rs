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
    mut player_collision_layer: Query<
        &mut CollisionLayer,
        (With<TankBodyMarker>, Without<FlagMarker>),
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
    let mut player_collision_layer = player_collision_layer
        .get_mut(picker_entity)
        .expect("Player not found");

    // Player picked up the flag, update the collision layer so we can collide with the flag base
    player_collision_layer.add_layer(CollisionLayer::FLAG_BASE);

    flag_base_marker.flag_in_base = false;

    *flag_state = FlagState::Carried(picker_entity);
    *collision_layer = CollisionLayer::none(); // Can not collide with anything while carried

    commands
        .entity(picker_entity)
        .insert(FlagCarrier { flag: flag_entity });
}
