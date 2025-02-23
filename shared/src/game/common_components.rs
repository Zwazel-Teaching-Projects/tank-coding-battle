use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct DespawnTimer(pub Timer);

#[derive(Debug, Component, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct TickBasedDespawnTimer {
    #[deref]
    pub ticks_left: u32,
}
