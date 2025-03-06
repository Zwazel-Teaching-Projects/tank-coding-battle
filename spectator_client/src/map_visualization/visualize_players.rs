use bevy::{color::palettes::css::WHITE, gltf::GltfMaterialName, prelude::*};
use bevy_mod_billboard::BillboardText;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        collision_handling::components::WantedTransform,
        player_handling::{Health, TankBodyMarker, TankTurretMarker},
    },
    networking::{
        lobby_management::InTeam,
        messages::{message_container::GameStartsTrigger, message_data::game_starts::GameStarts},
    },
};

use crate::{game_handling::entity_mapping::MyEntityMapping, VisualOffset};

pub fn create_player_visualisation(
    trigger: Trigger<GameStartsTrigger>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut entity_mapping: ResMut<MyEntityMapping>,
    client_side_loaded_tank_assets: TankConfigSystemParam,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let game_start = trigger.event();
    let tank_configs = &game_start.tank_configs;
    let font = asset_server.load("fonts/FiraSans-Regular.ttf");

    for server_side_client_config in game_start.connected_clients.iter() {
        let team_color = game_start
            .team_configs
            .get(&server_side_client_config.client_team)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(WHITE.into());

        let tank_type = &server_side_client_config.client_tank_type;
        let server_side_tank_config = tank_configs
            .get(tank_type)
            .expect("Failed to get tank config");
        let client_side_tank_config = client_side_loaded_tank_assets
            .get_tank_type_config(tank_type)
            .expect("Failed to get tank assets");
        let tank_model_handle =
            client_side_loaded_tank_assets.get_tank_model(client_side_tank_config.model.as_ref());
        let tank_gltf = gltf_assets
            .get(tank_model_handle.id())
            .expect("Failed to get tank gltf");
        let tank_y_visual_offset = server_side_tank_config.size.y / 2.0;

        // Tank Body
        let client_side_tank_body_entity = commands
            .spawn((
                Name::new(server_side_client_config.client_name.clone()),
                SceneRoot(tank_gltf.scenes[0].clone()),
                tank_type.clone(),
                InTeam(server_side_client_config.client_team.clone()),
                Health::new(server_side_tank_config.max_health),
                VisualOffset(Vec3::new(0.0, tank_y_visual_offset, 0.0)),
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
            })
            .id();

        // Turret
        let client_side_turret_entity = commands
            .spawn((
                Name::new("Turret Root"),
                TankTurretMarker {
                    body: client_side_tank_body_entity,
                },
                WantedTransform::default(),
                VisualOffset(Vec3::new(0.0, tank_y_visual_offset, 0.0)),
                Visibility::Inherited,
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

pub fn update_tank_color(
    trigger: Trigger<OnAdd, GltfMaterialName>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    tank: Query<&InTeam, With<TankBodyMarker>>,
    parent_query: Query<&Parent>,
    mat_query: Query<(&Name, &MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
    game_start: Res<GameStarts>,
) {
    let added_material_name_entity = trigger.entity();
    let (name, mat_handle, material_name) = mat_query
        .get(added_material_name_entity)
        .expect("Failed to get material name");

    if material_name.0 != "TeamColor" {
        return;
    }

    // I do not understand.
    println!("Name: {:?}, material name: {:?}", name, material_name.0);
    let root = parent_query.root_ancestor(added_material_name_entity);
    println!("Root: {:?}", root);

    if let Ok(in_team) = tank.get(root) {
        println!("Found tank with team: {:?}", in_team.0);
        let team_color = game_start
            .team_configs
            .get(&in_team.0)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(WHITE.into());

        if let Some(material) = materials.get_mut(mat_handle.id()) {
            material.base_color = team_color;
        }
    } else {
        println!("Failed to get tank");
    }
}
