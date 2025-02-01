use bevy::prelude::*;

use super::lib::{ClientDisconnectedTrigger, MyNetworkClient};

pub fn handle_client_disconnects(
    disconnected_client: Trigger<ClientDisconnectedTrigger>,
    clients: Query<(Entity, &MyNetworkClient)>,
    mut commands: Commands,
) {
    let disconnected_client = **disconnected_client;
    let (networked_entity, networked_client) = clients.get(disconnected_client).unwrap();

    info!(
        "Client disconnected: {:?} ({})",
        networked_client.name, networked_client.address
    );

    if let Some(local_client) = networked_client.my_local_client {
        commands.entity(local_client).despawn_recursive();
    }

    commands.entity(networked_entity).despawn_recursive();
}
