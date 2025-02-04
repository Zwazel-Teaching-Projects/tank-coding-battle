use std::collections::VecDeque;

use bevy::prelude::*;
use shared::networking::messages::message_container::MessageContainer;

#[derive(Debug, Resource, Reflect, Default, Deref, DerefMut)]
#[reflect(Resource)]
pub struct OutgoingMessageQueue {
    pub messages: VecDeque<MessageContainer>,
}
