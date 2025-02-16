use bevy::prelude::*;
use game_state::LobbyGameState;
use player_handling::TankTransform;

pub mod game_state;
pub mod player_handling;

pub struct MySharedGamePlugin;

impl Plugin for MySharedGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LobbyGameState>()
            .register_type::<TankTransform>();
    }
}
