use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::networking::messages::message_data::game_state::GameState;

use super::player_handling::TankTransform;

/// The full game state stored in the lobby
/// This is the state that is sent to the spectators
#[derive(Debug, Reflect, Serialize, Deserialize, Clone, PartialEq, Component, Default)]
#[reflect(Component)]
#[serde(rename_all = "camelCase")]
pub struct LobbyGameState {
    pub tick: u64,
    pub client_states: HashMap<Entity, ClientState>,
}

impl From<LobbyGameState> for GameState {
    fn from(lobby_game_state: LobbyGameState) -> Self {
        GameState {
            tick: lobby_game_state.tick,
            client_states: lobby_game_state.client_states,
        }
    }
}

/// The personalized, individual, game state representing the client's view of the game
/// This is the state that is sent to the client
#[derive(Debug, Reflect, Serialize, Deserialize, Clone, PartialEq, Component, Default)]
#[reflect(Component)]
#[serde(rename_all = "camelCase")]
pub struct PersonalizedClientGameState {
    pub tick: u64,
    pub personal_state: ClientState,
    pub other_client_states: HashMap<Entity, ClientState>,
}

// PersonalizedClientGameState to GameState
impl From<PersonalizedClientGameState> for GameState {
    fn from(personalized_client_game_state: PersonalizedClientGameState) -> Self {
        // Insert my own state into the other client states
        let client_states: HashMap<Entity, ClientState> = personalized_client_game_state
            .other_client_states
            .into_iter()
            .chain(std::iter::once((
                personalized_client_game_state.personal_state.id,
                personalized_client_game_state.personal_state,
            )))
            .collect();

        GameState {
            tick: personalized_client_game_state.tick,
            client_states,
        }
    }
}

/// The state of a client
/// Can be personalized for each client, depending on the information the client knows about the other clients
#[derive(Debug, Reflect, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ClientState {
    /// This is the entity id of the client
    pub id: Entity,
    /// The position and rotation of the client.
    /// None if the client that receives this state does not know the position of the client.
    /// e.g. because the client has not spotted the other client yet.
    pub position: Option<TankTransform>,
}

impl ClientState {
    pub fn new(id: Entity) -> Self {
        ClientState { id, position: None }
    }
}

impl Default for ClientState {
    fn default() -> Self {
        ClientState {
            id: Entity::PLACEHOLDER,
            position: None,
        }
    }
}
