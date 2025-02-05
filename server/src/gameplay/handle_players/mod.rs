use bevy::prelude::*;
use team_handling::InTeam;

use crate::networking::handle_clients::lib::ClientConnectedTrigger;

pub mod team_handling;

pub struct HandlePlayersPlugin;

impl Plugin for HandlePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<InTeam>().add_observer(spawn_new_player);
    }
}

fn spawn_new_player(new_client: Trigger<ClientConnectedTrigger>) {
    println!(
        "New player connected: {:?}, spawning tank (not implemented)",
        new_client.0
    );
}
