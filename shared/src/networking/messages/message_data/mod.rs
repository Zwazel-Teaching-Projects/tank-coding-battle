use bevy::prelude::*;

pub mod entity_data;
pub mod first_contact;
pub mod flag_event_data;
pub mod game_starts;
pub mod game_state;
pub mod message_error_types;
pub mod start_game_config;
pub mod tank_messages;
pub mod team_scored;
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
            .register_type::<game_state::GameState>()
            .register_type::<start_game_config::StartGameConfig>()
            .register_type::<flag_event_data::FlagEventDataWrapper>()
            .register_type::<flag_event_data::FlagSimpleEventDataWrapper>()
            .register_type::<entity_data::EntityDataWrapper>()
            .register_type::<team_scored::TeamScoredData>()
            .add_plugins((tank_messages::MyTankMessagesPlugin,));
    }
}
