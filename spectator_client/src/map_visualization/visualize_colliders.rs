use bevy::{color::palettes::css::WHITE, prelude::*};
use shared::game::collision_handling::{components::Collider, structs::Obb3d};

use crate::VisualOffset;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyColliderGizmos {}

pub fn visualize_colliders(
    mut my_gizmos: Gizmos<MyColliderGizmos>,
    query: Query<(&Transform, &Collider, Option<&VisualOffset>)>,
) {
    for (transform, collider, visual_offset) in query.iter() {
        let mut transform = *transform;
        if let Some(offset) = visual_offset {
            transform.translation += offset.0;
        }

        let obb = Obb3d::from_transform(&transform, collider);

        my_gizmos.primitive_3d(
            &Cuboid {
                half_size: obb.half_size.into(),
            },
            Isometry3d::new(obb.center, Quat::from_mat3a(&obb.basis)),
            WHITE,
        );
    }
}
