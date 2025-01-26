use bevy::prelude::*;
use handle_connect::accept_connections_system;
use handle_disconnect::handle_client_disconnects;
use lib::{ClientConnected, ClientDisconnected};

use super::run_conditions::server_running;

mod handle_connect;
mod handle_disconnect;
pub mod lib;

pub struct HandleClientsPlugin;

impl Plugin for HandleClientsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ClientConnected>()
            .add_event::<ClientDisconnected>()
            .add_systems(Update, (accept_connections_system).run_if(server_running))
            .add_observer(handle_client_disconnects);
    }
}
