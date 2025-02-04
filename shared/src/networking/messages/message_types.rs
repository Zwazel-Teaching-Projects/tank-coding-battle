use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::game_state::GameState;

use super::message_data::first_contact::FirstContactData;

#[derive(Serialize, Deserialize, Reflect, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "message_type")]
pub enum NetworkMessageType {
    FirstContact(FirstContactData),
    GameStateUpdate(GameState),
}

impl Default for NetworkMessageType {
    fn default() -> Self {
        NetworkMessageType::GameStateUpdate(GameState::default())
    }
}
