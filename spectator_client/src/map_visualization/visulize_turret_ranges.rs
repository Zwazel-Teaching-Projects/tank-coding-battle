use bevy::{color::palettes::css::RED, prelude::*};
use shared::game::player_handling::TankTurretMarker;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyTurretRangeGizmos {}

pub fn draw_turret_ranges(
    mut my_gizmos: Gizmos<MyTurretRangeGizmos>,
    turrets: Query<&GlobalTransform, With<TankTurretMarker>>,
) {
    const RANGE: f32 = 10.0;

    for turret in turrets.iter() {
        let position = turret.translation();
        let rotation = turret.rotation();

        // Draw a line to the front
        let end = position + rotation.mul_vec3(Vec3::new(0.0, 0.0, RANGE));
        my_gizmos.line(position, end, RED);
    }
}
