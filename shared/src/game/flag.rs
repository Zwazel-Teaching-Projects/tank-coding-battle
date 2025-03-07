use bevy::prelude::*;

#[derive(Debug, Clone, Default, Reflect, Component, Deref, DerefMut)]
#[reflect(Component)]
#[require(FlagState)]
pub struct FlagMarker(pub usize);

#[derive(Debug, Clone, Default, Reflect, Component)]
#[reflect(Component)]
pub enum FlagState {
    #[default]
    InBase,
    Carried(Entity),
    Dropped,
}
