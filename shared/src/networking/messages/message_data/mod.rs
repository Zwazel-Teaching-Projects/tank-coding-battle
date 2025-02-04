use bevy::prelude::*;

pub mod first_contact;

pub struct MySharedMessageDataPlugin;

impl Plugin for MySharedMessageDataPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<first_contact::FirstContactData>();
    }
}
