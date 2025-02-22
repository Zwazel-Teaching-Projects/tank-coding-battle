use bevy::{
    color::palettes::css::{GREEN, WHITE},
    prelude::*,
};
use bevy_mod_billboard::BillboardText;
use shared::{
    game::player_handling::{TankBodyMarker, TankTurretMarker},
    networking::{lobby_management::InTeam, messages::message_container::GameStartsTrigger},
};

use crate::game_handling::entity_mapping::MyEntityMapping;

pub fn create_player_visualisation(
    trigger: Trigger<GameStartsTrigger>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut entity_mapping: ResMut<MyEntityMapping>,
) {
    let game_start = trigger.event();
    let map_definition = &game_start.map_definition;
    let tank_configs = &game_start.tank_configs;
    let font = asset_server.load("fonts/FiraSans-Regular.ttf");

    for server_side_client_config in game_start.connected_clients.iter() {
        let team_color = game_start
            .team_configs
            .get(&server_side_client_config.client_team)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(WHITE.into());

        let tank_type = &server_side_client_config.client_tank_type;
        let tank_config = tank_configs
            .get(tank_type)
            .expect("Failed to get tank config");

        let player_position = map_definition
            .get_spawn_point_position(
                &server_side_client_config.client_team,
                server_side_client_config.assigned_spawn_point,
            )
            .expect("Failed to get spawn point position")
            + Vec3::new(0.0, tank_config.size.y, 0.0);

        // Tank Body
        let client_side_tank_body_entity = commands
            .spawn((
                Name::new(server_side_client_config.client_name.clone()),
                Mesh3d(meshes.add(Cuboid::new(
                    tank_config.size.x * 2.0,
                    tank_config.size.y * 2.0,
                    tank_config.size.z * 2.0,
                ))),
                MeshMaterial3d(materials.add(team_color)),
                Transform::from_translation(player_position),
                tank_type.clone(),
                InTeam(server_side_client_config.client_team.clone()),
            ))
            .with_children(|commands| {
                // Name tag
                commands.spawn((
                    BillboardText::new(&server_side_client_config.client_name),
                    TextFont::from_font(font.clone()).with_font_size(60.0),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    Transform::from_translation(Vec3::new(0.0, 1.0, 0.0))
                        .with_scale(Vec3::splat(0.0085)),
                ));

                // Forward marker
                commands.spawn((
                    Name::new("Forward marker"),
                    Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.1))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: GREEN.into(),
                        ..Default::default()
                    })),
                    Transform::from_translation(Vec3::new(0.0, 0.0, tank_config.size.z + 0.2)),
                ));
            })
            .id();

        // Turret
        let client_side_turret_entity = commands
            .spawn((
                Name::new("Turret Root"),
                Transform::from_translation(Vec3::new(0.0, tank_config.size.y, 0.0)),
                TankTurretMarker {
                    body: client_side_tank_body_entity,
                },
            ))
            .with_children(|commands| {
                // Turret, placed a bit in front of the turret root. This is just for visualization.
                // more rectangle, long not wide or tall
                commands.spawn((
                    Name::new("Turret"),
                    Mesh3d(meshes.add(Cuboid::new(0.1, 0.1, 0.5))),
                    MeshMaterial3d(materials.add(StandardMaterial {
                        base_color: team_color,
                        ..Default::default()
                    })),
                    Transform::from_translation(Vec3::new(0.0, 0.0, 0.25)),
                ));
            })
            .id();

        commands
            .entity(client_side_tank_body_entity)
            .insert((TankBodyMarker {
                turret: Some(client_side_turret_entity),
            },))
            .add_child(client_side_turret_entity);

        entity_mapping.mapping.insert(
            server_side_client_config.client_id,
            client_side_tank_body_entity,
        );
    }
}
