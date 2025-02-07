use bevy::{ecs::system::SystemParam, prelude::*, utils::Entry};

use crate::networking::lobby_management::PlayerRemovedFromLobbyTrigger;

use super::{MyLobbies, MyLobby};

#[derive(Debug, Default, Clone)]
pub struct LobbyManagementArgument {
    pub lobby: Option<Entity>,
    pub sender: Option<Entity>,
    pub target_player: Option<Entity>,
    pub team_name: Option<String>,
    pub team: Option<Entity>,
}

#[derive(SystemParam)]
pub struct LobbyManagementSystemParam<'w, 's> {
    lobby_resource: ResMut<'w, MyLobbies>,
    lobby_entities: Query<'w, 's, (Entity, &'static mut MyLobby)>,
}

impl<'w, 's> LobbyManagementSystemParam<'w, 's> {
    pub fn get_lobby_mut(&mut self, entity: Entity) -> Option<(Entity, Mut<MyLobby>)> {
        self.lobby_entities.get_mut(entity).ok()
    }

    pub fn get_or_insert_lobby_entity(
        &mut self,
        lobby_id: &str,
        player: Entity,
        map_name: Option<&str>,
        commands: &mut Commands,
    ) -> Result<Entity, ()> {
        let lobby_entity_entry = self.lobby_resource.lobbies.entry(lobby_id.to_string());

        match lobby_entity_entry {
            Entry::Occupied(entry) => Ok(*entry.get()),
            Entry::Vacant(entry) => {
                if let Some(map_name) = map_name {
                    let map_name = map_name.to_string();

                    let entity = commands
                        .spawn(MyLobby::new(lobby_id.to_string(), map_name).with_player(player))
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
        if let Ok((_, mut lobby)) = self.lobby_entities.get_mut(lobby) {
            lobby
                .players
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
            if let Ok((_, lobby)) = self.lobby_entities.get_mut(entity) {
                if lobby.players.is_empty() {
                    info!(
                        "Despawning lobby entity \"{}\" with name \"{}\" as it is empty",
                        entity, lobby.name
                    );

                    commands.entity(entity).despawn_recursive();
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
                if let Ok((lobby_entity, lobby)) = self.lobby_entities.get_mut(lobby) {
                    info!(
                        "Despawning lobby entity \"{}\" with name \"{}\"",
                        lobby_entity, lobby.name
                    );

                    for player in lobby.players.iter() {
                        info!("Removing player {} from lobby {}...", player, lobby_entity);
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

    pub fn get_lobby(&self, arg: LobbyManagementArgument) -> Result<(Entity, &MyLobby), String> {
        let lobby = arg.lobby.ok_or("No lobby entity provided")?;
        self.lobby_entities
            .get(lobby)
            .map_err(|_| "Lobby not found".to_string())
    }

    pub fn get_players_in_lobby(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        self.get_lobby(arg).map(|(_, lobby)| lobby.players.clone())
    }

    pub fn get_players_in_lobby_team(
        &self,
        arg: LobbyManagementArgument,
    ) -> Result<Vec<Entity>, String> {
        let team_name = arg.clone().team_name.ok_or("No team name provided")?;

        self.get_lobby(arg).and_then(|(_, lobby)| {
            lobby
                .map_config
                .as_ref()
                .ok_or(format!("Map config not found in lobby {}", lobby.name))
                .and_then(|map_config| {
                    if let Some(team) = map_config.get_team(&team_name) {
                        Ok(team.players.clone())
                    } else {
                        Err(format!(
                            "Team {} not found in lobby {}",
                            team_name, lobby.name
                        ))
                    }
                })
        })
    }

    pub fn get_single_player(&self, arg: LobbyManagementArgument) -> Result<Vec<Entity>, String> {
        arg.target_player
            .map(|player| Ok(vec![player]))
            .unwrap_or(Err("No target player provided".to_string()))
    }

    /// Returns an empty vec. this is a workaround for the "ServerOnly" message target
    pub fn get_empty(&self, _arg: LobbyManagementArgument) -> Result<Vec<Entity>, String> {
        Ok(Vec::new())
    }
}
