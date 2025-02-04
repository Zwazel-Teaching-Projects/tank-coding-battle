use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MyNetworkingSet {
    AcceptConnections,
    ReadingMessages,
    SendingMessages,
}