use bevy::prelude::*;

use crate::{config::MyConfig, gameplay::lib::TickIncreasedTrigger, main_state::MyMainState};

use super::{
    lib::{GameState, StartNextTickProcessing},
    system_sets::MyGameplaySet,
};

pub struct TickSystemsPlugin;

impl Plugin for TickSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<TickTimerResource>()
            .add_systems(OnEnter(MyMainState::Ready), init_tick_timer)
            .add_systems(
                Update,
                (
                    process_tick_timer
                        .in_set(MyGameplaySet::TickTimerProcessing)
                        .run_if(resource_exists::<TickTimerResource>),
                    increment_tick.in_set(MyGameplaySet::IncrementTick),
                ),
            );
    }
}

#[derive(Debug, Default, Reflect, Resource, Deref, DerefMut)]
#[reflect(Resource)]
struct TickTimerResource(Timer);

fn init_tick_timer(mut commands: Commands, config: Res<MyConfig>) {
    commands.insert_resource(TickTimerResource(Timer::from_seconds(
        1.0 / config.tick_rate,
        TimerMode::Repeating,
    )));
}

fn process_tick_timer(
    mut first_run: Local<bool>,
    mut event: EventWriter<StartNextTickProcessing>,
    mut tick_timer: ResMut<TickTimerResource>,
    time: Res<Time>,
) {
    if !*first_run {
        *first_run = true;
        event.send(StartNextTickProcessing);
    }

    if tick_timer.0.tick(time.delta()).just_finished() {
        event.send(StartNextTickProcessing);
    }
}

fn increment_tick(mut commands: Commands, mut state: ResMut<GameState>) {
    state.tick += 1;

    commands.trigger(TickIncreasedTrigger);
}
