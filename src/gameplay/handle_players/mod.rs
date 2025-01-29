use bevy::prelude::*;

use crate::networking::handle_clients::lib::ClientConnectedEvent;

use super::gameplay_state::MyGameplayState;

pub struct HandlePlayersPlugin;

impl Plugin for HandlePlayersPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_new_player).add_systems(
            Update,
            start_game.run_if(
                in_state(MyGameplayState::WaitingForBots).and(on_event::<ClientConnectedEvent>),
            ),
        );
    }
}

fn spawn_new_player(new_client: Trigger<ClientConnectedEvent>) {
    println!("New player connected: {:?}, spawning tank", new_client.0);
}

/*
   TODO: Start only when all bots are ready
*/
fn start_game(mut state: ResMut<NextState<MyGameplayState>>) {
    state.set(MyGameplayState::Running);
}
