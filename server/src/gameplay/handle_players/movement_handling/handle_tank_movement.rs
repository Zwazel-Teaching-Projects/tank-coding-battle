use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{collision_handling::components::WantedTransform, tank_types::TankType},
    networking::messages::message_container::MoveTankCommandTrigger,
};

pub fn handle_tank_movement(
    trigger: Trigger<MoveTankCommandTrigger>,
    mut tank: Query<(&mut WantedTransform, &TankType)>,
    tank_config: TankConfigSystemParam,
) {
    let client_entity = trigger.entity();
    let (mut tank_transform, tank_type) = tank
        .get_mut(client_entity)
        .expect("Failed to get tank transform");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");
    let distance = trigger
        .distance
        .clamp(-tank_config.move_speed, tank_config.move_speed);
    let move_direction = tank_transform.rotation * Vec3::new(0.0, 0.0, distance);
    let next_tank_position = tank_transform.translation + move_direction;

    tank_transform.translation = next_tank_position;
}
