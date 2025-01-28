use bevy::prelude::*;

#[derive(Debug, Default, Reflect, Event)]
pub struct StartNextTickProcessing;

#[derive(Debug, Default, Reflect, Event)]
pub struct TickIncreasedTrigger;

#[derive(Debug, Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct GameState {
    pub tick: u64,
}
