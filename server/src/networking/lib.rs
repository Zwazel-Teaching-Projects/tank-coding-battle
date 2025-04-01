use std::net::TcpListener;

use bevy::prelude::*;

// A simple resource holding the listener
#[derive(Resource)]
pub struct MyTcpListener {
    pub listener: TcpListener,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct GameManagerMarker;