use bevy::prelude::*;
use shared::networking::messages::message_container::GameStateTrigger;

use super::DelayedDespawn;

pub fn despawn_delayed_entites(
    trigger: Trigger<GameStateTrigger>,
    mut commands: Commands,
    query: Query<(Entity, &DelayedDespawn)>,
) {
    let current_tick = trigger.event().tick;
    for (entity, delayed_despawn) in query.iter() {
        if **delayed_despawn <= current_tick {
            commands.entity(entity).despawn_recursive();
        }
    }
}
