use bevy::{color::palettes::css::WHITE, prelude::*};
use shared::networking::messages::message_container::GameStartsTrigger;

pub fn create_player_visualisation(
    trigger: Trigger<GameStartsTrigger>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let game_start = trigger.event();

    for player in game_start.connected_clients.iter() {
        let team_color = game_start
            .team_configs
            .get(&player.client_team)
            .map(|config| Color::from(config.color.clone()))
            .unwrap_or(WHITE.into());

        commands.spawn((
            Name::new(player.client_name.clone()),
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(team_color)),
        ));
    }
}
