use bevy::{color::palettes::css::WHITE, gltf::GltfMaterialName, prelude::*, utils::HashMap};
use bevy_mod_billboard::BillboardText;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        collision_handling::components::WantedTransform,
        player_handling::{Health, TankBodyMarker, TankTurretMarker},
        tank_types::TankType,
    },
    networking::{lobby_management::InTeam, messages::message_container::GameStartsTrigger},
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
    let mut team_materials = HashMap::default();

    for server_side_client_config in game_start.connected_clients.iter() {
        let team_color = game_start
            .team_configs
            .get(&server_side_client_config.client_team)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(WHITE.into());

        let team_material = materials.add(StandardMaterial {
            base_color: team_color,
            ..Default::default()
        });
        team_materials.insert(server_side_client_config.client_team.clone(), team_material);

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
                tank_type.clone(),
                InTeam(server_side_client_config.client_team.clone()),
                Health::new(server_side_tank_config.max_health),
                VisualOffset(Vec3::new(0.0, tank_y_visual_offset, 0.0)),
                Visibility::Inherited,
                TankBodyMarker { turret: None },
                AwaitsSetup::default(),
            ))
            .with_children(|commands| {
                // Name tag
                commands.spawn((
                    Name::new("Name Tag"),
                    BillboardText::new(&server_side_client_config.client_name),
                    TextFont::from_font(font.clone()).with_font_size(60.0),
                    TextColor(Color::WHITE),
                    TextLayout::new_with_justify(JustifyText::Center),
                    Transform::from_translation(Vec3::new(0.0, 1.0, 0.0))
                        .with_scale(Vec3::splat(0.0085)),
                ));

                // GLTF Scene
                commands.spawn((
                    Name::new("GLTF Scene"),
                    SceneRoot(tank_gltf.scenes[0].clone()),
                ));
            })
            .id();

        entity_mapping.mapping.insert(
            server_side_client_config.client_id,
            client_side_tank_body_entity,
        );
    }

    commands.insert_resource(TeamColorMaterialsResource {
        materials: team_materials,
    });
}

#[derive(Debug, Default, Component, Reflect)]
#[reflect(Component)]
pub struct AwaitsSetup;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct TeamColorMaterialsResource {
    pub materials: HashMap<String, Handle<StandardMaterial>>,
}

pub fn setup_tank(
    mut commands: Commands,
    team_materials: Res<TeamColorMaterialsResource>,
    mut tank: Query<(Entity, &mut TankBodyMarker, &InTeam), With<AwaitsSetup>>,
    children: Query<&Children>,
    names: Query<&Name>,
    mat_query: Query<&GltfMaterialName, With<MeshMaterial3d<StandardMaterial>>>,
) {
    for (awaits_setup_entity, mut tank_body, in_team) in tank.iter_mut() {
        let team_name = in_team.0.clone();
        let team_color = team_materials
            .materials
            .get(&team_name)
            .expect("Failed to get team color material");

        let mut setup_color = false;
        let mut setup_turret = false;
        for child in children.iter_descendants(awaits_setup_entity) {
            // Updating the material of the tank body to match the team color
            if let Ok(material_name) = mat_query.get(child) {
                if material_name.0 != "TeamColor" {
                    continue;
                }

                let material = team_color.clone();
                commands.entity(child).insert(MeshMaterial3d(material));
                setup_color = true;
            }

            // Inserting marker component for the turret
            if let Ok(name) = names.get(child) {
                if **name == *"Turret" {
                    commands.entity(child).insert((
                        TankTurretMarker {
                            body: awaits_setup_entity,
                        },
                        WantedTransform::default(),
                    ));

                    tank_body.turret = Some(child);
                    setup_turret = true;
                }
            }

            if setup_color && setup_turret {
                commands.entity(awaits_setup_entity).remove::<AwaitsSetup>();

                break;
            }
        }
    }
}
