use bevy::{color::palettes::css::WHITE, prelude::*};
use shared::{
    game::collision_handling::{components::Collider, structs::Obb3d},
    networking::lobby_management::{LobbyState, MyLobby},
};

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
                half_size: obb.half_size.into(),
            },
            Isometry3d::new(obb.center, Quat::from_mat3a(&obb.basis)),
            WHITE,
        );
    }
}

pub fn visualize_world(mut my_gizmos: Gizmos<MyColliderGizmos>, lobbies: Query<&MyLobby>) {
    for lobby in lobbies.iter() {
        if lobby.state != LobbyState::InProgress {
            continue;
        }

        // Tiles is a Vec<Vec<f32>> where each f32 represents the height of the tile. enumerate the tiles and draw a cube for each tile
        lobby
            .map_config
            .as_ref()
            .expect("MapConfig should exist")
            .map
            .tiles
            .iter()
            .enumerate()
            .for_each(|(y, row)| {
                row.iter().enumerate().for_each(|(x, height)| {
                    let top_of_tile = lobby
                        .map_config
                        .as_ref()
                        .unwrap()
                        .map
                        .get_real_world_position_of_tile((x, y))
                        .unwrap();
                    let center = Vec3::new(top_of_tile.x, *height / 2.0, top_of_tile.z);
                    let half_size = Vec3::new(0.5, *height / 2.0, 0.5);
                    my_gizmos.primitive_3d(
                        &Cuboid { half_size },
                        Isometry3d::new(center, Quat::IDENTITY),
                        WHITE,
                    );
                });
            });
    }
}
