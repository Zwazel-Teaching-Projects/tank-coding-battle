use bevy::prelude::*;
use game_state::LobbyGameState;
use player_handling::{TankBodyMarker, TankTurretMarker};
use tank_types::TankType;

pub mod game_state;
pub mod player_handling;
pub mod tank_types;

pub struct MySharedGamePlugin;

impl Plugin for MySharedGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LobbyGameState>()
            .register_type::<TankBodyMarker>()
            .register_type::<TankTurretMarker>()
            .register_type::<TankType>();
    }
}
