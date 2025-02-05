use bevy::prelude::*;
use shared::networking::messages::message_container::FirstContactTrigger;

use crate::networking::handle_clients::lib::{AwaitingFirstContact, ClientDisconnectedTrigger};

pub fn handle_awaiting_first_contact(
    mut commands: Commands,
    mut clients: Query<(Entity, &mut AwaitingFirstContact)>,
    time: Res<Time>,
) {
    for (entity, mut timer) in clients.iter_mut() {
        if timer.0.tick(time.delta()).finished() {
            info!("Client {:?} timed out waiting for first contact", entity);
            commands.trigger(ClientDisconnectedTrigger(entity));
        }
    }
}

// Proof of concept for handling a message using an observer
// We can even make targeted ones and only trigger for specific clients!
pub fn handle_first_contact_message(trigger: Trigger<FirstContactTrigger>, mut commands: Commands) {
    let message = &trigger.message;
    let sender = trigger.sender;
    info!(
        "Received first contact message: {:?} from {:?}",
        message, sender
    );

    // For now, just remove the timer
    commands.entity(sender).remove::<AwaitingFirstContact>();
}
