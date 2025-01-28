use bevy::prelude::*;

use super::lib::{GameState, TickIncreasedEvent};

pub fn increment_tick(mut commands: Commands, mut state: ResMut<GameState>) {
    state.tick += 1;

    commands.trigger(TickIncreasedEvent);
}
