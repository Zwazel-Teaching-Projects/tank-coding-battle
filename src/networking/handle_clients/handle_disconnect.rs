use bevy::prelude::*;

use crate::networking::lib::MyConnectedClients;

use super::lib::ClientDisconnected;

pub fn handle_client_disconnects(
    disconnected_client: Trigger<ClientDisconnected>,
    mut connections: ResMut<MyConnectedClients>,
) {
    let disconnected_client = **disconnected_client;

    info!("Client disconnected: {}", disconnected_client);

    connections.streams.remove(&disconnected_client);
}
