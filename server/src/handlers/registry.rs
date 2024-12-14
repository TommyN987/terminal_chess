use std::{collections::HashMap, sync::Arc};
use strum::IntoEnumIterator;

use protocol::packet::{Packet, PacketType};

use crate::global_state::GlobalState;

use super::{game_request::GameRequestHandler, PacketHandler};

pub struct HandlerRegistry {
    handlers: HashMap<PacketType, Box<dyn PacketHandler + Send + Sync>>,
}

impl HandlerRegistry {
    pub fn new() -> Self {
        let mut handlers: HashMap<PacketType, Box<dyn PacketHandler + Send + Sync>> =
            HashMap::new();

        for packet_type in PacketType::iter() {
            match packet_type {
                PacketType::GameRequest => {
                    handlers.insert(packet_type, Box::new(GameRequestHandler));
                }
                _ => {}
            }
        }

        Self { handlers }
    }

    pub async fn process_packet(
        &self,
        packet: Packet,
        global_state: Arc<GlobalState>,
    ) -> Result<(), String> {
        if let Some(handler) = self.handlers.get(&packet.packet_type()) {
            handler.handle(packet, global_state).await
        } else {
            Err(format!(
                "No handler registered for packet type: {:?}",
                packet.packet_type()
            ))
        }
    }
}
