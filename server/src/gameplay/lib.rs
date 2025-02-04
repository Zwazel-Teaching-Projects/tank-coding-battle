use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Reflect, Event)]
pub struct StartNextTickProcessing;

#[derive(Debug, Default, Reflect, Event)]
pub struct TickIncreasedTrigger;

#[derive(Debug, Default, Resource, Reflect, Serialize, Deserialize, Clone)]
#[reflect(Resource)]
pub struct GameState {
    pub tick: u64,
}
