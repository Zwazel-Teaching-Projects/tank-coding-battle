use bevy::prelude::*;
use gameplay_state::MyGameplayState;
use lib::{GameState, TickIncreasedEvent};
use system_sets::MyGameplaySet;

pub mod gameplay_state;
pub mod lib;
pub mod system_sets;
mod tick_systems;

pub struct MyGameplayPlugin;

impl Plugin for MyGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                MyGameplaySet::CollectBotInput,
                MyGameplaySet::ApplyBotInput,
                MyGameplaySet::UpdateGameState,
                MyGameplaySet::TickIncrease,
            )
                .chain()
                .run_if(in_state(MyGameplayState::Running)),
        )
        .add_sub_state::<MyGameplayState>()
        .register_type::<GameState>()
        .init_resource::<GameState>()
        .add_event::<TickIncreasedEvent>();
    }
}
