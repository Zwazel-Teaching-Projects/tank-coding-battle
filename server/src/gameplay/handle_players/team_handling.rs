use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Clone, Component)]
#[reflect(Component)]
pub struct InTeam {
    pub team_name: String,
}