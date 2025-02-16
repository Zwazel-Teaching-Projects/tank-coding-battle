use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct FirstContactData {
    pub bot_name: String,
    pub lobby_name: String,
    pub map_name: Option<String>,
    pub team_name: Option<String>,
    pub client_type: ClientType,
    pub bot_assigned_spawn_point: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq, Component, Default)]
#[reflect(Component)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientType {
    Player,
    #[default]
    Spectator,
    #[serde(skip)]
    Dummy,
}
