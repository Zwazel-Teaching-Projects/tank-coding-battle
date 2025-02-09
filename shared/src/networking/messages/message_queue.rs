use bevy::prelude::*;

use std::collections::VecDeque;

use super::message_container::MessageContainer;

/// All Messages we RECEIVE
#[derive(Debug, Default, Reflect, Clone, PartialEq, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct InMessageQueue(pub MessageQueue);

/// All Messages we SEND
#[derive(Debug, Default, Reflect, Clone, PartialEq, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct OutMessageQueue(pub MessageQueue);

#[derive(Debug, Default, Reflect, Clone, Deref, DerefMut, PartialEq)]
pub struct MessageQueue(VecDeque<MessageContainer>);

/// All Errors the server sends to the client
#[derive(Debug, Default, Reflect, Clone, PartialEq, Component, Deref, DerefMut)]
#[reflect(Component)]
pub struct ErrorMessageQueue(pub MessageQueue);