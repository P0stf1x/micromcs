use crate::game_packet::clientbound::ServerResponse;
use crate::types;

pub struct PongPacket {
    timestamp: i64,
}

impl PongPacket {
    pub fn new(timestamp: i64) -> Self {
        Self {
            timestamp,
        }
    }

    pub fn send(&self) -> ServerResponse {
        return ServerResponse::new(
            0x01,
            types::Long::new(self.timestamp).write()
        );
    }
}
