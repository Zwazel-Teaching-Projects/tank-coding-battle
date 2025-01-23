use bevy::prelude::*;

use std::sync::{Arc, Mutex};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

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

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Create a shared state resource. We'll insert this into the Bevy world
    // AND also move a clone into the network task.
    let shared_state = SharedGameState::default();

    // Launch a background task to handle incoming TCP connections.
    // You could also store the 'listener' in the resource if you want more control.
    let network_state = shared_state.clone();
    tokio::spawn(async move {
        if let Err(e) = run_network_task(network_state).await {
            eprintln!("Network Task Error: {:?}", e);
        }
    });

    // Now run our Bevy app in the current thread (the "main" thread).
    App::new()
        .insert_resource(shared_state)
        .add_plugins((MinimalPlugins, GamePlugin))
        .run();

    Ok(())
}

/// An async function that sets up a listener and processes connections.
async fn run_network_task(shared: SharedGameState) -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("Network: Listening on 127.0.0.1:9999");

    // Accept connections in a loop (this example processes one at a time; you might spawn tasks).
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Network: Client connected: {:?}", addr);

        // Handle the connection (in a real scenario, you might spawn a new task each time).
        if let Err(e) = handle_connection(socket, &shared).await {
            eprintln!("Error handling client {}: {:?}", addr, e);
        }
    }
}

/// Process a single client connection.
async fn handle_connection(socket: TcpStream, shared: &SharedGameState) -> std::io::Result<()> {
    // Split into a read half and write half
    let (read_half, mut write_half) = socket.into_split();
    let mut reader = BufReader::new(read_half);

    // Write a greeting
    write_half
        .write_all(
            format!(
                "Hello from Bevy+Tokio! Current game frame: {}\n",
                shared.data.lock().unwrap().frame
            )
            .as_bytes(),
        )
        .await?;

    // Read a line from the client
    let mut buf = String::new();
    let bytes_read = reader.read_line(&mut buf).await?;
    if bytes_read == 0 {
        println!("Client disconnected immediately.");
        return Ok(());
    }
    println!("Network: received line -> {}", buf.trim_end());

    // Show how we can read the "game frame" from shared state
    let frame_count = {
        let data = shared.data.lock().unwrap();
        data.frame
    };
    // Send the current frame count back to the client
    let response = format!("Current game frame: {}\n", frame_count);
    write_half.write_all(response.as_bytes()).await?;

    println!("Network: closing connection");
    Ok(())
}
