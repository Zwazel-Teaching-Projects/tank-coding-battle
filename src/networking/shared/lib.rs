use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::gameplay::lib::GameState;

use super::shared_data::first_contact::FirstContactData;

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message: NetworkMessageType,

    pub tick_sent: u64,
    pub tick_received: u64,
}

impl MessageContainer {
    pub fn new_sent(target: MessageTarget, message: NetworkMessageType, tick: u64) -> Self {
        let mut message = MessageContainer::new(target, message);
        message.with_sent(tick);

        message
    }

    pub fn new_received(target: MessageTarget, message: NetworkMessageType, tick: u64) -> Self {
        let mut message = MessageContainer::new(target, message);
        message.with_received(tick);

        message
    }

    pub fn new(target: MessageTarget, message: NetworkMessageType) -> Self {
        MessageContainer {
            target,
            message,
            tick_sent: 0,
            tick_received: 0,
        }
    }

    pub fn with_received(&mut self, tick: u64) -> &mut Self {
        self.tick_received = tick;
        self
    }

    pub fn with_sent(&mut self, tick: u64) -> &mut Self {
        self.tick_sent = tick;
        self
    }
}

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

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageTarget {
    #[default]
    Team,
    ServerOnly,
    All,
    Client, // TODO: we need to store the client ID here. what to use? Entity? SocketAddr? also, not send this out. because the receiver will just receive it, not send it.
}
