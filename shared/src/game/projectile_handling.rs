use bevy::prelude::*;

use super::common_components::TickBasedDespawnTimer;

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
#[require(TickBasedDespawnTimer(default_despawn_timer))]
pub struct ProjectileMarker {
    pub speed: f32,
    pub damage: f32,
    pub owner: Entity,
}

fn default_despawn_timer() -> TickBasedDespawnTimer {
    TickBasedDespawnTimer {
        ticks_left: 100,
        ticks_total: 100,
    }
}
