use std::{net::SocketAddr, sync::Arc};

use global_state::GlobalState;
use handlers::HandlerRegistry;
use matchmaker::Matchmaker;
use protocol::packet::PacketFramer;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::mpsc,
};

mod global_state;
mod handlers;
mod ids;
mod matchmaker;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    let global_state = Arc::new(GlobalState::default());

    let (matchmaker_tx, matchmaker_rx) = mpsc::channel(100);

    *global_state.matchmaker.write().await = Some(matchmaker_tx.clone());

    tokio::spawn(async move {
        let mut matchmaker = Matchmaker::new(matchmaker_rx);
        matchmaker.run().await
    });

    println!("Server running on {}", addr);

    while let Ok((socket, addr)) = listener.accept().await {
        println!("New connection: {}", addr);
        tokio::spawn(handle_client(socket, addr, Arc::clone(&global_state)));
    }
}

async fn handle_client(mut socket: TcpStream, addr: SocketAddr, global_state: Arc<GlobalState>) {
    println!("Handling client: {}", addr);

    let mut framer = PacketFramer::new();
    let mut buffer = [0; 1024];

    let handler_registry = HandlerRegistry::new();

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", addr);
                break;
            }
            Ok(n) => {
                if let Some(packet) = framer.push(&buffer[..n]).ok().flatten() {
                    if let Err(err) = handler_registry
                        .process_packet(packet, Arc::clone(&global_state))
                        .await
                    {
                        eprintln!("Error processing packet from {}: {}", addr, err);
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}
