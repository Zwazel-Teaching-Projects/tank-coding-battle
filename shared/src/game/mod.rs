use bevy::prelude::*;
use game_state::GameState;

pub mod game_state;

pub struct MySharedGamePlugin;

impl Plugin for MySharedGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameState>();
    }
}
