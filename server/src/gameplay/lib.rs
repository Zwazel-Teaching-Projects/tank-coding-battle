use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Reflect, Event)]
pub struct StartNextTickProcessing;

#[derive(Debug, Default, Reflect, Event)]
pub struct TickIncreasedTrigger;