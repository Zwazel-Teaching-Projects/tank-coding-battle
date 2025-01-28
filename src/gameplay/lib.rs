use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Event)]
pub struct TickIncreasedEvent;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameState {
    pub tick: u64,
}
