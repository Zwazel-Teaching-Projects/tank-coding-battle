use bevy::prelude::*;
use shared::{
    asset_handling::config::TankConfigSystemParam,
    game::{
        player_handling::{TankBodyMarker, TankTurretMarker},
        tank_types::TankType,
    },
};

pub fn insert_turret(
    trigger: Trigger<OnAdd, TankBodyMarker>,
    mut commands: Commands,
    tank_config: TankConfigSystemParam,
    mut tank: Query<(&mut TankBodyMarker, &TankType)>,
) {
    let new_tank = trigger.entity();
    let (mut tank_body, tank_type) = tank.get_mut(new_tank).expect("Failed to get tank type");

    let tank_config = tank_config
        .get_tank_type_config(tank_type)
        .expect("Failed to get tank config");

    let turret = commands
        .spawn((
            Name::new("Turret"),
            TankTurretMarker { body: new_tank },
            Transform::from_translation(Vec3::new(0.0, tank_config.size.y, 0.0)),
        ))
        .id();
    tank_body.turret = Some(turret);

    commands.entity(new_tank).add_child(turret);
}
