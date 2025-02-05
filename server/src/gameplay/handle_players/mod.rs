use bevy::prelude::*;

use crate::networking::handle_clients::lib::{ClientConnectedTrigger, MyNetworkClient};

use super::gameplay_state::MyGameplayState;

pub struct HandlePlayersPlugin;

impl Plugin for HandlePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_new_player).add_systems(
            Update,
            start_game.run_if(in_state(MyGameplayState::WaitingForBots)),
        );
    }
}

fn spawn_new_player(new_client: Trigger<ClientConnectedTrigger>) {
    println!(
        "New player connected: {:?}, spawning tank (not implemented)",
        new_client.0
    );
}

/*
   TODO: Start only when all bots are ready / enough bots are connected / someone specifically starts it
*/
fn start_game(mut state: ResMut<NextState<MyGameplayState>>, clients: Query<&MyNetworkClient>) {
    if clients.iter().count() >= 1 {
        state.set(MyGameplayState::Running);
    }
}
