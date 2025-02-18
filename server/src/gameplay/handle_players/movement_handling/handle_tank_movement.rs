use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{player_handling::TankTransform, tank_types::TankType},
    networking::messages::{
        message_container::MoveTankCommandTrigger, message_data::tank_messages::MoveDirection,
    },
};

pub fn handle_tank_movement(
    trigger: Trigger<MoveTankCommandTrigger>,
    mut client: Query<(&mut TankTransform, &TankType)>,
    tank_config: TankConfigSystemParam,
) {
    let client_entity = trigger.entity();
    let (mut tank_transform, tank_type) = client
        .get_mut(client_entity)
        .expect("Failed to get tank transform");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");
    let direction = match trigger.direction {
        MoveDirection::Forward => 1.0,
        MoveDirection::Backward => -1.0,
    };

    let speed = tank_config.move_speed.min(trigger.distance);
    let distance = direction * speed;

    let move_direction = tank_transform.rotation * Vec3::new(0.0, 0.0, distance);
    tank_transform.position += move_direction;
}
