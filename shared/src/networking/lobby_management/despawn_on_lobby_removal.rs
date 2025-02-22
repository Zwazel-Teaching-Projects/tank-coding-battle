use bevy::prelude::*;

use crate::game::projectile_handling::ProjectileMarker;

use super::{InLobby, MyLobbies, MyLobby};

pub fn add_observers_on_lobby_join(trigger: Trigger<OnAdd, InLobby>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(lobby_despawn)
        .observe(remove_projectile_on_projectile_despawn);
}

fn lobby_despawn(trigger: Trigger<OnRemove, MyLobby>, mut commands: Commands) {
    commands.entity(trigger.entity()).despawn_recursive();
}

pub fn update_my_lobbies_on_lobby_despawn(
    trigger: Trigger<OnRemove, MyLobby>,
    mut my_lobbies: ResMut<MyLobbies>,
) {
    my_lobbies.remove_lobby(trigger.entity());
}

pub fn remove_projectile_on_projectile_despawn(
    trigger: Trigger<OnRemove, ProjectileMarker>,
    mut lobby: Query<&mut MyLobby>,
    projectiles: Query<&InLobby>,
) {
    let projectile_entity = trigger.entity();
    if let Ok(in_lobby) = projectiles.get(projectile_entity) {
        if let Ok(mut lobby) = lobby.get_mut(in_lobby.0) {
            lobby.remove_projectile(projectile_entity);
        }
    }
}
