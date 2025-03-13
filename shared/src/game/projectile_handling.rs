use bevy::prelude::*;

use crate::asset_handling::config::TankConfigSystemParam;

use super::{
    collision_handling::components::{Collider, WantedTransform},
    common_components::TickBasedDespawnTimer,
    tank_types::TankType,
};

#[derive(Debug, Component, Reflect, Clone, PartialEq)]
#[reflect(Component)]
#[require(WantedTransform)]
pub struct ProjectileMarker {
    pub speed: f32,
    pub damage: f32,
    pub owner: Entity,

    pub just_spawned: bool,
}

pub fn setup_projectile(
    trigger: Trigger<OnAdd, ProjectileMarker>,
    mut commands: Commands,
    tank_configs: TankConfigSystemParam,
    tank_type: Query<&TankType>,
    projectile: Query<&ProjectileMarker>,
) {
    let projectile_entity = trigger.entity();
    let projectile = projectile
        .get(projectile_entity)
        .expect("ProjectileMarker should exist");

    let tank_type = tank_type
        .get(projectile.owner)
        .expect("TankType should exist");
    let tank_config = tank_configs
        .get_tank_type_config(tank_type)
        .expect("TankConfig should exist");

    commands.entity(projectile_entity).insert((
        TickBasedDespawnTimer {
            ticks_left: tank_config.projectile_lifetime,
        },
        Collider {
            half_size: tank_config.projectile_size / 2.0,
            max_slope: 0.0,
            height_offset: 0.0,
        },
    ));
}
