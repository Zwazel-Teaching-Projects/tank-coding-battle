use bevy::{color::palettes::css::WHITE, ecs::entity::EntityHashSet, prelude::*};
use shared::{
    game::{
        collision_handling::components::WantedTransform, projectile_handling::ProjectileMarker,
        tank_types::TankType,
    },
    networking::{
        lobby_management::InTeam,
        messages::{message_container::GameStateTrigger, message_data::game_starts::GameStarts},
    },
};

use super::{entity_mapping::MyEntityMapping, DelayedDespawn};

pub fn handle_projectile_on_game_state_update(
    trigger: Trigger<GameStateTrigger>,
    game_config: Res<GameStarts>,
    mut commands: Commands,
    mut entity_mapping: ResMut<MyEntityMapping>,
    players: Query<(&InTeam, &TankType)>,
    mut existing_projectiles: Query<(Entity, &mut WantedTransform), With<ProjectileMarker>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let game_state = &(**trigger.event());
    let current_tick = game_state.tick;

    // Collect all server projectile IDs from the game state.
    let mut server_projectile_ids = EntityHashSet::default();

    game_state.projectile_states.iter().for_each(
        |(server_side_projectile_entity, server_side_projectile_state)| {
            server_projectile_ids.insert(*server_side_projectile_entity);

            let client_side_projectile_entity =
                entity_mapping.map_entity(*server_side_projectile_entity);
            // If projectile already exists on the client, update its position.
            if let Ok((_, mut existing_transform)) =
                existing_projectiles.get_mut(client_side_projectile_entity)
            {
                existing_transform.translation = server_side_projectile_state.transform.translation;
                existing_transform.rotation = server_side_projectile_state.transform.rotation;
            } else {
                // Create a new projectile if it doesn't exist yet on the client.
                let client_side_projectile_owner_id =
                    entity_mapping.map_entity(server_side_projectile_state.owner_id);
                if let Ok((player_in_team, tank_type)) =
                    players.get(client_side_projectile_owner_id)
                {
                    let team_color = game_config
                        .team_configs
                        .get(&player_in_team.0)
                        .map(|config| Color::from(config.color.clone()))
                        .unwrap_or(WHITE.into());
                    let tank_config = game_config
                        .tank_configs
                        .get(tank_type)
                        .expect("Failed to get tank config");

                    let new_client_side_projectile_entity = commands
                        .spawn((
                            Name::new("Projectile"),
                            server_side_projectile_state.transform.clone(),
                            ProjectileMarker {
                                damage: tank_config.projectile_damage,
                                speed: tank_config.projectile_speed,
                                owner: client_side_projectile_owner_id,
                            },
                            Mesh3d(meshes.add(Cuboid::from_size(tank_config.projectile_size))),
                            MeshMaterial3d(materials.add(team_color)),
                            WantedTransform(
                                Transform::from_translation(
                                    server_side_projectile_state.transform.translation,
                                )
                                .with_rotation(server_side_projectile_state.transform.rotation),
                            ),
                        ))
                        .id();
                    entity_mapping.mapping.insert(
                        *server_side_projectile_entity,
                        new_client_side_projectile_entity,
                    );
                } else {
                    warn!(
                        "Failed to get player in team for server side projectile owner {:?}",
                        server_side_projectile_state.owner_id
                    );
                    return;
                }
            }
        },
    );

    // Despawn any projectile on the client that is not present in the game state.
    // in the entity mapping we have the mapping of server side projectile entity to client side projectile entity
    // It's a one-to-one mapping, so we can use the server side projectile entity as the key to get the client side projectile entity.
    // If the server side projectile entity is not present in the game state, we can despawn the client side projectile entity.
    entity_mapping.mapping.retain(
        |server_side_projectile_entity, client_side_projectile_entity| {
            // Only process if the client entity exists in the projectile query.
            if existing_projectiles
                .get(*client_side_projectile_entity)
                .is_ok()
            {
                if !server_projectile_ids.contains(server_side_projectile_entity) {
                    commands
                        .entity(*client_side_projectile_entity)
                        .insert(DelayedDespawn(current_tick + 1));
                    return false;
                }
            }
            true
        },
    );
}
