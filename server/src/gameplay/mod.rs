use bevy::prelude::*;
use handle_players::HandlePlayersPlugin;
use shared::networking::{
    lobby_management::{lobby_management::LobbyManagementSystemParam, LobbyState, MyLobby},
    messages::{
        message_container::{MessageContainer, MessageTarget, NetworkMessageType},
        message_queue::OutMessageQueue,
    },
};
use simulation::run_next_simulation_tick;
use system_sets::MyGameplaySet;
use tick_systems::TickSystemsPlugin;
use triggers::{NextSimulationStepDoneTrigger, SendOutgoingMessagesTrigger};

pub mod handle_players;
pub mod simulation;
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
        .add_systems(Update, start_game)
        .add_plugins((TickSystemsPlugin, HandlePlayersPlugin))
        .add_observer(add_triggers_to_lobby);
    }
}

fn start_game(mut lobbies: Query<&mut MyLobby>) {
    for mut lobby in lobbies.iter_mut() {
        if lobby.state == LobbyState::InProgress {
            continue;
        }

        if lobby.players.len() < 1 {
            continue;
        }
        lobby.state = LobbyState::InProgress;
        info!("Game for lobby {} started", lobby.lobby_name);
    }
}

fn add_triggers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(add_current_game_state_to_message_queue)
        .observe(run_next_simulation_tick);
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

    info!(
        "Sending game state of lobby {} to clients",
        lobby.lobby_name
    );

    for player_entity in lobby.players.iter() {
        let mut out_message_queue = out_message_queues
            .get_mut(*player_entity)
            .expect("Failed to get client");

        let message = MessageContainer::new(
            MessageTarget::Client,
            NetworkMessageType::GameState(lobby.game_state.clone()),
        );
        out_message_queue.push_back(message);
    }

    commands.trigger_targets(SendOutgoingMessagesTrigger, lobby_entity);
}
