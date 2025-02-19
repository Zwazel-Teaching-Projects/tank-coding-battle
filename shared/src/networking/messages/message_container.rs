use bevy::prelude::*;
use proc_macros::{auto_trigger_message_received, generate_message_data_triggers};
use serde::{Deserialize, Serialize};

use crate::networking::{
    lobby_management::lobby_management::{LobbyManagementArgument, LobbyManagementSystemParam},
    messages::message_queue::OutMessageQueue,
};

use super::message_data::{
    first_contact::FirstContactData,
    game_starts::GameStarts,
    game_state::GameState,
    message_error_types::ErrorMessageTypes,
    start_game_config::StartGameConfig,
    tank_messages::{
        move_tank::MoveTankCommand, rotate_tank_body::RotateTankBodyCommand,
        rotate_tank_turret::RotateTankTurretCommand,
    },
    text_data::TextDataWrapper,
};

#[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[auto_trigger_message_received(
    target = {
        #[derive(Serialize, Deserialize, Default, Reflect, Clone, Debug, PartialEq)]
        #[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "type", content = "clientId")]
        pub enum MessageTarget {
            #[default]
            #[get_targets(targets_get_players_in_lobby_team)]
            // To everyone in the same team in the same lobby
            Team,
            #[get_targets(targets_get_empty)]
            // To the server directly, no lobby or client. Used for first contact
            ServerOnly,
            #[get_targets(targets_get_players_in_lobby)]
            // To everyone in the same lobby
            AllInLobby,
            #[get_targets(targets_get_single_player)]
            // To a single player, excluding the the sender itself
            Client(Entity),
            #[get_targets(targets_get_lobby_directly)]
            // To the lobby itself (for example to start the game)
            ToLobbyDirectly,
            // To the sender itself (used for commands, e.g. tank movement)
            #[get_targets(targets_get_self)]
            ToSelf,
        }
    },
    message = {
        #[derive(Serialize, Deserialize, Reflect, Clone, Debug, PartialEq)]
        #[serde(tag = "message_type")]
        #[generate_message_data_triggers]
        pub enum NetworkMessageType {
            /// The first message sent by the client to the server
            /// Used to determine the client type and the lobby to join or create and the team to join and other initial information
            #[target(ServerOnly)]
            FirstContact(FirstContactData),
            /// The current game state, sent each tick to the clients
            /// Each client could receive a different state, depending on their view of the game
            /// Can not be sent by a client, only by the server
            GameState(GameState),
            /// A simple Text Message
            /// Can be sent to a single client, everyone in the team or everyone in the lobby
            /// The server does not do anything with this message, it only forwards it to the specified targets
            /// We need to rename it, because we don't want it to be serialized as "TextDataWrapper"
            #[serde(rename = "SimpleTextMessage")]
            #[target(Client, Team, AllInLobby)]
            #[behaviour(Forward)]
            SimpleTextMessage(TextDataWrapper),
            /// An error message, sent to the client that caused the error.
            /// Can not be sent by a client, only by the server
            MessageError(ErrorMessageTypes),
            /// Sent to the client when the game starts, contains the game configuration
            /// e.g. the map, the teams, the players, etc.
            /// Can not be sent by a client, only by the server
            #[serde(rename = "GameConfig")]
            GameStarts(GameStarts),
            /// Sent to the lobby by a client to start the game
            /// Can only be sent to the lobby directly
            #[target(ToLobbyDirectly)]
            StartGame(StartGameConfig),
            /// Sent to the client when they successfully joined a lobby
            /// Can not be sent by a client, only by the server
            /// We need to rename it, because we don't want it to be serialized as "TextDataWrapper"
            #[serde(rename = "SuccessfullyJoinedLobby")]
            SuccessFullyJoinedLobby(TextDataWrapper),
            /// Sent from the client to the server to move the tank
            /// Will only be sent by a client
            /// Can only be sent to itself on the server
            #[target(ToSelf)]
            MoveTankCommand(MoveTankCommand),
            #[target(ToSelf)]
            RotateTankBodyCommand(RotateTankBodyCommand),
            #[target(ToSelf)]
            RotateTankTurretCommand(RotateTankTurretCommand),
        }
    }
)]
pub struct MessageContainer {
    pub target: MessageTarget,
    pub message: NetworkMessageType,

    pub sender: Option<Entity>,

    /// The tick when the message was sent
    pub tick_sent: u64,
    /// The tick when the message was received
    pub tick_received: u64,
    /// The tick when the message should be processed at on the server
    #[serde(skip)]
    pub tick_to_be_processed_at: u64,
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
            tick_to_be_processed_at: 0,
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
