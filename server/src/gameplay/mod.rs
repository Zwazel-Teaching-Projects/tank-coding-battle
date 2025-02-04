use bevy::prelude::*;
use gameplay_state::MyGameplayState;
use handle_players::HandlePlayersPlugin;
use lib::StartNextTickProcessing;
use shared::{
    game::game_state::GameState,
    networking::messages::{
        message_container::MessageContainer, message_targets::MessageTarget,
        message_types::NetworkMessageType,
    },
};
use system_sets::MyGameplaySet;
use tick_systems::TickSystemsPlugin;

use crate::networking::handle_messages::message_queue::OutgoingMessageQueue;

pub mod gameplay_state;
mod handle_players;
pub mod lib;
pub mod system_sets;
mod tick_systems;

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
                    .chain()
                    .run_if(on_event::<StartNextTickProcessing>),
            )
                .chain()
                .run_if(in_state(MyGameplayState::Running)),
        )
        .add_sub_state::<MyGameplayState>()
        .add_event::<StartNextTickProcessing>()
        .add_plugins((TickSystemsPlugin, HandlePlayersPlugin))
        .add_systems(
            Update,
            add_current_game_state_to_message_queue.in_set(MyGameplaySet::SimulationStepDone),
        );

        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<MyGameplayState>,
        );
    }
}

fn add_current_game_state_to_message_queue(
    mut message_queue: ResMut<OutgoingMessageQueue>,
    game_state: Res<GameState>,
) {
    message_queue.push_back(MessageContainer::new(
        MessageTarget::All,
        NetworkMessageType::GameStateUpdate(game_state.clone()),
    ));
}
