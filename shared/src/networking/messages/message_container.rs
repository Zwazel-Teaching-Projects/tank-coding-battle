use bevy::prelude::*;
use proc_macros::{auto_trigger_message_received, generate_message_data_triggers};
use serde::{Deserialize, Serialize};

use crate::game::game_state::GameState;

use super::{message_data::first_contact::FirstContactData, message_targets::MessageTarget};

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug)]
#[serde(rename_all = "camelCase")]
#[auto_trigger_message_received(
    #[derive(Serialize, Deserialize, Reflect, Clone, Debug)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "message_type")]
    #[generate_message_data_triggers]
    pub enum NetworkMessageType {
        FirstContact(FirstContactData),
        GameStateUpdate(GameState),
    }
)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message: NetworkMessageType,

    #[serde(skip)]
    pub sender: Option<Entity>,

    pub tick_sent: u64,
    pub tick_received: u64,
}

impl MessageContainer {
    pub fn new_sent(target: MessageTarget, message: NetworkMessageType, tick: u64) -> Self {
        let mut message = MessageContainer::new(target, message);
        message.with_sent(tick);

        message
    }

    pub fn new_received(
        target: MessageTarget,
        message: NetworkMessageType,
        tick: u64,
        sender: Entity,
    ) -> Self {
        let mut message = MessageContainer::new(target, message);
        message.with_received(tick, sender);

        message
    }

    pub fn new(target: MessageTarget, message: NetworkMessageType) -> Self {
        MessageContainer {
            target,
            message,

            sender: None,

            tick_sent: 0,
            tick_received: 0,
        }
    }

    pub fn with_received(&mut self, tick: u64, sender: Entity) -> &mut Self {
        self.tick_received = tick;
        self.sender = Some(sender);
        self
    }

    pub fn with_sent(&mut self, tick: u64) -> &mut Self {
        self.tick_sent = tick;
        self
    }
}

impl Default for NetworkMessageType {
    fn default() -> Self {
        NetworkMessageType::GameStateUpdate(GameState::default())
    }
}
