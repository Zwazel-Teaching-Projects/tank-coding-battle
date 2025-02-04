use bevy::prelude::*;
use handle_first_contact::handle_first_contact_message;

pub mod handle_first_contact;

pub struct MyLobbyManagement;

impl Plugin for MyLobbyManagement {
    fn build(&self, app: &mut App) {
        app.add_observer(handle_first_contact_message);
    }
}
