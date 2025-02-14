use std::f32::consts::FRAC_PI_2;

use bevy::{color::palettes::css::RED, prelude::*};
use shared::networking::messages::message_data::game_starts::GameStarts;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct MyMarkerGizmos {}

pub fn draw_markers(mut my_gizmos: Gizmos<MyMarkerGizmos>, game_start: Res<GameStarts>) {
    let map_definition = &game_start.map_definition;

    for marker in &map_definition.markers {
        let group = &marker.group;

        let color = game_start
            .team_configs
            .get(group)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(RED.into());

        let tile = &marker.tile;
        let position = map_definition.get_real_world_position(tile.x, tile.y);
        let rotation = Quat::from_rotation_x(-FRAC_PI_2);

        my_gizmos.circle(Isometry3d::new(position, rotation), 0.5, color);
    }
}
