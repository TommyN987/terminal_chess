use std::sync::Arc;

use async_trait::async_trait;
use protocol::packet::Packet;

use crate::{global_state::GlobalState, ids::PlayerId};

#[async_trait]
pub trait PacketHandler: Send + Sync {
    async fn handle(
        &self,
        packet: Packet,
        global_state: Arc<GlobalState>,
        player_id: &PlayerId,
    ) -> Result<(), String>;
}
