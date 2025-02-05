use bevy::{ecs::system::SystemParam, prelude::*, utils::Entry};

use super::{MyLobbies, MyLobby};

#[derive(SystemParam)]
pub struct LobbyManagementSystemParam<'w, 's> {
    lobby_resource: ResMut<'w, MyLobbies>,
    lobby_entities: Query<'w, 's, (Entity, &'static mut MyLobby)>,
}

impl<'w, 's> LobbyManagementSystemParam<'w, 's> {
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

    pub fn remove_player_from_lobby(&mut self, player: Entity, lobby: Entity) {
        if let Ok((_, mut lobby)) = self.lobby_entities.get_mut(lobby) {
            lobby.players.retain(|&x| x != player);
        } else {
            error!(
                "Failed to get lobby for lobby entity: {}, cannot remove player {} from lobby",
                lobby, player
            );
        }
    }

    pub fn cleanup_lobbies(&mut self, commands: &mut Commands) {
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
}
