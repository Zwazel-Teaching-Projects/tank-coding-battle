use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use bevy::{prelude::*, utils::HashMap};
use tokio::net::TcpStream;

#[derive(Resource, Default, Clone, Deref, DerefMut)]
pub struct MyConnectedClients(pub Arc<Mutex<HashMap<SocketAddr, TcpStream>>>);
