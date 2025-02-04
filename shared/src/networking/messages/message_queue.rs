use std::collections::VecDeque;

use bevy::prelude::*;

use super::message_container::MessageContainer;

#[derive(Debug, Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct OutgoingMessageQueue {
    pub messages: VecDeque<MessageContainer>,
}
