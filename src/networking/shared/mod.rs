use bevy::prelude::*;
use lib::{MessageContainer, MessageTarget, NetworkMessageTypes};

pub mod lib;

pub struct MySharedPlugin;

impl Plugin for MySharedPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MessageContainer>()
            .register_type::<NetworkMessageTypes>()
            .register_type::<MessageTarget>();
    }
}
