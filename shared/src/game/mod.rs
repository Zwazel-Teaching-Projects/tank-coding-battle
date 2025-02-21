use bevy::prelude::*;
use game_state::{ClientState, LobbyGameState, PersonalizedClientGameState, ProjectileState};
use player_handling::{ShootCooldown, TankBodyMarker, TankTurretMarker};
use projectile_handling::ProjectileMarker;
use tank_types::TankType;

pub mod game_state;
pub mod player_handling;
pub mod projectile_handling;
pub mod tank_types;

pub struct MySharedGamePlugin;

impl Plugin for MySharedGamePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LobbyGameState>()
            .register_type::<PersonalizedClientGameState>()
            .register_type::<ClientState>()
            .register_type::<ProjectileState>()
            .register_type::<TankBodyMarker>()
            .register_type::<TankTurretMarker>()
            .register_type::<ShootCooldown>()
            .register_type::<TankType>()
            .register_type::<ProjectileMarker>();
    }
}
