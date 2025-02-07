use bevy::prelude::*;
use proc_macros::{auto_trigger_message_received, generate_message_data_triggers};
use serde::{Deserialize, Serialize};

use crate::{
    game::game_state::GameState,
    networking::lobby_management::lobby_management::{
        LobbyManagementArgument, LobbyManagementSystemParam,
    },
};

use super::message_data::{
    first_contact::FirstContactData, simple_text_message::SimpleTextMessage,
};

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[auto_trigger_message_received(
    target = {
        #[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug, PartialEq)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        pub enum MessageTarget {
            #[default]
            #[get_targets(get_players_in_lobby_team)]
            Team,
            #[get_targets(get_empty)]
            ServerOnly,
            #[get_targets(get_players_in_lobby)]
            AllInLobby,
            #[get_targets(get_single_player)]
            Client,
        }
    },a
    message = {
        #[derive(Serialize, Deserialize, Reflect, Clone, Debug, PartialEq)]
        #[serde(tag = "message_type")]
        #[generate_message_data_triggers]
        pub enum NetworkMessageType {
            #[target(ServerOnly)]
            FirstContact(FirstContactData),
            GameState(GameState),
            #[target(Client, Team, AllInLobby)]
            SimpleTextMessage(SimpleTextMessage)
        }
    }
)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message: NetworkMessageType,

    #[serde(skip)]
    pub sender: Option<Entity>,

    // TODO: Do we need that? maybe just store the tick_received, maybe even store in the list of messages?
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
        NetworkMessageType::GameState(GameState::default())
    }
}
