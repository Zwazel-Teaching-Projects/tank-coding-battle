use bevy::prelude::*;
use shared::{
    game::collision_handling::components::WantedTransform,
    networking::messages::message_data::game_starts::GameStarts,
};

pub fn interpolate_transforms(
    mut query: Query<(&mut Transform, &WantedTransform)>,
    game_config: Res<GameStarts>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_secs();
    let tick_rate = game_config.tick_rate as f32; // how many ticks per second
    let alpha = delta_seconds * tick_rate;

    for (mut transform, wanted_transform) in query.iter_mut() {
        transform.translation = transform
            .translation
            .lerp(wanted_transform.translation, alpha);
        transform.rotation = transform.rotation.slerp(wanted_transform.rotation, alpha);
    }
}
