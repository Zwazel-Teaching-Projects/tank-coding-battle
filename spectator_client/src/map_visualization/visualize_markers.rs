use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::css::YELLOW, prelude::*};
use shared::{
    asset_handling::maps::MarkerType, networking::messages::message_data::game_starts::GameStarts,
};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyMarkerGizmos {}

pub fn draw_markers(mut my_gizmos: Gizmos<MyMarkerGizmos>, game_config: Res<GameStarts>) {
    let map_definition = &game_config.map_definition;

    for marker in &map_definition.markers {
        let color = game_config
            .team_configs
            .get(&marker.group)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(YELLOW.into());

        let marker_type = &marker.kind;
        let tile = &marker.tile;
        let position = map_definition
            .get_real_world_position_of_tile((tile.x, tile.y))
            .expect("Failed to get real world position of tile");
        let rotation = Quat::from_rotation_x(-FRAC_PI_2);

        match marker_type {
            MarkerType::Spawn { .. } => {
                my_gizmos.circle(Isometry3d::new(position, rotation), 0.5, color);
            }
            // TODO: This should be a tracked object, as it can move. the marker in the map definition only defines the initial position
            MarkerType::Flag => {
                let height = 0.5;
                let width = 0.25;
                my_gizmos.primitive_3d(
                    &Cuboid {
                        half_size: Vec3::new(width, height, width),
                    },
                    Isometry3d::new(position + Vec3::new(0.0, height, 0.0), Quat::IDENTITY),
                    color,
                );
            }
        }
    }
}
