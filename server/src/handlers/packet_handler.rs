use std::sync::Arc;

use async_trait::async_trait;
use protocol::packet::Packet;

use crate::global_state::GlobalState;

#[async_trait]
pub trait PacketHandler: Send + Sync {
    async fn handle(&self, packet: Packet, global_state: Arc<GlobalState>) -> Result<(), String>;
}
