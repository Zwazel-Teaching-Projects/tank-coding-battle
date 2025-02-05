use bevy::prelude::*;

use crate::networking::{
    handle_clients::lib::ClientHasBeenDespawnedTrigger,
    lobby_management::{lobby_management::LobbyManagementSystemParam, InLobby},
};

use super::lib::{ClientDisconnectedTrigger, MyNetworkClient};

pub fn handle_client_disconnects(
    disconnected_client: Trigger<ClientDisconnectedTrigger>,
    clients: Query<(Entity, &MyNetworkClient, Option<&InLobby>)>,
    mut commands: Commands,
    mut lobby_management: LobbyManagementSystemParam,
) {
    let disconnected_client = **disconnected_client;
    let (networked_entity, networked_client, in_lobby) = clients.get(disconnected_client).unwrap();

    info!(
        "Client disconnected: {:?} ({})",
        networked_client.name, networked_client.address
    );

    if let Some(local_client) = networked_client.my_local_client {
        commands.entity(local_client).despawn_recursive();
    }

    commands.entity(networked_entity).despawn_recursive();

    if let Some(in_lobby) = in_lobby {
        info!(
            "Client was in lobby: {:?}, removing from lobby...",
            in_lobby
        );
        lobby_management.remove_player_from_lobby(networked_entity, **in_lobby, &mut commands);
    }

    commands.trigger(ClientHasBeenDespawnedTrigger);
}
