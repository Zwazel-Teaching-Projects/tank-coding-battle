use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::asset_handling::config::TankConfigSystemParam;

use super::{
    collision_handling::components::{Collider, CollisionLayer, WantedTransform},
    tank_types::TankType,
};

#[derive(Debug, Reflect, Component, Clone, PartialEq, Default, Serialize, Deserialize, Copy)]
#[reflect(Component)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct RespawnTimer(pub u32);

#[derive(Debug, Component, Reflect, Clone, PartialEq, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Health {
    #[deref]
    pub health: f32,
    pub max_health: f32,
}

impl Health {
    pub fn new(max_health: f32) -> Self {
        Self {
            health: max_health,
            max_health,
        }
    }
}

#[derive(Debug, Component, Reflect, Clone, PartialEq, Default)]
#[reflect(Component)]
#[require(ShootCooldown, PlayerState, WantedTransform, Health)]
pub struct TankBodyMarker {
    pub turret: Option<Entity>,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
#[require(Transform)]
pub struct TankTurretMarker {
    pub body: Entity,
}

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
pub struct ShootCooldown {
    pub ticks_left: u32,
    pub ticks_cooldown: u32,
}

impl Default for ShootCooldown {
    fn default() -> Self {
        Self {
            ticks_left: 0,
            ticks_cooldown: 0,
        }
    }
}

pub fn setup_tank_body(
    trigger: Trigger<OnAdd, TankBodyMarker>,
    mut commands: Commands,
    tank_configs: TankConfigSystemParam,
    tank_type: Query<&TankType>,
) {
    let tank_body_entity = trigger.entity();
    let tank_type = tank_type
        .get(tank_body_entity)
        .expect("TankType should exist");
    let tank_config = tank_configs
        .get_tank_type_config(tank_type)
        .expect("TankConfig should exist");

    commands.entity(tank_body_entity).insert((
        Collider {
            half_size: tank_config.size / 2.0,
            max_slope: tank_config.max_slope,
        },
        CollisionLayer::player().with_additional_layers(&[CollisionLayer::FLAG]),
        ShootCooldown {
            ticks_left: 0,
            ticks_cooldown: tank_config.shoot_cooldown,
        },
        Health::new(tank_config.max_health),
    ));
}
