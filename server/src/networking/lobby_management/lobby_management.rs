use bevy::{ecs::system::SystemParam, prelude::*};

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
        commands: &mut Commands,
    ) -> Entity {
        let lobby_entity = self
            .lobby_resource
            .lobbies
            .entry(lobby_id.to_string())
            .or_insert(
                commands
                    .spawn(MyLobby {
                        name: lobby_id.to_string(),
                        players: vec![player],
                    })
                    .id(),
            );

        *lobby_entity
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
