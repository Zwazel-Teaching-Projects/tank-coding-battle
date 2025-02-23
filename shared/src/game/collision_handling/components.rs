use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct Collider {
    pub half_size: Vec3,
}

#[derive(Debug, Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct WantedTransform(pub Transform);
