use bevy::prelude::*;
use std::io::Write;

use crate::{
    gameplay::lib::GameState,
    networking::{
        lib::MyConnectedClients,
        shared::lib::{MessageContainer, MessageTarget, NetworkMessageType},
    },
};

pub fn sending_messages(
    game_state: Res<GameState>,
    mut connected_clients: ResMut<MyConnectedClients>,
) {
    let message = MessageContainer {
        message: NetworkMessageType::GameStateUpdate(game_state.clone()),
        target: MessageTarget::Team,
    };
    let message = serde_json::to_vec(&message).expect("Failed to serialize message");
    let length = (message.len() as u32).to_le_bytes();

    for (_client_id, stream) in connected_clients.streams.iter_mut() {
        let _ = stream.write_all(&length).expect("Failed to send length");
        let _ = stream.write_all(&message).expect("Failed to send message");
    }
}
