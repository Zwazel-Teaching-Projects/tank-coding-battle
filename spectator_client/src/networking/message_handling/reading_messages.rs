use bevy::prelude::*;

use crate::networking::MyNetworkStream;

pub fn reading_messages(mut client: Query<&mut MyNetworkStream>) {}
