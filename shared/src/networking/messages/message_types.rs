use bevy::prelude::*;
use proc_macros::generate_message_data_triggers;
use serde::{Deserialize, Serialize};

use crate::game::game_state::GameState;

use super::message_data::first_contact::FirstContactData;

#[derive(Serialize, Deserialize, Reflect, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "message_type")]
#[generate_message_data_triggers]
pub enum NetworkMessageType {
    FirstContact(FirstContactData),
    GameStateUpdate(GameState),
}

impl Default for NetworkMessageType {
    fn default() -> Self {
        NetworkMessageType::GameStateUpdate(GameState::default())
    }
}
