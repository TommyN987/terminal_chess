use std::sync::Arc;

use async_trait::async_trait;
use protocol::packet::Packet;
use uuid::Uuid;

use crate::{global_state::GlobalState, ids::PlayerId, matchmaker::GameRequest};

use super::packet_handler::PacketHandler;

pub struct GameRequestHandler;

#[async_trait]
impl PacketHandler for GameRequestHandler {
    async fn handle(&self, packet: Packet, global_state: Arc<GlobalState>) -> Result<(), String> {
        println!("Processing GameRequest packet.");

        let payload = packet.payload();

        let player_id = Uuid::from_slice(payload)
            .map_err(|e| format!("Failed to parse Uuid from payload: {}", e))?;

        let _active_games = global_state.active_games.read().await;

        // TODO: Check if the player is already in an active game. If so, return an error.

        let request = GameRequest {
            player_id: PlayerId::from(player_id),
        };

        let matchmaker_tx = global_state
            .matchmaker
            .read()
            .await
            .as_ref()
            .ok_or_else(|| "Matchmaker is not initialized.".to_string())?
            .clone();

        matchmaker_tx
            .send(request)
            .await
            .map_err(|e| format!("Failed to send to matchmaker: {}", e))?;

        Ok(())
    }
}
