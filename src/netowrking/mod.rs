use bevy::prelude::*;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};

use lib::MyTcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

use crate::config::{ConfigLoadState, MyConfig};
use crate::SharedGameState;

mod lib;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TokioTasksPlugin::default())
            .add_systems(OnEnter(ConfigLoadState::Loaded), init_tcp_listener);
    }
}

fn init_tcp_listener(runtime: ResMut<TokioTasksRuntime>, config: Res<MyConfig>) {
    let config = config.clone();
    runtime.spawn_background_task(|mut ctx| async move {
        let config = config.clone();
        if let Ok(listener) =
            TcpListener::bind(format!("{}:{}", &config.server_ip, config.server_port)).await
        {
            info!("Listening on {}:{}", &config.server_ip, config.server_port);

            ctx.run_on_main_thread(move |ctx| {
                let world = ctx.world;
                world.insert_resource(MyTcpListener(listener));
            })
            .await;
        } else {
            panic!(
                "Failed to bind to {}:{}",
                &config.server_ip, config.server_port
            );
        }

        println!(
            "Network: Listening on {}:{}",
            &config.server_ip, config.server_port
        );
    });
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
