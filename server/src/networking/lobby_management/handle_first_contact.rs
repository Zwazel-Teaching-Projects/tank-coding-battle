use bevy::prelude::*;
use shared::networking::messages::message_container::FirstContactTrigger;

// Proof of concept for handling a message using an observer
// We can even make targeted ones and only trigger for specific clients!
pub fn handle_first_contact_message(trigger: Trigger<FirstContactTrigger>) {
    info!("Received first contact message: {:?}", trigger.0);
}
