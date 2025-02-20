use bevy::prelude::*;

pub mod move_tank;
pub mod rotate_tank_body;
pub mod rotate_tank_turret;

pub struct MyTankMessagesPlugin;

impl Plugin for MyTankMessagesPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<move_tank::MoveTankCommand>()
            .register_type::<rotate_tank_body::RotateTankBodyCommand>()
            .register_type::<rotate_tank_turret::RotateTankTurretCommand>();
    }
}
