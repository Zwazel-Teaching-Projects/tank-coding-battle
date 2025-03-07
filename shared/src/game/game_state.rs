use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::networking::messages::message_data::game_state::GameState;

use super::player_handling::PlayerState;

/// The full game state stored in the lobby
/// This is the state that is sent to the spectators
#[derive(Debug, Reflect, Serialize, Deserialize, Clone, PartialEq, Component, Default)]
#[reflect(Component)]
#[serde(rename_all = "camelCase")]
pub struct LobbyGameState {
    pub tick: u64,
    pub client_states: HashMap<Entity, ClientState>,
    pub projectiles: HashMap<Entity, ProjectileState>,
    pub flags: HashMap<Entity, FlagGameState>,
}

impl From<LobbyGameState> for GameState {
    fn from(lobby_game_state: LobbyGameState) -> Self {
        GameState {
            tick: lobby_game_state.tick,
            client_states: lobby_game_state
                .client_states
                .into_iter()
                .map(|(entity, client_state)| (entity, Some(client_state)))
                .collect(),
            projectile_states: lobby_game_state.projectiles,
            flag_states: lobby_game_state.flags,
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
    pub other_client_states: HashMap<Entity, Option<ClientState>>,
    pub projectiles: HashMap<Entity, ProjectileState>,
    pub flags: HashMap<Entity, FlagGameState>,
}

impl PersonalizedClientGameState {
    pub fn clear_non_persistent_data(&mut self) {
        self.personal_state.transform_body = None;
        for (_, state) in self.other_client_states.iter_mut() {
            state
                .as_mut()
                .map(|state| state.clear_non_persistent_information());
        }
        self.projectiles.clear();
    }
}

// PersonalizedClientGameState to GameState
impl From<PersonalizedClientGameState> for GameState {
    fn from(personalized_client_game_state: PersonalizedClientGameState) -> Self {
        // Insert my own state into the other client states
        let client_states = personalized_client_game_state
            .other_client_states
            .into_iter()
            .chain(std::iter::once((
                personalized_client_game_state.personal_state.id,
                Some(personalized_client_game_state.personal_state),
            )))
            .collect::<HashMap<Entity, Option<ClientState>>>();

        GameState {
            tick: personalized_client_game_state.tick,
            client_states,
            projectile_states: personalized_client_game_state.projectiles,
            flag_states: personalized_client_game_state.flags,
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
    /// The position and rotation of the clients body.
    /// None if the client that receives this state does not know the position of the client.
    /// e.g. because the client has not spotted the other client yet.
    pub transform_body: Option<Transform>,
    /// The position and rotation of the clients turret.
    /// Relative to the body.
    /// None if the client that receives this state does not know the position of the client.
    /// e.g. because the client has not spotted the other client yet.
    pub transform_turret: Option<Transform>,
    /// The state of the player (e.g. alive, dead)
    pub state: Option<PlayerState>,
    /// The time in ticks until the client can shoot again
    pub shoot_cooldown: u32,
    /// The current health of the client
    pub current_health: f32,
}

impl ClientState {
    pub fn new(id: Entity) -> Self {
        ClientState {
            id,
            transform_body: None,
            transform_turret: None,
            state: None,
            shoot_cooldown: 0,
            current_health: 100.0,
        }
    }

    /// Clears all information about the client that do not persist between ticks
    /// e.g. the transform
    /// While the tank type is not cleared, as once a client knows the tank type of another client, it will not forget it as it is a constant property
    pub fn clear_non_persistent_information(&mut self) {
        self.transform_body = None;
        self.transform_turret = None;
        self.state = None;
    }
}

impl Default for ClientState {
    fn default() -> Self {
        ClientState {
            id: Entity::PLACEHOLDER,
            transform_body: None,
            transform_turret: None,
            state: None,
            shoot_cooldown: 0,
            current_health: 100.0,
        }
    }
}

#[derive(Debug, Reflect, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ProjectileState {
    pub projectile_id: Entity,
    pub owner_id: Entity,
    pub transform: Transform,
}

impl ProjectileState {
    pub fn new(projectile_id: Entity, owner_id: Entity, transform: Transform) -> Self {
        ProjectileState {
            projectile_id,
            owner_id,
            transform,
        }
    }
}

#[derive(Debug, Reflect, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FlagGameState {
    pub flag_id: Entity,
    pub flag_number: usize,
    pub team: String,
    pub transform: Transform,
    pub collider_size: Vec3,
    pub state: super::flag::FlagState,
}

impl FlagGameState {
    pub fn new(
        flag_id: Entity,
        flag_number: usize,
        team: String,
        transform: Transform,
        collider_size: Vec3,
        state: super::flag::FlagState,
    ) -> Self {
        FlagGameState {
            flag_id,
            flag_number,
            team,
            transform,
            collider_size,
            state,
        }
    }
}
