use bevy::prelude::*;
use std::io::Write;

use crate::{
    gameplay::lib::GameState,
    networking::{
        lib::MyConnectedClients,
        shared::lib::{MessageContainer, NetworkMessageType},
    },
};

pub fn sending_messages(
    game_state: Res<GameState>,
    mut connected_clients: ResMut<MyConnectedClients>,
) {
    let message = MessageContainer {
        message_type: NetworkMessageType::GameStateUpdate,
        message_data: serde_json::to_value(&*game_state).unwrap(),
        target: Default::default(),
    };
    let message = serde_json::to_vec(&message).expect("Failed to serialize message");
    let length = message.len() as u32;
    info!(
        "Sending message after serialize with length: {}:\n{:?}",
        length, message
    );
    let length = length.to_le_bytes();

    for (_client_id, stream) in connected_clients.streams.iter_mut() {
        let _ = stream.write_all(&length).expect("Failed to send length");
        let _ = stream.write_all(&message).expect("Failed to send message");
    }
}
