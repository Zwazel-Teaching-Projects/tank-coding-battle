use bevy::prelude::*;

use std::collections::VecDeque;

use super::message_container::MessageContainer;

/// All Messages we SEND
#[derive(Debug, Default, Reflect, Clone, PartialEq, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct OutMessageQueue(pub MessageQueue);

/// All Messages we want the server to send immediately (e.g. error messages)
#[derive(Debug, Default, Reflect, Clone, PartialEq, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct ImmediateOutMessageQueue(pub MessageQueue);

#[derive(Debug, Default, Reflect, Clone, Deref, DerefMut, PartialEq)]
pub struct MessageQueue(VecDeque<MessageContainer>);
