use bevy::prelude::*;
use tokio::net::TcpListener;

#[derive(Resource)]
pub struct MyTcpListener(pub TcpListener);
