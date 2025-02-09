use bevy::prelude::*;
use handle_clients::HandleClientsPlugin;
use handle_messages::HandleMessagesPlugin;
use lib::MyTcpListener;
use lobby_management::MyLobbyManagementPlugin;
use shared::{
    asset_handling::config::ServerConfigSystemParam,
    main_state::MyMainState,
    networking::{networking_state::MyNetworkingState, networking_system_sets::MyNetworkingSet},
};
use std::net::TcpListener;

pub mod handle_clients;
pub mod handle_messages;
pub mod lib;
pub mod lobby_management;

use crate::gameplay::system_sets::MyGameplaySet;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MyNetworkingSet::ReadingMessages,
                MyNetworkingSet::SendingMessages,
            )
                .after(MyGameplaySet::SimulationStepDone),
        )
        .add_plugins((
            HandleClientsPlugin,
            HandleMessagesPlugin,
            MyLobbyManagementPlugin,
        ))
        .add_systems(OnEnter(MyMainState::Ready), setup_listener);
    }
}

fn setup_listener(
    mut commands: Commands,
    server_config: ServerConfigSystemParam,
    mut networking_state: ResMut<NextState<MyNetworkingState>>,
) {
    let config = server_config.server_config();
    let listener = TcpListener::bind(format!("{:}:{:}", config.ip, config.port))
        .expect(format!("Failed to bind to port {} on {}", config.port, config.ip).as_str());
    info!("TCP server listening on {}", listener.local_addr().unwrap());

    // Set to non-blocking so `accept()` won't block the main thread
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking mode");

    commands.insert_resource(MyTcpListener { listener });
    networking_state.set(MyNetworkingState::Running);
}
