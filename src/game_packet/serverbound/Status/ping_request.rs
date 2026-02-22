use tracing::{debug, trace};

use crate::types;
use crate::packed_data::PackedDataIterator;
use crate::game_packet::GamePacket;

pub struct PingPacket {
    timestamp: i64,
}

impl PingPacket {
    pub fn new(mut packet_data: PackedDataIterator) -> Self {
        let timestamp = packet_data.read_long();

        Self {
            timestamp: timestamp.data()
        }
    }
}

impl<'a> GamePacket<'a> for PingPacket {
    fn log(&self) {
        debug!("Recieved Ping packet");
        trace!("Ping packet data:");
        trace!("Timestamp: {}", self.timestamp);
    }

    fn respond<'b>(&self, mut send_data: Box<dyn FnMut(i32, Vec<u8>) + 'b>) {
        send_data(0x01, types::Long::new(self.timestamp).write());
    }
}
