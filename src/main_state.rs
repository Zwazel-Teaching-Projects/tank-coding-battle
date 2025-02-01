use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MyMainState {
    #[default]
    SettingUp,
    Ready,
}