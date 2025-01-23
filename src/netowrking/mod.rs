use std::sync::{Arc, Mutex};

use bevy::prelude::*;

use bevy_tokio_tasks::{TokioTasksPlugin, TokioTasksRuntime};

use lib::MyConnectedClients;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime;

use crate::config::{ConfigLoadState, MyConfig};
use crate::SharedGameState;

mod lib;

pub struct MyNetworkingPlugin;

impl Plugin for MyNetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TokioTasksPlugin::default())
            .add_systems(
                OnEnter(ConfigLoadState::Loaded),
                (init_tcp_listener,).chain(),
            )
            .init_resource::<MyConnectedClients>();
    }
}

fn init_tcp_listener(runtime: ResMut<TokioTasksRuntime>, config: Res<MyConfig>) {
    let config = config.clone();
    runtime.spawn_background_task(|mut ctx| async move {
        let config = config.clone();
        if let Ok(listener) =
            TcpListener::bind(format!("{}:{}", &config.server_ip, config.server_port)).await
        {
            info!(
                "Network: Listening on {}:{}",
                &config.server_ip, config.server_port
            );

            loop {
                let (socket, addr) = listener.accept().await.unwrap();

                info!("Network: Client connected: {:?}", addr);

                ctx.run_on_main_thread(move |ctx| {
                    let world = ctx.world;
                    let connected_clients = world.get_resource_mut::<MyConnectedClients>().unwrap();
                    connected_clients.0.lock().unwrap().insert(addr, socket);
                })
                .await;
            }
        } else {
            panic!(
                "Failed to bind to {}:{}",
                &config.server_ip, config.server_port
            );
        }
    });
}

fn send_messages(
    runtime: ResMut<TokioTasksRuntime>,
    shared: Res<SharedGameState>,
    connected_clients: Res<MyConnectedClients>,
) {
    let connected_clients = connected_clients.0.clone();
    let shared = shared.clone();

    runtime.spawn_background_task(|mut ctx| async move {
        let connected_clients = connected_clients.lock().unwrap();
        for (addr, socket) in connected_clients.iter() {
            socket.write_all(b"Hello from Bevy+Tokio!\n").await.unwrap();
        }
    });
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
