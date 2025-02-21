use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        player_handling::{TankBodyMarker, TankTurretMarker},
        tank_types::TankType,
    },
    networking::messages::message_container::RotateTankTurretCommandTrigger,
};

pub fn handle_tank_turret_rotation(
    trigger: Trigger<RotateTankTurretCommandTrigger>,
    body: Query<(&TankType, &TankBodyMarker), Without<TankTurretMarker>>,
    mut turret_transform: Query<&mut Transform, With<TankTurretMarker>>,
    tank_config: TankConfigSystemParam,
) {
    let client_entity = trigger.entity();
    let (tank_type, tank_body) = body
        .get(client_entity)
        .expect("Failed to get tank transform");
    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");

    // Calculate the delta rotations for yaw and pitch.
    let yaw_delta = trigger.yaw_angle.clamp(
        -tank_config.turret_yaw_rotation_speed,
        tank_config.turret_yaw_rotation_speed,
    );
    let pitch_delta = trigger.pitch_angle.clamp(
        -tank_config.turret_pitch_rotation_speed,
        tank_config.turret_pitch_rotation_speed,
    );

    // Retrieve the turret entity and its transform.
    let turret_entity = tank_body.turret.expect("Failed to get turret entity");
    let mut turret_transform = turret_transform
        .get_mut(turret_entity)
        .expect("Failed to get turret");

    // Extract only yaw and pitch from the current rotation; discard any roll.
    let (current_yaw, current_pitch, _current_roll) =
        turret_transform.rotation.to_euler(EulerRot::YXZ);
    let new_yaw = current_yaw + yaw_delta;
    let new_pitch = current_pitch + pitch_delta;

    // Clamp pitch to prevent the turret from rotating upside down.
    let max_pitch = tank_config.turret_max_pitch;
    let min_pitch = tank_config.turret_min_pitch;

    // Clamp the pitch to the maximum and minimum pitch angles.
    let new_pitch = new_pitch.clamp(min_pitch, max_pitch);

    // Construct a new rotation with roll forcibly set to zero.
    turret_transform.rotation = Quat::from_euler(EulerRot::YXZ, new_yaw, new_pitch, 0.0);
}
