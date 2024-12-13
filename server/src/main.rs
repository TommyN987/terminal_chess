use std::net::SocketAddr;

use ids::PlayerId;
use matchmaker::{GameRequest, Matchmaker};
use protocol::packet::{PacketFramer, PacketType};
use tokio::{
    io::AsyncReadExt,
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use uuid::Uuid;

mod game;
mod ids;
mod matchmaker;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    let (matchmaker_tx, matchmaker_rx) = mpsc::channel(100);
    let mut matchmaker = Matchmaker::new(matchmaker_rx);
    tokio::spawn(async move { matchmaker.run().await });

    println!("Server running on {}", addr);

    while let Ok((socket, addr)) = listener.accept().await {
        println!("New connection: {}", addr);
        tokio::spawn(handle_client(socket, addr, matchmaker_tx.clone()));
    }
}

async fn handle_client(
    mut socket: TcpStream,
    addr: SocketAddr,
    matchmaker_tx: mpsc::Sender<GameRequest>,
) {
    println!("Handling client: {}", addr);

    let mut framer = PacketFramer::new();
    let mut buffer = [0; 1024];

    loop {
        match socket.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected", addr);
                break;
            }
            Ok(n) => {
                if let Err(err) = process_data(&mut framer, &buffer[..n], &matchmaker_tx).await {
                    eprintln!("Error processing data from {}: {}", addr, err);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
                break;
            }
        }
    }
}

async fn process_data(
    framer: &mut PacketFramer,
    data: &[u8],
    matchmaker_tx: &mpsc::Sender<GameRequest>,
) -> Result<(), String> {
    if let Some(packet) = framer.push(data).map_err(|e| e.to_string())? {
        match packet.packet_type() {
            PacketType::GameRequest => {
                println!("Received a game request.");
                let payload = packet.payload();

                let player_id = Uuid::from_slice(payload)
                    .map_err(|e| format!("Failed to parse Uuid from payload: {}", e))?;

                let request = GameRequest {
                    player_id: PlayerId::from(player_id),
                };

                matchmaker_tx
                    .send(request)
                    .await
                    .map_err(|e| format!("Failed to send to matchmaker: {}", e))?;
            }
            PacketType::MovePiece => {
                println!("Received a move.");
                todo!()
            }
            _ => {
                println!("Unhandled packet type.");
                todo!()
            }
        }
    }
    Ok(())
}
