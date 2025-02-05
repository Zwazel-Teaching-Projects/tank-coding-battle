use bevy::{ecs::system::SystemParam, prelude::*};
use shared::networking::messages::message_container::FirstContactTrigger;

use crate::networking::{
    handle_clients::lib::{AwaitingFirstContact, ClientDisconnectedTrigger},
    lobby_management::{InLobby, MyLobby},
};

use super::MyLobbies;

pub fn handle_awaiting_first_contact(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut AwaitingFirstContact)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in clients.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            info!("Client {:?} timed out waiting for first contact", entity);
            commands.trigger(ClientDisconnectedTrigger(entity));
        }
    }
}

#[derive(SystemParam)]
pub struct LobbyManagementSystemParam<'w, 's> {
    lobby_resource: ResMut<'w, MyLobbies>,
    lobby_entities: Query<'w, 's, &'static mut MyLobby>,
}

impl<'w, 's> LobbyManagementSystemParam<'w, 's> {
    pub fn get_or_insert_lobby(
        &mut self,
        lobby_id: &str,
        commands: &mut Commands,
    ) -> (Entity, Mut<MyLobby>) {
        let lobby_entity = self
            .lobby_resource
            .lobbies
            .entry(lobby_id.to_string())
            .or_insert(
                commands
                    .spawn(MyLobby {
                        name: lobby_id.to_string(),
                        players: Vec::new(),
                    })
                    .id(),
            );

        (
            *lobby_entity,
            self.lobby_entities.get_mut(*lobby_entity).unwrap(),
        )
    }

    pub fn remove_player_from_lobby(&mut self, player: Entity, lobby_id: &str) {
        if let Some(lobby_entity) = self.lobby_resource.lobbies.get(lobby_id) {
            if let Ok(mut lobby) = self.lobby_entities.get_mut(*lobby_entity) {
                lobby.players.retain(|&x| x != player);
            } else {
                error!(
                    "Failed to get lobby for lobby id: {}, cannot remove player {} from lobby",
                    lobby_id, player
                );
            }
        } else {
            error!(
                "Failed to get lobby entity for lobby id: {}, cannot remove player {} from lobby",
                lobby_id, player
            );
        }
    }
}

// Proof of concept for handling a message using an observer
// We can even make targeted ones and only trigger for specific clients!
pub fn handle_first_contact_message(
    trigger: Trigger<FirstContactTrigger>,
    mut commands: Commands,
    mut lobby_management: LobbyManagementSystemParam,
) {
    let message = &trigger.message;
    let sender = trigger.sender;
    info!(
        "Received first contact message: {:?} from {:?}",
        message, sender
    );

    // get or insert lobby
    let (lobby_entity, mut lobby) =
        lobby_management.get_or_insert_lobby(&message.lobby_id, &mut commands);
    lobby.players.push(sender);

    commands
        .entity(sender)
        .remove::<AwaitingFirstContact>()
        .insert(InLobby(lobby_entity));
}
