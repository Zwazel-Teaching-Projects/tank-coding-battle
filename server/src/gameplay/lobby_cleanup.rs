use bevy::prelude::*;
use shared::networking::lobby_management::{InLobby, MyLobby};

use super::triggers::StartNextTickProcessingTrigger;

/// Entities marked with this get cleaned up on the next tick (Only if they are in a lobby)
#[derive(Debug, Reflect, Default, Component)]
#[reflect(Component)]
pub struct CleanupNextTick;

pub fn cleanup_lobby(
    trigger: Trigger<OnRemove, MyLobby>,
    query: Query<(Entity, &InLobby)>,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();
    for (entity, in_lobby) in query.iter() {
        if in_lobby.0 == lobby_entity {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn cleanup_entities(
    trigger: Trigger<StartNextTickProcessingTrigger>,
    query: Query<(Entity, &InLobby), With<CleanupNextTick>>,
    mut commands: Commands,
) {
    let lobby = trigger.entity();
    for (entity, in_lobby) in query.iter() {
        if in_lobby.0 == lobby {
            commands.entity(entity).despawn_recursive();
        }
    }
}
