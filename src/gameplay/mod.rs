use bevy::prelude::*;
use gameplay_state::MyGameplayState;
use handle_players::HandlePlayersPlugin;
use lib::{GameState, StartNextTickProcessing};
use system_sets::MyGameplaySet;
use tick_systems::TickSystemsPlugin;

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
                (MyGameplaySet::IncrementTick, MyGameplaySet::RunSimulation)
                    .chain()
                    .run_if(on_event::<StartNextTickProcessing>),
            )
                .chain()
                .run_if(in_state(MyGameplayState::Running)),
        )
        .add_sub_state::<MyGameplayState>()
        .register_type::<GameState>()
        .init_resource::<GameState>()
        .add_event::<StartNextTickProcessing>()
        .add_plugins((TickSystemsPlugin, HandlePlayersPlugin));

        #[cfg(debug_assertions)]
        app.add_systems(
            Update,
            bevy::dev_tools::states::log_transitions::<MyGameplayState>,
        );
    }
}
