use bevy::prelude::*;
use shared::{
    game::game_state::LobbyGameState,
    networking::{
        lobby_management::{InLobby, InTeam, MyLobby},
        messages::{
            message_container::{MessageContainer, MessageTarget, NetworkMessageType},
            message_data::team_scored::TeamScoredData,
            message_queue::OutMessageQueue,
        },
    },
};

use crate::networking::handle_clients::lib::MyNetworkClient;

use super::triggers::TeamScoredTrigger;

pub fn handle_scoring(
    trigger: Trigger<TeamScoredTrigger>,
    players: Query<(&InTeam, &InLobby)>,
    mut lobby: Query<
        (&mut LobbyGameState, &mut OutMessageQueue),
        (With<MyLobby>, Without<MyNetworkClient>),
    >,
) {
    let _lobby_entity = trigger.entity();
    let scorer = trigger.scorer;

    if let Ok((team, in_lobby)) = players.get(scorer) {
        let team = &team.0;
        let in_lobby = in_lobby.0;

        if let Ok((mut lobby_state, mut lobby_message_queue)) = lobby.get_mut(in_lobby) {
            lobby_state
                .score
                .entry(team.clone())
                .and_modify(|score| *score += 1);

            lobby_message_queue.push_back(MessageContainer::new(
                MessageTarget::ToEveryone,
                NetworkMessageType::TeamScored(TeamScoredData {
                    score: lobby_state.score.get(team).unwrap().clone(),
                    team: team.clone(),
                    scorer,
                }),
            ));
        } else {
            warn!("Failed to get lobby state and message queue");
        }
    } else {
        warn!("Failed to get team and lobby");
    }
}
