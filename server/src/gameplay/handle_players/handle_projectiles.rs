use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::{
            components::{Collider, WantedTransform},
            triggers::{CollidedWithTrigger, CollidedWithWorldTrigger},
        },
        common_components::TickBasedDespawnTimer,
        player_handling::TankBodyMarker,
        projectile_handling::ProjectileMarker,
    },
    networking::lobby_management::MyLobby,
};

use crate::gameplay::triggers::{
    FinishedNextSimulationStepTrigger, StartNextSimulationStepTrigger,
    StartNextTickProcessingTrigger,
};

pub fn colliding_with_entity(
    trigger: Trigger<CollidedWithTrigger>,
    projectile: Query<(&ProjectileMarker, &Transform)>,
    players: Query<(&TankBodyMarker, &Transform, &Collider)>,
    mut commands: Commands,
) {
    // TODO: Apply damage to player
    let projectile_entity = trigger.entity();
    let (projectile, projectile_transform) = projectile
        .get(projectile_entity)
        .expect("Failed to get projectile");
    let collided_with = trigger.event().entity;

    if let Ok((body, body_transform, body_collider)) = players.get(collided_with) {
        let body_half_size = body_collider.half_size;
        println!("Projectile hit player: {:?}", body);

        // Get the relative vector from the body to the projectile in world space.
        let relative = projectile_transform.translation - body_transform.translation;

        // Transform the relative vector into the body's local space.
        let local_pos = body_transform.rotation.inverse() * relative;

        let face_dx = body_half_size.x - local_pos.x.abs();
        let face_dy = body_half_size.y - local_pos.y.abs();
        let face_dz = body_half_size.z - local_pos.z.abs();

        if face_dx < face_dy && face_dx < face_dz {
            // Collision on x-axis (left or right)
            if local_pos.x > 0.0 {
                println!("Collided with the left face."); // Swapped condition
            } else {
                println!("Collided with the right face.");
            }
        } else if face_dy < face_dx && face_dy < face_dz {
            // Collision on y-axis (top or bottom)
            if local_pos.y > 0.0 {
                println!("Collided with the top face.");
            } else {
                println!("Collided with the bottom face.");
            }
        } else {
            // Collision on z-axis (front or back)
            if local_pos.z > 0.0 {
                println!("Collided with the front face.");
            } else {
                println!("Collided with the back face.");
            }
        }

        commands.entity(projectile_entity).despawn_recursive();
    }
}

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

pub fn move_projectiles(
    trigger: Trigger<StartNextSimulationStepTrigger>,
    lobby: Query<&MyLobby>,
    mut projectiles: Query<(&mut WantedTransform, &ProjectileMarker)>,
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

pub fn despawn_on_collision_with_world(
    trigger: Trigger<CollidedWithWorldTrigger>,
    mut commands: Commands,
) {
    commands.entity(trigger.entity()).despawn_recursive();
}
