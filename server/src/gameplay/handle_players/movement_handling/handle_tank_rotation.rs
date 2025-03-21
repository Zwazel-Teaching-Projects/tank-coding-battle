use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{collision_handling::components::WantedTransform, player_handling::TankBodyMarker, tank_types::TankType},
    networking::messages::message_container::RotateTankBodyCommandTrigger,
};

pub fn handle_tank_body_rotation(
    trigger: Trigger<RotateTankBodyCommandTrigger>,
    mut body_transform: Query<(&mut WantedTransform, &TankType), With<TankBodyMarker>>,
    tank_config: TankConfigSystemParam,
) {
    let client_entity = trigger.entity();
    let (mut tank_transform, tank_type) = body_transform
        .get_mut(client_entity)
        .expect("Failed to get tank transform");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");

    let rotation = trigger.angle.clamp(
        -tank_config.body_rotation_speed,
        tank_config.body_rotation_speed,
    );

    tank_transform.rotation *= Quat::from_rotation_y(rotation);
}
