use bevy::prelude::*;
use shared::game::{
    collision_handling::components::WantedTransform,
    flag::{FlagBaseMarker, FlagMarker, FlagState},
};

use super::triggers::ResetFlagTrigger;

pub fn reset_flag(
    trigger: Trigger<ResetFlagTrigger>,
    mut flags: Query<
        (
            &mut WantedTransform,
            &mut Transform,
            &FlagMarker,
            &mut FlagState,
        ),
        Without<FlagBaseMarker>,
    >,
    bases: Query<&Transform, With<FlagBaseMarker>>,
) {
    let flag_entity = trigger.entity();

    let (mut wanted_transform, mut transform, flag_marker, mut flag_state) =
        flags.get_mut(flag_entity).expect("Flag not found");

    let flag_base_transform = bases.get(flag_marker.base).expect("Flag base not found");

    *wanted_transform = WantedTransform(*flag_base_transform);
    *transform = *flag_base_transform;
    *flag_state = FlagState::InBase;
}
