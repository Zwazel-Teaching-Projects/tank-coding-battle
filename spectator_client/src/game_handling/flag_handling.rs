use bevy::{color::palettes::css::WHITE, prelude::*};
use shared::{
    game::{
        collision_handling::components::{Collider, WantedTransform},
        flag::{FlagMarker, FlagState},
    },
    networking::messages::{
        message_container::GameStateTrigger, message_data::game_starts::GameStarts,
    },
};

use super::entity_mapping::MyEntityMapping;

pub fn update_flag_state_on_game_state_update(
    trigger: Trigger<GameStateTrigger>,
    game_config: Res<GameStarts>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entity_mapping: ResMut<MyEntityMapping>,
    mut existing_flags: Query<
        (Entity, &mut WantedTransform, &mut FlagState, &Collider),
        With<FlagMarker>,
    >,
) {
    let game_state = &(**trigger.event());

    game_state
        .flag_states
        .iter()
        .for_each(|(server_side_flag_entity, server_side_flag_state)| {
            let client_side_flag_entity = entity_mapping.map_entity(*server_side_flag_entity);

            // If flag already exists on the client, update its position and state.
            if let Ok((_, mut existing_transform, mut existing_state, existing_collider)) =
                existing_flags.get_mut(client_side_flag_entity)
            {
                existing_transform.translation = server_side_flag_state.transform.translation
                    + Vec3::new(0.0, existing_collider.half_size.y, 0.0);
                existing_transform.rotation = server_side_flag_state.transform.rotation;

                // If server state is "carried", map the carried entity to the client entity.
                if let FlagState::Carried(server_carried_entity) = &server_side_flag_state.state {
                    let client_carried_entity = entity_mapping.map_entity(*server_carried_entity);
                    *existing_state = FlagState::Carried(client_carried_entity);
                } else {
                    *existing_state = server_side_flag_state.state.clone();
                }
            } else {
                // Create a new flag if it doesn't exist yet on the client.
                let team_color = game_config
                    .team_configs
                    .get(&server_side_flag_state.team)
                    .map(|config| Color::from(config.color.clone()))
                    .unwrap_or(WHITE.into());
                let flag_size = server_side_flag_state.collider_size;

                let new_client_side_flag_entity = commands
                    .spawn((
                        Name::new(format!(
                            "Flag_{}_{}",
                            server_side_flag_state.team, server_side_flag_state.flag_base_id
                        )),
                        Mesh3d(meshes.add(Cuboid::new(flag_size.x, flag_size.y, flag_size.z))),
                        MeshMaterial3d(materials.add(StandardMaterial {
                            base_color: team_color,
                            ..Default::default()
                        })),
                        WantedTransform(Transform::from_translation(Vec3::new(
                            0.0,
                            flag_size.y / 2.0,
                            0.0,
                        ))),
                        FlagMarker {
                            base: entity_mapping.map_entity(server_side_flag_state.flag_base_id),
                        },
                        server_side_flag_state.state.clone(),
                        Collider {
                            half_size: flag_size / 2.0,
                            max_slope: 0.0,
                        },
                    ))
                    .id();

                entity_mapping
                    .mapping
                    .insert(*server_side_flag_entity, new_client_side_flag_entity);
            }
        });
}
