use bevy::prelude::*;
use shared::{
    game::game_state::LobbyGameState,
    networking::lobby_management::{LobbyState, MyLobby},
};

use crate::gameplay::triggers::CollectAndTriggerMessagesTrigger;

use super::{
    system_sets::MyGameplaySet, triggers::StartNextTickProcessingTrigger, LobbyWaitTicksUntilStart,
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
    mut lobbies: Query<(Entity, &mut MyLobby, Option<&mut LobbyWaitTicksUntilStart>)>,
    time: Res<Time>,
) {
    for (entity, mut lobby, lobby_wait_tick_until_start) in lobbies.iter_mut() {
        if LobbyState::InProgress == lobby.state {
            if lobby.tick_timer.tick(time.delta()).just_finished() {
                if let Some(mut lobby_wait_tick_until_start) = lobby_wait_tick_until_start {
                    if lobby_wait_tick_until_start.0 > 0 {
                        lobby_wait_tick_until_start.0 -= 1;
                    } else {
                        commands.entity(entity).remove::<LobbyWaitTicksUntilStart>();
                    }
                } else {
                    commands.trigger_targets(StartNextTickProcessingTrigger, entity);
                }
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
    mut lobbies: Query<&mut LobbyGameState>,
) {
    let lobby_entity = trigger.entity();
    let mut game_state = lobbies.get_mut(lobby_entity).unwrap();
    game_state.tick += 1;

    commands.trigger_targets(CollectAndTriggerMessagesTrigger, lobby_entity);
}
