use bevy::prelude::*;
use shared::{
    game::projectile_handling::ProjectileMarker,
    networking::messages::message_container::GameStateTrigger,
};

use super::entity_mapping::MyEntityMapping;
use std::collections::HashSet;

pub fn handle_projectile_on_game_state_update(
    trigger: Trigger<GameStateTrigger>,
    mut commands: Commands,
    mut existing_projectiles: Query<
        (Entity, &mut Transform, &MyEntityMapping),
        With<ProjectileMarker>,
    >,
) {
    let game_state = &(**trigger.event());

    // Collect all server projectile IDs from the game state.
    let mut server_projectile_ids = HashSet::new();

    game_state
        .projectile_states
        .iter()
        .for_each(|(projectile_entity, projectile_state)| {
            server_projectile_ids.insert(*projectile_entity);

            // Check if projectile already exists
            if let Some((_, mut existing_transform, _)) = existing_projectiles
                .iter_mut()
                .find(|(_, _, mapping)| mapping.server_entity == *projectile_entity)
            {
                existing_transform.translation = projectile_state.transform.translation;
                existing_transform.rotation = projectile_state.transform.rotation;
            } else {
                // Create a new projectile if it doesn't exist on the client.
                commands.spawn((
                    Name::new("Projectile"),
                    projectile_state.transform.clone(),
                    MyEntityMapping {
                        server_entity: *projectile_entity,
                    },
                    ProjectileMarker {
                        damage: 0.0, // Placeholder
                        speed: 0.0,  // Placeholder
                        owner: projectile_state.owner_id,
                    },
                ));
            }
        });

    // Despawn any projectile on the client that is not present in the game state.
    for (entity, _, mapping) in existing_projectiles.iter_mut() {
        if !server_projectile_ids.contains(&mapping.server_entity) {
            commands.entity(entity).despawn_recursive();
        }
    }
}
