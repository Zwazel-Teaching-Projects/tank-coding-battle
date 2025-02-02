use bevy::prelude::*;
use std::io::Write;

use crate::{
    gameplay::lib::GameState,
    networking::{
        handle_clients::lib::MyNetworkClient,
        shared::lib::{MessageContainer, MessageTarget, NetworkMessageType},
    },
};

pub fn sending_messages(
    game_state: Res<GameState>,
    mut connected_clients: Query<&mut MyNetworkClient>,
) {
    let message = MessageContainer::new_sent(
        MessageTarget::Team,
        NetworkMessageType::GameStateUpdate(game_state.clone()),
        game_state.tick,
    );
    let message = serde_json::to_vec(&message).expect("Failed to serialize message");
    let length = (message.len() as u32).to_le_bytes();

    for mut network_client in connected_clients.iter_mut() {
        let stream = &mut network_client.stream;
        let _ = stream.write_all(&length).expect("Failed to send length");
        let _ = stream.write_all(&message).expect("Failed to send message");
    }
}
