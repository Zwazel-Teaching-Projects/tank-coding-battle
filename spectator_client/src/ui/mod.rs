use bevy::prelude::*;
use start_game::{spawn_start_game_button, StartGameButton};

use crate::{game_state::MyGameState, networking::MyNetworkStream};

pub mod interaction_system;
pub mod run_conditions;
pub mod start_game;

pub struct MyUiPlugin;

impl Plugin for MyUiPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<StartGameButton>()
            .add_systems(OnEnter(MyGameState::SettingUp), spawn_start_game_button)
            .add_systems(
                Update,
                (interaction_system::interaction_system,)
                    .run_if(not(run_conditions::cursor_is_locked)),
            )
            .add_observer(add_observers_to_client);
    }
}

fn add_observers_to_client(trigger: Trigger<OnAdd, MyNetworkStream>, mut commands: Commands) {
    commands
        .entity(trigger.entity())
        .observe(start_game::start_game_error_handling);
}

pub const NORMAL_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);
