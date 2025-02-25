use bevy::prelude::*;
use shared::networking::lobby_management::{InLobby, MyLobby};

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
