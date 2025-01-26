use bevy::prelude::*;

use super::lib::MyTcpListener;

pub fn server_running(my_listener: Option<Res<MyTcpListener>>) -> bool {
    my_listener.is_some()
}
