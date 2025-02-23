use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
#[require(CollisionLayer)]
pub struct Collider {
    pub half_size: Vec3,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq, Eq, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct CollisionLayer {
    pub mask: u32,
}

impl CollisionLayer {
    /// Create a collision layer from a list of layer indices.
    /// Each index in the list will be set as a bit in the mask.
    pub fn new(layers: &[u32]) -> Self {
        let mask = layers.iter().fold(0, |acc, &layer| acc | (1 << layer));
        Self { mask }
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
#[require(Transform)]
pub struct WantedTransform(pub Transform);
