use bevy::prelude::*;
use shared::{
    game::{collision_handling::components::CollisionLayer, flag::FlagState},
    networking::lobby_management::lobby_management::LobbyManagementSystemParam,
};

use crate::gameplay::capture_the_flag::triggers::FlagGotDroppedTrigger;

#[derive(Debug, Reflect, Event)]
pub struct ClientDiedTrigger;

pub fn client_died(
    trigger: Trigger<ClientDiedTrigger>,
    lobby: LobbyManagementSystemParam,
    mut player: Query<&mut CollisionLayer>,
    flags: Query<&FlagState>,
    mut commands: Commands,
) {
    let player_entity = trigger.entity();
    let (lobby_entity, lobby, _) = lobby
        .get_lobby_of_player(player_entity)
        .expect("Lobby not found");

    let mut player_collision_layer = player.get_mut(player_entity).expect("Player not found");
    *player_collision_layer = CollisionLayer::none();

    // Check all flags in the lobby, if the state is Carried and the carrier is the player, drop the flag
    for flag in lobby.flags.iter() {
        let flag_state = flags.get(*flag).expect("Flag not found");
        if let FlagState::Carried(carrier_entity) = *flag_state {
            if carrier_entity == player_entity {
                commands.trigger_targets(
                    FlagGotDroppedTrigger {
                        flag: *flag,
                        carrier: carrier_entity,
                    },
                    lobby_entity,
                );
            }

            return;
        }
    }
}
