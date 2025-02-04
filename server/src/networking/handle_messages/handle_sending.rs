use bevy::prelude::*;
use shared::{
    game::game_state::GameState,
    networking::messages::{
        message_container::MessageContainer, message_targets::MessageTarget,
        message_types::NetworkMessageType,
    },
};
use std::io::Write;

use crate::networking::handle_clients::lib::MyNetworkClient;

pub fn sending_messages(
    game_state: Res<GameState>,
    mut connected_clients: Query<&mut MyNetworkClient>,
) {
    let message = MessageContainer::new_sent(
        MessageTarget::Client,
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
