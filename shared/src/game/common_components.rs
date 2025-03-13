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

#[derive(Debug, Component, Reflect, Clone, PartialEq, Deref, DerefMut)]
#[reflect(Component)]
pub struct Gravity {
    #[deref]
    pub gravity: f32,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq, Deref, DerefMut, Default)]
#[reflect(Component)]
pub struct Velocity {
    #[deref]
    pub velocity: Vec3,
}
