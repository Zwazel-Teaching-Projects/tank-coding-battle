use bevy::{prelude::*, utils::hashbrown::HashSet};
use shared::{
    game::{
        collision_handling::components::Collider,
        flag::{FlagBaseMarker, FlagMarker, FlagState},
        game_state::{
            ClientState, FlagBaseState, FlagGameState, PersonalizedClientGameState, ProjectileState,
        },
        player_handling::{Health, BotState, ShootCooldown, TankBodyMarker, TankTurretMarker},
        projectile_handling::ProjectileMarker,
        tank_types::TankType,
    },
    networking::{
        lobby_management::{lobby_management::LobbyManagementSystemParam, InTeam, LobbyState},
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_queue::OutMessageQueue,
        },
    },
};

use crate::gameplay::triggers::UpdateClientGameStatesTrigger;

use super::triggers::{
    AddStateUpdateToQueue, SendOutgoingMessagesTrigger, UpdateLobbyGameStateTrigger,
};

pub fn update_lobby_state(
    trigger: Trigger<UpdateLobbyGameStateTrigger>,
    mut lobby_management: LobbyManagementSystemParam,
    tanks: Query<(
        &Transform,
        &TankType,
        &TankBodyMarker,
        &ShootCooldown,
        &Health,
        &BotState,
    )>,
    turrets: Query<&Transform, With<TankTurretMarker>>,
    projectiles: Query<(&Transform, &ProjectileMarker)>,
    flags: Query<(&Transform, &FlagState, &InTeam, &FlagMarker, &Collider)>,
    flag_bases: Query<(&Transform, &FlagBaseMarker, &Collider, &InTeam)>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();

    let player_entities = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby")
        .1
        .players
        .iter()
        .map(|(_, entity, _)| *entity)
        .collect::<Vec<_>>();
    let flag_entities = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby")
        .1
        .flags
        .iter()
        .map(|entity| *entity)
        .collect::<HashSet<_>>();
    let flag_base_entities = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby")
        .1
        .flag_bases
        .iter()
        .map(|entity| *entity)
        .collect::<HashSet<_>>();
    let projectile_entities = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby")
        .1
        .projectiles
        .iter()
        .map(|entity| *entity)
        .collect::<HashSet<_>>();
    let mut lobby_game_state = lobby_management
        .get_lobby_gamestate_mut(lobby_entity)
        .expect("Failed to get lobby game state");

    // Updating client states of all players
    for player_entity in player_entities.iter() {
        let (tank_transform, _tank_type, tank_body, shoot_cooldown, tank_health, player_state) =
            tanks.get(*player_entity).expect("Failed to get tank");

        let relative_turret_transform = turrets
            .get(tank_body.turret.expect("Failed to get turret entity"))
            .expect("Failed to get turret");

        let client_state = lobby_game_state
            .client_states
            .entry(*player_entity)
            .or_insert_with(|| ClientState::new(*player_entity));
        client_state.transform_body = Some(tank_transform.clone());
        client_state.transform_turret = Some(relative_turret_transform.clone());
        client_state.shoot_cooldown = shoot_cooldown.ticks_left;
        client_state.current_health = tank_health.health;
        client_state.state = Some(player_state.clone());
    }

    // Updating states of all projectiles and removing those that are not in the world anymore from the game state
    lobby_game_state
        .projectiles
        .retain(|entity, _| projectile_entities.contains(entity));
    for projectile_entity in projectile_entities.iter() {
        let (projectile_transform, projectile_data) = projectiles
            .get(*projectile_entity)
            .expect("Failed to get projectile transform");

        lobby_game_state
            .projectiles
            .entry(*projectile_entity)
            .and_modify(|state| state.transform = projectile_transform.clone())
            .or_insert_with(|| {
                ProjectileState::new(
                    *projectile_entity,
                    projectile_data.owner,
                    projectile_transform.clone(),
                )
            });
    }

    // Updating states of all flags
    lobby_game_state
        .flags
        .retain(|entity, _| flag_entities.contains(entity));
    for flag_entity in flag_entities.iter() {
        let (flag_transform, flag_state, in_team, flag_marker, flag_collider) =
            flags.get(*flag_entity).expect("Failed to get flag");

        lobby_game_state
            .flags
            .entry(*flag_entity)
            .and_modify(|state| {
                state.transform = flag_transform.clone();
                state.state = flag_state.clone();
            })
            .or_insert_with(|| FlagGameState {
                flag_id: *flag_entity,
                transform: flag_transform.clone(),
                state: flag_state.clone(),
                team: in_team.0.clone(),
                flag_base_id: flag_marker.base,
                collider_size: flag_collider.half_size * 2.0,
            });
    }

    // Updating states of all flag bases
    lobby_game_state
        .flag_bases
        .retain(|entity, _| flag_base_entities.contains(entity));
    for flag_base_entity in flag_base_entities.iter() {
        let (flag_base_transform, flag_base_marker, flag_base_collider, in_team) = flag_bases
            .get(*flag_base_entity)
            .expect("Failed to get flag base");

        lobby_game_state
            .flag_bases
            .entry(*flag_base_entity)
            .and_modify(|state| {
                state.transform = flag_base_transform.clone();
                state.flag_in_base = flag_base_marker.flag_in_base;
            })
            .or_insert_with(|| FlagBaseState {
                flag_in_base: flag_base_marker.flag_in_base,
                collider_size: flag_base_collider.half_size * 2.0,
                flag_id: flag_base_marker.my_flag,
                flag_base_id: *flag_base_entity,
                team: in_team.0.clone(),
                transform: flag_base_transform.clone(),
            });
    }

    commands.trigger_targets(UpdateClientGameStatesTrigger, player_entities);
}

pub fn check_if_client_states_are_all_up_to_date(
    mut lobby_management: LobbyManagementSystemParam,
    client_states: Query<&PersonalizedClientGameState>,
    mut commands: Commands,
) {
    // Go through all lobbies, get their game state, then check for all clients if they have the same tick
    for (entity, mut lobby, game_state) in lobby_management.lobby_entities.iter_mut() {
        match lobby.state {
            LobbyState::InProgress => (),
            _ => continue,
        }

        if lobby.tick_processed == game_state.tick {
            continue;
        }

        let mut all_up_to_date = true;
        for (_, player_entity, _) in lobby.players.iter() {
            let client_state = client_states
                .get(*player_entity)
                .expect("Failed to get client state");

            if client_state.tick != game_state.tick {
                all_up_to_date = false;
            }
        }
        if all_up_to_date {
            lobby.tick_processed = game_state.tick;
            commands.trigger_targets(AddStateUpdateToQueue, entity);
        }
    }
}

pub fn add_current_game_state_to_message_queue(
    trigger: Trigger<AddStateUpdateToQueue>,
    lobby_management: LobbyManagementSystemParam,
    mut out_message_queues: Query<&mut OutMessageQueue>,
    client_states: Query<&PersonalizedClientGameState>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let (_, lobby, _) = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby");
    let lobby_state = lobby_management
        .get_lobby_gamestate(lobby_entity)
        .expect("Failed to get lobby game state");

    // Sending the game state to all players
    for (_, player_entity, _) in lobby.players.iter() {
        let mut out_message_queue = out_message_queues
            .get_mut(*player_entity)
            .expect("Failed to get client out message queue");
        let client_state = client_states
            .get(*player_entity)
            .expect("Failed to get client state");

        let message = MessageContainer::new(
            MessageTarget::Client(*player_entity),
            NetworkMessageType::GameState(client_state.clone().into()),
        );

        // Make sure the game state is sent before any other messages
        out_message_queue.push_front(message);
    }

    // Sending the (global) game state to all spectators
    for spectator_entity in lobby.spectators.iter() {
        if let Ok(mut out_message_queue) = out_message_queues.get_mut(*spectator_entity) {
            let message = MessageContainer::new(
                MessageTarget::Client(*spectator_entity),
                NetworkMessageType::GameState(lobby_state.clone().into()),
            );

            // Make sure the game state is sent before any other messages
            out_message_queue.push_front(message);
        }
    }

    commands.trigger_targets(SendOutgoingMessagesTrigger, lobby_entity);
}
