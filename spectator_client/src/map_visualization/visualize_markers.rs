use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::css::YELLOW, prelude::*};
use shared::networking::messages::message_data::game_starts::GameStarts;

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

        // TODO: Different marker types
        // let marker_type = &marker.kind;

        let tile = &marker.tile;
        let position = map_definition
            .get_real_world_position_of_tile(tile.x, tile.y)
            .expect("Failed to get real world position of tile");
        let rotation = Quat::from_rotation_x(-FRAC_PI_2);

        my_gizmos.circle(Isometry3d::new(position, rotation), 0.5, color);
    }
}
