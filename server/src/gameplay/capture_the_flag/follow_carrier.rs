use bevy::prelude::*;
use shared::{
    game::{
        collision_handling::components::WantedTransform,
        flag::{FlagMarker, FlagState},
        player_handling::TankBodyMarker,
    },
    networking::lobby_management::MyLobby,
};

use crate::gameplay::{
    capture_the_flag::triggers::FlagGotDroppedTrigger,
    triggers::{MoveFlagsSimulationStepTrigger, UpdateLobbyGameStateTrigger},
};

pub fn follow_carrier(
    trigger: Trigger<MoveFlagsSimulationStepTrigger>,
    my_lobby: Query<&MyLobby>,
    tanks: Query<&Transform, (With<TankBodyMarker>, Without<FlagMarker>)>,
    mut flags: Query<
        (&mut Transform, &mut WantedTransform, &FlagState),
        (With<FlagMarker>, Without<TankBodyMarker>),
    >,
    mut commands: Commands,
) {
    let lobby_entity = trigger.entity();

    let lobby = my_lobby.get(lobby_entity).expect("Lobby not found");
    for flag in lobby.flags.iter() {
        let (mut transform, mut wanted_transform, flag_state) =
            flags.get_mut(*flag).expect("Flag not found");

        match flag_state {
            FlagState::Carried(carrier_entity) => {
                if let Ok(carrier_transform) = tanks.get(*carrier_entity) {
                    transform.translation = carrier_transform.translation;
                    transform.rotation = carrier_transform.rotation;
                    wanted_transform.0 = transform.clone();
                } else {
                    warn!("Carrier not found, dropping flag");
                    commands.trigger_targets(
                        FlagGotDroppedTrigger {
                            flag: *flag,
                            carrier: *carrier_entity,
                        },
                        lobby_entity,
                    );
                }
            }
            FlagState::InBase => {
                // Do nothing
            }
            FlagState::Dropped => {
                // Do nothing
            }
        }
    }

    commands.trigger_targets(UpdateLobbyGameStateTrigger, lobby_entity);
}
