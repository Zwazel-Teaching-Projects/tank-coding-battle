use bevy::prelude::*;

use crate::networking::shared::lib::MessageContainer;

#[derive(Default, Resource, Reflect)]
#[reflect(Resource)]
pub struct QueuedMessages {
    pub messages: Vec<MessageContainer>,
}
