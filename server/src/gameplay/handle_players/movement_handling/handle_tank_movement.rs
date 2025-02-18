use bevy::prelude::*;
use shared::networking::messages::message_container::MoveTankCommandTrigger;

pub fn handle_tank_movement(trigger: Trigger<MoveTankCommandTrigger>) {
    info!("Handling tank movement for entity: {}", trigger.entity());
}
