use bevy::prelude::*;
use handle_clients::HandleClientsPlugin;
use handle_messages::HandleMessagesPlugin;
use lib::{MyConnectedClients, MyTcpListener};
use networking_state::MyNetworkingState;
use shared::MySharedPlugin;
use std::net::TcpListener;
use system_sets::MyNetworkingSet;

pub mod handle_clients;
pub mod handle_messages;
pub mod lib;
pub mod networking_state;
pub mod shared;
pub mod system_sets;

use crate::{config::MyConfig, main_state::MyMainState};

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MyNetworkingState>()
            .configure_sets(
                PreUpdate,
                (
                    MyNetworkingSet::AcceptConnections,
                    MyNetworkingSet::IncomingMessages,
                    MyNetworkingSet::OutgoingMessages,
                )
                    .run_if(in_state(MyNetworkingState::Running))
                    .chain(),
            )
            .insert_resource(MyConnectedClients::default())
            .add_plugins((MySharedPlugin, HandleClientsPlugin, HandleMessagesPlugin))
            .add_systems(OnEnter(MyMainState::Ready), setup_listener);
    }
}

fn setup_listener(
    mut commands: Commands,
    config: Res<MyConfig>,
    mut networking_state: ResMut<NextState<MyNetworkingState>>,
) {
    let listener = TcpListener::bind(format!("{:}:{:}", config.server_ip, config.server_port))
        .expect(
            format!(
                "Failed to bind to port {} on {}",
                config.server_port, config.server_ip
            )
            .as_str(),
        );
    info!("TCP server listening on {}", listener.local_addr().unwrap());

    // Set to non-blocking so `accept()` won't block the main thread
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking mode");

    commands.insert_resource(MyTcpListener { listener });
    networking_state.set(MyNetworkingState::Running);
}
