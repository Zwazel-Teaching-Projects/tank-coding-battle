use bevy::{prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::game::game_state::ClientState;

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    pub tick: u64,
    pub client_states: HashMap<Entity, ClientState>,
}
