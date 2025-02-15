use bevy::prelude::*;
use shared::networking::networking_state::MyNetworkingState;
use start_game::{spawn_start_game_button, StartGameButton};

pub mod interaction_system;
pub mod start_game;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartGameButton>()
            .add_systems(OnEnter(MyNetworkingState::Running), spawn_start_game_button)
            .add_systems(Update, (interaction_system::interaction_system,));
    }
}

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);
