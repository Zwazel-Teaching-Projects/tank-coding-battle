use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Reflect, Clone, PartialEq)]
#[serde(
    rename_all = "SCREAMING_SNAKE_CASE",
    tag = "error_type",
    content = "error_message"
)]
pub enum ErrorMessageTypes {
    InvalidTarget(String),
    LobbyManagementError(String),
    LobbyAlreadyRunning(String),
    TeamDoesNotExist(String),
    TeamFull(String),
}
