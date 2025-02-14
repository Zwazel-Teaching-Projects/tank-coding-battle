use std::net::TcpStream;

use bevy::prelude::*;
use shared::main_state::MyMainState;

pub mod connect;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MyMainState::Ready), (connect::connect_to_server,));
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct MyNetworkStream(pub TcpStream);
