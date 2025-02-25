use bevy::{color::palettes::css::WHITE, prelude::*};
use shared::game::collision_handling::components::Collider;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyColliderGizmos {}

pub fn visualize_colliders(
    mut my_gizmos: Gizmos<MyColliderGizmos>,
    query: Query<(&Transform, &Collider)>,
) {
    for (transform, collider) in query.iter() {
        let position = transform.translation;
        let rotation = transform.rotation;

        my_gizmos.primitive_3d(
            &Cuboid {
                half_size: collider.half_size,
            },
            Isometry3d::new(position, rotation),
            WHITE,
        );
    }
}
