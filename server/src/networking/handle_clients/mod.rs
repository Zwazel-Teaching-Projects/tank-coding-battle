use bevy::prelude::*;
use handle_connect::accept_connections_system;
use handle_disconnect::handle_client_disconnects;
use lib::{ClientConnectedEvent, MyLocalClient};
use shared::networking::networking_system_sets::MyNetworkingSet;

mod handle_connect;
mod handle_disconnect;
pub mod lib;

pub struct HandleClientsPlugin;

impl Plugin for HandleClientsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MyLocalClient>()
            .add_event::<ClientConnectedEvent>()
            .add_systems(
                Update,
                (accept_connections_system,).in_set(MyNetworkingSet::AcceptConnections),
            )
            .add_observer(handle_client_disconnects);
    }
}
