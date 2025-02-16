use bevy::prelude::*;

pub mod first_contact;
pub mod game_starts;
pub mod game_state;
pub mod message_error_types;
pub mod start_game_config;
pub mod text_data;

pub struct MySharedMessageDataPlugin;

impl Plugin for MySharedMessageDataPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<first_contact::FirstContactData>()
            .register_type::<first_contact::ClientType>()
            .register_type::<message_error_types::ErrorMessageTypes>()
            .register_type::<game_starts::GameStarts>()
            .register_type::<game_starts::ConnectedClientConfig>()
            .register_type::<text_data::TextDataWrapper>()
            .register_type::<game_state::GameState>();
    }
}
