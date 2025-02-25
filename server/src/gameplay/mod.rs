use bevy::prelude::*;
use handle_collisions::MyCollisionHandlingPlugin;
use handle_players::HandlePlayersPlugin;
use lobby_cleanup::CleanupMarker;
use shared::networking::lobby_management::MyLobby;
use system_sets::MyGameplaySet;
use tick_systems::TickSystemsPlugin;

pub mod game_state_handling;
pub mod handle_collisions;
pub mod handle_players;
pub mod lobby_cleanup;
pub mod process_messages;
pub mod process_messages_when_lobby_not_ready;
pub mod simulation;
pub mod start_lobby;
pub mod system_sets;
mod tick_systems;
pub mod triggers;

pub struct MyGameplayPlugin;

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CleanupMarker>()
            .configure_sets(
                Update,
                (
                    (
                        MyGameplaySet::TickTimerProcessing,
                        MyGameplaySet::ProcessMessagesBeforeLobbyReady,
                    ),
                    (
                        MyGameplaySet::IncrementTick,
                        MyGameplaySet::ProcessCommands,
                        MyGameplaySet::RunSimulationStep,
                        MyGameplaySet::SimulationStepDone,
                        MyGameplaySet::UpdatingGameStates,
                    )
                        .chain(),
                )
                    .chain(),
            )
            .add_plugins((
                TickSystemsPlugin,
                HandlePlayersPlugin,
                MyCollisionHandlingPlugin,
            ))
            .add_systems(
                Update,
                (
                    game_state_handling::check_if_client_states_are_all_up_to_date
                        .in_set(MyGameplaySet::UpdatingGameStates),
                    process_messages_when_lobby_not_ready::process_messages_before_lobby_is_ready
                        .in_set(MyGameplaySet::ProcessMessagesBeforeLobbyReady),
                ),
            )
            .add_observer(add_observers_to_lobby)
            .add_observer(lobby_cleanup::cleanup_lobby);
    }
}

fn add_observers_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(game_state_handling::add_current_game_state_to_message_queue)
        .observe(game_state_handling::update_lobby_state)
        .observe(simulation::process_tick_sim)
        .observe(simulation::process_tick_sim_finished)
        .observe(start_lobby::check_if_lobby_should_start)
        .observe(start_lobby::start_lobby)
        .observe(process_messages::process_lobby_messages)
        .observe(lobby_cleanup::cleanup_entities);
}
