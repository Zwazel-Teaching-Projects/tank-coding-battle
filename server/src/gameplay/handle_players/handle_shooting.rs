use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        player_handling::{ShootCooldown, TankBodyMarker, TankTurretMarker},
        projectile_handling::ProjectileMarker,
        tank_types::TankType,
    },
    networking::{
        lobby_management::{InLobby, MyLobby},
        messages::message_container::ShootCommandTrigger,
    },
};

use crate::gameplay::triggers::StartNextTickProcessingTrigger;

pub fn handle_tank_shooting_command(
    trigger: Trigger<ShootCommandTrigger>,
    mut lobby: Query<&mut MyLobby>,
    mut body: Query<(&TankType, &mut ShootCooldown, &TankBodyMarker, &InLobby)>,
    turret_transform: Query<&GlobalTransform, With<TankTurretMarker>>,
    mut commands: Commands,
) {
    let client_entity = trigger.entity();
    let (_tank_type, mut cooldown, tank_body, in_lobby) = body
        .get_mut(client_entity)
        .expect("Failed to get tank transform");

    if cooldown.ticks_left <= 0 {
        let mut lobby = lobby.get_mut(in_lobby.0).expect("Failed to get lobby");

        let turret_entity = tank_body.turret.expect("Failed to get turret entity");
        let turret_transform = turret_transform
            .get(turret_entity)
            .expect("Failed to get turret transform");

        let bullet_spawn_position = turret_transform.translation();
        let bullet_spawn_rotation = turret_transform.rotation();

        let bullet = commands
            .spawn((
                Name::new("Bullet"),
                Transform::from_translation(bullet_spawn_position)
                    .with_rotation(bullet_spawn_rotation),
                ProjectileMarker {
                    owner: client_entity,
                    damage: 1.0,
                    speed: 0.5,
                },
                in_lobby.clone(),
            ))
            .id();

        lobby.projectiles.push(bullet);

        cooldown.ticks_left = cooldown.ticks_cooldown;
    }
}

pub fn tick_shoot_cooldowns(
    trigger: Trigger<StartNextTickProcessingTrigger>,
    lobby: Query<&MyLobby>,
    mut body: Query<&mut ShootCooldown>,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby.get(lobby_entity).expect("Failed to get lobby");

    for (_, player, _) in lobby.players.iter() {
        if let Ok(mut shoot_cooldown) = body.get_mut(*player) {
            if shoot_cooldown.ticks_left > 0 {
                shoot_cooldown.ticks_left -= 1;
            }
        }
    }
}

pub fn set_timer_for_shooting(
    trigger: Trigger<OnAdd, ShootCooldown>,
    tank_config: TankConfigSystemParam,
    mut tank: Query<(&mut ShootCooldown, &TankType)>,
) {
    let entity = trigger.entity();
    let (mut cooldown, tank_type) = tank.get_mut(entity).expect("Failed to get tank type");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");

    cooldown.ticks_left = tank_config.shoot_cooldown;
    cooldown.ticks_cooldown = tank_config.shoot_cooldown;
}
