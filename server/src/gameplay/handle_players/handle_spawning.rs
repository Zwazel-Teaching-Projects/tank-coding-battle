use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        player_handling::{PlayerState, TankBodyMarker, TankTurretMarker},
        tank_types::TankType,
    },
    networking::lobby_management::{lobby_management::LobbyManagementSystemParam, InLobby, InTeam},
};

use crate::networking::handle_clients::lib::MyNetworkClient;

#[derive(Debug, Reflect, Event)]
pub struct RespawnPlayerTrigger;

pub fn respawn_player(
    trigger: Trigger<RespawnPlayerTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut body_query: Query<(
        &mut Transform,
        &mut PlayerState,
        &MyNetworkClient,
        &InTeam,
        &InLobby,
        &TankType,
        &TankBodyMarker,
    )>,
    mut turret_query: Query<&mut Transform, (With<TankTurretMarker>, Without<TankBodyMarker>)>,
    tank_configs: TankConfigSystemParam,
) {
    let client_entity = trigger.entity();

    if let Ok((
        mut tank_transform,
        mut player_state,
        client,
        client_team,
        client_in_lobby,
        tank_type,
        tank_body_marker,
    )) = body_query.get_mut(client_entity)
    {
        *player_state = PlayerState::Alive;

        let lobby = lobby_management
            .get_lobby(client_in_lobby.0)
            .expect("Failed to get lobby");
        let map = &lobby
            .map_config
            .as_ref()
            .expect("Failed to get map config")
            .map;
        let spawn_point = client
            .assigned_spawn_point
            .expect(format!("Failed to get assigned spawn point for client {:?}", client).as_str());

        let mut turret_transform = turret_query
            .get_mut(
                tank_body_marker
                    .turret
                    .expect("Failed to get turret entity"),
            )
            .expect("Failed to get turret transform");
        turret_transform.rotation = Quat::IDENTITY;

        let spawn_point_position = map.get_spawn_point_position(client_team, spawn_point);
        let spawn_point_rotation = map.get_spawn_point_rotation(client_team, spawn_point);

        if let Some(spawn_point_position) = spawn_point_position {
            let tank_config = tank_configs
                .get_tank_type_config(tank_type)
                .expect("Failed to get tank config");
            tank_transform.translation =
                spawn_point_position + Vec3::new(0.0, tank_config.size.y, 0.0);
        } else {
            error!(
                "Failed to get spawn point position for team {} and spawn point {}",
                client_team.0, spawn_point
            );
        }

        if let Some(spawn_point_rotation) = spawn_point_rotation {
            tank_transform.rotation = spawn_point_rotation;
        } else {
            error!(
                "Failed to get spawn point rotation for team {} and spawn point {}",
                client_team.0, spawn_point
            );
        }
    } else {
        error!(
            "Failed to get tank transform for client {:?}",
            client_entity
        );
    }
}
