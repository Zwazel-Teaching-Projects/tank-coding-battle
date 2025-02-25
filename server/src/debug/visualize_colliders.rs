use bevy::{color::palettes::css::WHITE, prelude::*};
use shared::game::collision_handling::{components::Collider, structs::Obb3d};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyColliderGizmos {}

pub fn visualize_colliders(
    mut my_gizmos: Gizmos<MyColliderGizmos>,
    query: Query<(&Transform, &Collider)>,
) {
    for (transform, collider) in query.iter() {
        let obb = Obb3d::from_transform(transform, collider);

        my_gizmos.primitive_3d(
            &Cuboid {
                half_size: obb.half_extents,
            },
            Isometry3d::new(obb.center, Quat::from_mat3(&obb.axes)),
            WHITE,
        );
    }
}
