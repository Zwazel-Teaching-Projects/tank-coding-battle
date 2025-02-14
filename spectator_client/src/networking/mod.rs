use std::net::TcpStream;

use bevy::prelude::*;
use message_handling::MyMessageHandlingPlugin;
use shared::{
    main_state::MyMainState, networking::messages::message_queue::ImmediateOutMessageQueue,
};

pub mod connect;
pub mod first_contact;
pub mod message_handling;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MyMessageHandlingPlugin,))
            .add_systems(OnEnter(MyMainState::Ready), (connect::connect_to_server,))
            .add_observer(first_contact::send_first_contact);
    }
}

#[derive(Component, Debug, Deref, DerefMut)]
#[require(ImmediateOutMessageQueue)]
pub struct MyNetworkStream(pub TcpStream);
