use bevy::prelude::*;
use handle_players::HandlePlayersPlugin;
use shared::networking::{
    lobby_management::{lobby_management::LobbyManagementSystemParam, MyLobby},
    messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_queue::OutMessageQueue,
    },
};
use simulation::run_next_simulation_tick;
use start_lobby::check_if_lobby_should_start;
use system_sets::MyGameplaySet;
use tick_systems::TickSystemsPlugin;
use triggers::{NextSimulationStepDoneTrigger, SendOutgoingMessagesTrigger};

pub mod handle_players;
pub mod simulation;
pub mod start_lobby;
pub mod system_sets;
mod tick_systems;
pub mod triggers;

pub struct MyGameplayPlugin;

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                (MyGameplaySet::TickTimerProcessing,),
                (
                    MyGameplaySet::IncrementTick,
                    MyGameplaySet::RunSimulationStep,
                    MyGameplaySet::SimulationStepDone,
                )
                    .chain(),
            )
                .chain(),
        )
        .add_systems(Update, check_if_lobby_should_start)
        .add_plugins((TickSystemsPlugin, HandlePlayersPlugin))
        .add_observer(add_observers_to_lobby);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(add_current_game_state_to_message_queue)
        .observe(run_next_simulation_tick)
        .observe(start_lobby::start_lobby);
}

fn add_current_game_state_to_message_queue(
    trigger: Trigger<NextSimulationStepDoneTrigger>,
    lobby_management: LobbyManagementSystemParam,
    mut out_message_queues: Query<&mut OutMessageQueue>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    let lobby = lobby_management
        .get_lobby(lobby_entity)
        .expect("Failed to get lobby");

    for (_, player_entity) in lobby.players.iter() {
        let mut out_message_queue = out_message_queues
            .get_mut(*player_entity)
            .expect("Failed to get client");

        let message = MessageContainer::new(
            MessageTarget::Client(*player_entity),
            NetworkMessageType::GameState(lobby.game_state.clone()),
        );

        // Make sure the game state is sent before any other messages
        out_message_queue.push_front(message);
    }

    commands.trigger_targets(SendOutgoingMessagesTrigger, lobby_entity);
}
