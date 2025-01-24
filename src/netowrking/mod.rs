use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use bevy_renet::netcode::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;
use lib::MyConnectedClients;

use crate::config::{ConfigLoadState, MyConfig};
use crate::SharedGameState;

mod lib;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RenetServerPlugin, NetcodeServerPlugin))
            .add_systems(
                OnEnter(ConfigLoadState::Loaded),
                (init_renet_server,).chain(),
            )
            .init_resource::<MyConnectedClients>();
    }
}

fn init_renet_server(config: Res<MyConfig>) {}

fn send_messages(shared: Res<SharedGameState>, connected_clients: Res<MyConnectedClients>) {}

/// Process a single client connection.
fn handle_connection(shared: &SharedGameState) {}
