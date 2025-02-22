use bevy::prelude::*;
use shared::{
    game::{common_components::TickBasedDespawnTimer, projectile_handling::ProjectileMarker},
    networking::lobby_management::MyLobby,
};

use crate::gameplay::triggers::{
    FinishedNextSimulationStepTrigger, StartNextSimulationStepTrigger,
    StartNextTickProcessingTrigger,
};

pub fn handle_despawn_timer(
    trigger: Trigger<StartNextTickProcessingTrigger>,
    mut lobby: Query<&mut MyLobby>,
    mut despawn_timer: Query<(Entity, &mut TickBasedDespawnTimer)>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let mut lobby = lobby.get_mut(lobby_entity).expect("Failed to get lobby");

    lobby.projectiles.retain(|projectile| {
        if let Ok((entity, mut despawn_timer)) = despawn_timer.get_mut(*projectile) {
            if despawn_timer.ticks_left > 0 {
                despawn_timer.ticks_left -= 1;
                true
            } else {
                commands.entity(entity).despawn_recursive();
                false
            }
        } else {
            false
        }
    });
}

// TODO: Check for collisions and out of bounds of world!!!
pub fn move_projectiles(
    trigger: Trigger<StartNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    mut projectiles: Query<(&mut Transform, &ProjectileMarker)>,
) {
    let lobby_entity = trigger.entity();

    let lobby = lobby.get(lobby_entity).expect("Failed to get lobby");

    for projectile in lobby.projectiles.iter() {
        let (mut transform, projectile) = projectiles
            .get_mut(*projectile)
            .expect("Failed to get projectile");

        let rotation = transform.rotation;
        transform.translation += rotation * Vec3::new(0.0, 0.0, projectile.speed);
    }
}

pub fn despawn_out_of_bounds(
    trigger: Trigger<FinishedNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    projectiles: Query<&Transform, With<ProjectileMarker>>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();

    let lobby = lobby.get(lobby_entity).expect("Failed to get lobby");
    let map = &lobby
        .map_config
        .as_ref()
        .expect("Failed to get map config")
        .map;

    for projectile_entity in lobby.projectiles.iter() {
        let transform = projectiles
            .get(*projectile_entity)
            .expect("Failed to get projectile");

        if !map.is_inside_bounds(transform.translation) {
            commands.entity(*projectile_entity).despawn_recursive();
        }
    }
}
