use bevy::prelude::*;
use shared::{
    game::game_state::LobbyGameState,
    networking::lobby_management::{LobbyState, MyLobby},
};

use super::{
    system_sets::MyGameplaySet,
    triggers::{
        StartNextSimulationStepTrigger, StartNextTickProcessingTrigger,
        UpdateClientGameStatesTrigger,
    },
};

pub struct TickSystemsPlugin;

impl Plugin for TickSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (process_tick_timer.in_set(MyGameplaySet::TickTimerProcessing),),
        )
        .add_observer(add_trigger_to_lobby);
    }
}

fn process_tick_timer(
    mut commands: Commands,
    mut lobbies: Query<(Entity, &mut MyLobby)>,
    time: Res<Time>,
) {
    for (entity, mut lobby) in lobbies.iter_mut() {
        if LobbyState::InProgress == lobby.state {
            if lobby.tick_timer.tick(time.delta()).just_finished() {
                commands.trigger_targets(StartNextTickProcessingTrigger, entity);
            }
        }
    }
}

fn add_trigger_to_lobby(trigger: Trigger<OnAdd, MyLobby>, mut commands: Commands) {
    commands.entity(trigger.entity()).observe(increment_tick);
}

fn increment_tick(
    trigger: Trigger<StartNextTickProcessingTrigger>,
    mut commands: Commands,
    mut lobbies: Query<(&MyLobby, &mut LobbyGameState)>,
) {
    let lobby_entity = trigger.entity();
    let (lobby, mut game_state) = lobbies.get_mut(lobby_entity).unwrap();
    game_state.tick += 1;

    commands.trigger_targets(
        UpdateClientGameStatesTrigger,
        lobby
            .players
            .iter()
            .map(|(_, entity, _)| *entity)
            .collect::<Vec<_>>(),
    );

    commands.trigger_targets(StartNextSimulationStepTrigger, lobby_entity);
}
