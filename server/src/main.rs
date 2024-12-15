use std::{net::SocketAddr, sync::Arc};

use global_state::GlobalState;
use handlers::HandlerRegistry;
use ids::PlayerId;
use matchmaker::Matchmaker;
use protocol::packet::PacketFramer;
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use uuid::Uuid;

mod game_session;
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

    let handler_registry = Arc::new(HandlerRegistry::new());

    let global_state = Arc::new(GlobalState::default());
    let global_state_clone = Arc::clone(&global_state);

    let (matchmaker_tx, matchmaker_rx) = mpsc::channel(100);

    *global_state.matchmaker.write().await = Some(matchmaker_tx.clone());

    tokio::spawn(async move {
        let mut matchmaker = Matchmaker::new(matchmaker_rx);
        matchmaker.run(global_state_clone).await
    });

    println!("Server running on {}", addr);

    while let Ok((socket, addr)) = listener.accept().await {
        println!("New connection: {}", addr);
        tokio::spawn(handle_client(
            socket,
            addr,
            Arc::clone(&global_state),
            Arc::clone(&handler_registry),
        ));
    }
}

async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    global_state: Arc<GlobalState>,
    handler_registry: Arc<HandlerRegistry>,
) {
    println!("Handling client: {}", addr);

    let mut framer = PacketFramer::new();
    let mut buffer = [0; 1024];

    let (mut read_half, write_half) = socket.into_split();

    let player_id = PlayerId::from(Uuid::new_v4());

    global_state
        .active_connections
        .write()
        .await
        .insert(player_id.clone(), write_half);

    loop {
        match read_half.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", addr);
                break;
            }
            Ok(n) => {
                if let Some(packet) = framer.push(&buffer[..n]).ok().flatten() {
                    if let Err(err) = handler_registry
                        .process_packet(packet, Arc::clone(&global_state), &player_id)
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
