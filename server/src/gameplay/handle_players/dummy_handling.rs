use bevy::prelude::*;

#[derive(Debug, Component, Reflect, Default)]
#[reflect(Component)]
pub struct DummyClientMarker;

pub fn add_observers_to_dummies(
    trigger: Trigger<OnAdd, DummyClientMarker>,
    mut commands: Commands,
) {
}

// TODO: Simulate receiving commands from clients, for movement and stuff
