use bevy::prelude::*;
use lib::{MessageContainer, MessageTarget, NetworkMessageType};

pub mod lib;

pub struct MySharedPlugin;

impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MessageContainer>()
            .register_type::<NetworkMessageType>()
            .register_type::<MessageTarget>();
    }
}
