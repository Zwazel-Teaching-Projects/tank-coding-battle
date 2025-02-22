use bevy::prelude::*;

use super::common_components::DespawnTimer;

pub fn handle_despawn_timer(
    mut timer: Query<(Entity, &mut DespawnTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut timer) in timer.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.tick(time.delta()).finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}
