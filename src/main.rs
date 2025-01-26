use bevy::prelude::*;

use config::MyConfigPlugin;
use networking::MyNetworkingPlugin;

use std::sync::{Arc, Mutex};

pub mod config;
mod networking;

// A simple resource to share game state with both Bevy systems and network tasks.
#[derive(Resource, Default, Clone)]
pub struct SharedGameState {
    pub data: Arc<Mutex<GameData>>,
}

// The actual game data (what you store is up to you).
#[derive(Default)]
pub struct GameData {
    pub frame: u64,
}

// This Bevy system runs each frame/tick and updates our game data.
fn game_update_system(shared: ResMut<SharedGameState>) {
    let mut data = shared.data.lock().unwrap();
    data.frame += 1;
    // For demonstration, we'll just increment a frame counter
    // In a real game, you'd update positions, handle collisions, etc.
    //println!("Bevy System: frame={}", data.frame);
}

// A small plugin that registers our system.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, game_update_system);
    }
}

fn main() {
    App::new()
        .insert_resource(SharedGameState::default())
        .add_plugins((
            DefaultPlugins,
            GamePlugin,
            MyConfigPlugin,
            MyNetworkingPlugin,
        ))
        .run();
}
