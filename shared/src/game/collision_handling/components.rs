use bevy::{ecs::entity::EntityHashSet, prelude::*};

#[derive(Debug, Component, Reflect, Clone)]
#[reflect(Component)]
#[require(CollisionLayer)]
pub struct Collider {
    pub half_size: Vec3,
    pub max_slope: f32,
}

impl Collider {
    pub fn new(half_size: Vec3, max_slope: f32) -> Self {
        Self {
            half_size,
            max_slope,
        }
    }
}

#[derive(Debug, Component, Reflect, Clone, PartialEq, Eq, Default)]
#[reflect(Component)]
pub struct CollisionLayer {
    pub mask: u32,
    /// Collection of entities that this entity should ignore collisions with.
    pub ignore: EntityHashSet,
}

impl CollisionLayer {
    pub const ALL: u32 = u32::MAX;

    pub const NO_COLLISION: u32 = 0;
    pub const PLAYER: u32 = 1;
    pub const FLAG: u32 = 2;
    pub const FLAG_BASE: u32 = 3;

    /// Create a collision layer from a list of layer indices.
    /// Each index in the list will be set as a bit in the mask.
    pub fn new(layers: &[u32]) -> Self {
        let mask = layers.iter().fold(0, |acc, &layer| acc | (1 << layer));
        Self {
            mask,
            ignore: EntityHashSet::default(),
        }
    }

    /// Create a collision layer for flag
    pub fn flag() -> Self {
        Self::new(&[Self::FLAG])
    }

    /// Create a collision layer for flag base
    pub fn flag_base() -> Self {
        Self::new(&[Self::FLAG_BASE])
    }

    /// Create a collision layer for player
    pub fn player() -> Self {
        Self::new(&[Self::PLAYER])
    }

    pub fn none() -> Self {
        Self::new(&[Self::NO_COLLISION])
    }

    pub fn with_ignore(mut self, ignore: EntityHashSet) -> Self {
        self.ignore = ignore;
        self
    }

    pub fn with_additional_layers(mut self, layers: &[u32]) -> Self {
        let mask = layers
            .iter()
            .fold(self.mask, |acc, &layer| acc | (1 << layer));
        self.mask = mask;
        self
    }

    /// Check if the collision layer contains the given layer index.
    pub fn contains(&self, layer: u32) -> bool {
        (self.mask & (1 << layer)) != 0
    }

    /// Add a layer by setting its respective bit.
    pub fn add_layer(&mut self, layer: u32) {
        self.mask |= 1 << layer;
    }

    /// Remove a layer by clearing its respective bit.
    pub fn remove_layer(&mut self, layer: u32) {
        self.mask &= !(1 << layer);
    }

    /// Check if there is any overlapping layer between two CollisionLayers.
    pub fn intersects(&self, other: &Self) -> bool {
        (self.mask & other.mask) != 0
    }
}

#[derive(Debug, Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct WantedTransform(pub Transform);

pub fn insert_transform_for_wanted_transform(
    trigger: Trigger<OnAdd, WantedTransform>,
    wanted_transform: Query<&WantedTransform>,
    mut commands: Commands,
) {
    let entity = trigger.entity();
    let wanted_transform = wanted_transform
        .get(entity)
        .expect("WantedTransform should exist");

    commands.entity(entity).insert(**wanted_transform);
}
