use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::collision_handling::structs::Side;

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct HitMessageData {
    /// The entity that was hit
    pub hit_entity: Entity,
    /// The projectile entity that hit the entity
    pub projectile_entity: Entity,
    /// The side of the entity that was hit
    pub hit_side: Side,
    /// The damage that was dealt to the entity
    pub damage_dealt: f32,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GotHitMessageData {
    /// The entity that shot the entity
    pub shooter_entity: Entity,
    /// The projectile entity that hit the entity
    pub projectile_entity: Entity,
    /// The side of the entity that was hit
    pub hit_side: Side,
    /// The damage that was dealt to the entity
    pub damage_received: f32,
}
