use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Bind our server to localhost:9999
    let listener = TcpListener::bind("127.0.0.1:9999").await?;
    println!("Rust Server: listening on 127.0.0.1:9999");

    // Accept a single client connection
    let (mut socket, addr) = listener.accept().await?;
    println!("Rust Server: client connected: {}", addr);

    // Send a greeting
    socket.write_all(b"Hello from Rust!\n").await?;
    println!("Rust Server: sent greeting to client");

    // Prepare to read the client's response
    let mut reader = BufReader::new(socket);
    let mut buffer = String::new();

    // Read one line from the client
    let bytes_read = reader.read_line(&mut buffer).await?;
    if bytes_read == 0 {
        println!("Rust Server: client closed connection without response");
        return Ok(());
    }

    println!("Rust Server: received from client -> {}", buffer.trim_end());

    println!("Rust Server: shutting down");
    Ok(())
}
