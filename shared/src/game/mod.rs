use bevy::prelude::*;
use collision_handling::MyCollisionHandlingPlugin;
use game_state::{ClientState, LobbyGameState, PersonalizedClientGameState, ProjectileState};
use player_handling::{
    Health, PlayerState, RespawnTimer, ShootCooldown, TankBodyMarker, TankTurretMarker,
};
use tank_types::TankType;

pub mod collision_handling;
pub mod common_components;
pub mod common_systems;
pub mod flag;
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
            .register_type::<Health>()
            .register_type::<RespawnTimer>()
            .register_type::<TankBodyMarker>()
            .register_type::<TankTurretMarker>()
            .register_type::<ShootCooldown>()
            .register_type::<TankType>()
            .register_type::<PlayerState>()
            .register_type::<projectile_handling::ProjectileMarker>()
            .register_type::<common_components::DespawnTimer>()
            .register_type::<common_components::TickBasedDespawnTimer>()
            .register_type::<flag::FlagMarker>()
            .register_type::<flag::FlagState>()
            .register_type::<flag::FlagBaseMarker>()
            .add_plugins((MyCollisionHandlingPlugin,))
            .add_systems(
                Update,
                common_systems::handle_despawn_timer
                    .run_if(any_with_component::<common_components::DespawnTimer>),
            )
            .add_observer(projectile_handling::setup_projectile)
            .add_observer(player_handling::setup_tank_body);
    }
}
