use bevy::{ecs::system::SystemParam, prelude::*, utils::Entry};

use crate::{
    asset_handling::config::ServerConfig,
    game::{game_state::LobbyGameState, player_handling::PlayerState},
    networking::{
        lobby_management::PlayerRemovedFromLobbyTrigger,
        messages::message_data::first_contact::ClientType,
    },
};

use super::{MyLobbies, MyLobby};

#[derive(Debug, Default, Clone)]
pub struct LobbyManagementArgument {
    pub lobby: Option<Entity>,
    pub sender: Option<Entity>,
    pub target_player: Option<Entity>,
    pub team_name: Option<String>,
    pub sender_state: Option<PlayerState>,
}

#[derive(SystemParam)]
pub struct LobbyManagementSystemParam<'w, 's> {
    pub lobby_resource: ResMut<'w, MyLobbies>,
    pub lobby_entities: Query<'w, 's, (Entity, &'static mut MyLobby, &'static mut LobbyGameState)>,
}

impl<'w, 's> LobbyManagementSystemParam<'w, 's> {
    pub fn get_or_insert_lobby_entity(
        &mut self,
        lobby_id: &str,
        map_name: Option<&str>,
        commands: &mut Commands,
        server_config: &ServerConfig,
    ) -> Result<Entity, ()> {
        let lobby_entity_entry = self.lobby_resource.lobbies.entry(lobby_id.to_string());

        match lobby_entity_entry {
            Entry::Occupied(entry) => Ok(*entry.get()),
            Entry::Vacant(entry) => {
                if let Some(map_name) = map_name {
                    let map_name = map_name.to_string();

                    let entity = commands
                        .spawn((
                            Name::new(format!("Lobby_{}_{}", lobby_id, map_name)),
                            MyLobby::new(lobby_id.to_string(), map_name, server_config.tick_rate),
                        ))
                        .id();

                    entry.insert(entity);

                    Ok(entity)
                } else {
                    error!("Failed to get map name for lobby: {} (lobby doesn't exist and should be created, but needs a map name for that!)", lobby_id);
                    return Err(());
                }
            }
        }
    }

    pub fn remove_player_from_lobby(
        &mut self,
        player: Entity,
        lobby: Entity,
        commands: &mut Commands,
    ) {
        if let Ok((_, mut lobby, _)) = self.lobby_entities.get_mut(lobby) {
            lobby
                .players
                .retain(|(_, x, _)| if *x == player { false } else { true });

            lobby
                .spectators
                .retain(|&x| if x == player { false } else { true });

            // Also remove from team
            if let Some(ref mut map_config) = &mut lobby.map_config {
                map_config.remove_player_from_team(player);
            }

            commands.trigger_targets(PlayerRemovedFromLobbyTrigger, player);
        } else {
            error!(
                "Failed to get lobby for lobby entity: {}, cannot remove player {} from lobby",
                lobby, player
            );
        }

        self.cleanup_lobbies(commands);
    }

    fn cleanup_lobbies(&mut self, commands: &mut Commands) {
        self.lobby_resource.lobbies.retain(|_, &mut entity| {
            if let Ok((_, lobby, _)) = self.lobby_entities.get_mut(entity) {
                // Count only normal players (ignoring dummy players) based on client type.
                let normal_player_count = lobby
                    .players
                    .iter()
                    .filter(|(_, _, client_type)| client_type != &ClientType::Dummy)
                    .count();

                // If there are no spectators and no normal players, despawn the lobby.
                if normal_player_count == 0 && lobby.spectators.is_empty() {
                    info!(
                        "Despawning lobby entity \"{}\" with name \"{}\" as it has no normal players or spectators",
                        entity, lobby.lobby_name
                    );
                    commands.entity(entity).despawn_recursive();

                    // Cleanup dummies
                    for (_, player, _) in lobby.players.iter() {
                        commands.entity(*player).despawn_recursive();
                    }

                    false
                } else {
                    true
                }
            } else {
                error!(
                    "Failed to get lobby for lobby entity: {}, cannot cleanup",
                    entity
                );
                false
            }
        });
    }

    pub fn remove_lobby(&mut self, lobby: Entity, commands: &mut Commands) {
        self.lobby_resource.lobbies.retain(|_, &mut entity| {
            if entity == lobby {
                if let Ok((lobby_entity, lobby, _)) = self.lobby_entities.get_mut(lobby) {
                    info!(
                        "Despawning lobby entity \"{}\" with name \"{}\"",
                        lobby_entity, lobby.lobby_name
                    );

                    // Loop through players and spectators and remove them from the lobby (spectators are only Entity, while players are (String, Entity))
                    for player in lobby
                        .players
                        .iter()
                        .map(|(_, player, _)| player)
                        .chain(lobby.spectators.iter())
                    {
                        info!(
                            "Removing player/spectator {} from lobby {}...",
                            player, lobby_entity
                        );
                        commands.trigger_targets(PlayerRemovedFromLobbyTrigger, *player);
                    }

                    commands.entity(lobby_entity).despawn_recursive();
                } else {
                    error!(
                        "Failed to get lobby for lobby entity: {}, cannot remove lobby",
                        lobby
                    );
                }
                false
            } else {
                true
            }
        });
    }

    pub fn get_lobby(&self, lobby: Entity) -> Result<&MyLobby, String> {
        self.lobby_entities
            .get(lobby)
            .map(|(_, lobby, _)| lobby)
            .map_err(|_| format!("Failed to get lobby for lobby entity: {}", lobby))
    }

    pub fn get_lobby_mut(&mut self, lobby: Entity) -> Result<Mut<MyLobby>, String> {
        self.lobby_entities
            .get_mut(lobby)
            .map(|(_, lobby, _)| lobby)
            .map_err(|_| format!("Failed to get lobby for lobby entity: {}", lobby))
    }

    pub fn get_lobby_gamestate(&self, lobby: Entity) -> Result<&LobbyGameState, String> {
        self.lobby_entities
            .get(lobby)
            .map(|(_, _, game_state)| game_state)
            .map_err(|_| format!("Failed to get game state for lobby entity: {}", lobby))
    }

    pub fn get_lobby_gamestate_mut(
        &mut self,
        lobby: Entity,
    ) -> Result<Mut<LobbyGameState>, String> {
        self.lobby_entities
            .get_mut(lobby)
            .map(|(_, _, game_state)| game_state)
            .map_err(|_| format!("Failed to get game state for lobby entity: {}", lobby))
    }

    pub fn targets_get_players_in_lobby(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        self.get_lobby(arg.lobby.ok_or("No lobby provided")?)
            .map(|lobby| {
                lobby
                    .players
                    .iter()
                    .filter(|(_, player, _)| Some(player) != arg.sender.as_ref())
                    .map(|(_, player, _)| *player)
                    .collect()
            })
    }

    pub fn targets_get_players_and_spectators_in_lobby(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        self.get_lobby(arg.lobby.ok_or("No lobby provided")?)
            .map(|lobby| {
                lobby
                    .players
                    .iter()
                    .map(|(_, player, _)| *player)
                    .chain(lobby.spectators.iter().cloned())
                    .filter(|&player| Some(player) != arg.sender)
                    .collect()
            })
    }

    pub fn targets_get_spectators_in_lobby(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        self.get_lobby(arg.lobby.ok_or("No lobby provided")?)
            .map(|lobby| {
                lobby
                    .spectators
                    .iter()
                    .filter(|&&player| Some(player) != arg.sender)
                    .cloned()
                    .collect()
            })
    }

    pub fn targets_get_players_in_lobby_team(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        let team_name = arg.clone().team_name.ok_or("No team name provided")?;

        self.get_lobby(arg.lobby.ok_or("No lobby provided")?)
            .and_then(|lobby| {
                lobby
                    .map_config
                    .as_ref()
                    .ok_or(format!(
                        "Map config not found in lobby {}",
                        lobby.lobby_name
                    ))
                    .and_then(|map_config| {
                        if let Some(team) = map_config.get_team(&team_name) {
                            Ok(team
                                .players
                                .iter()
                                // Filtering out the sender
                                .filter(|&&player| Some(player) != arg.sender)
                                .cloned()
                                .collect())
                        } else {
                            Err(format!(
                                "Team {} not found in lobby {}",
                                team_name, lobby.lobby_name
                            ))
                        }
                    })
            })
    }

    pub fn targets_get_single_player(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        if arg.target_player == arg.sender {
            return Err("Target player cannot be the sender".to_string());
        }
        arg.target_player
            .map(|player| Ok(vec![player]))
            .unwrap_or(Err("No target player provided".to_string()))
    }

    /// Returns an empty vec. this is a workaround for the "ServerOnly" message target
    pub fn targets_get_empty(&self, _arg: LobbyManagementArgument) -> Result<Vec<Entity>, String> {
        Ok(Vec::new())
    }

    pub fn targets_get_lobby_directly(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        Ok(vec![arg.lobby.ok_or("No lobby provided")?])
    }

    pub fn targets_get_self(&self, arg: LobbyManagementArgument) -> Result<Vec<Entity>, String> {
        arg.sender
            .map(|sender| Ok(vec![sender]))
            .unwrap_or(Err("No sender provided".to_string()))
    }
}
