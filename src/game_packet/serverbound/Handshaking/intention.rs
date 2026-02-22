use tracing::{debug, trace};

use crate::network_handler::Connection;
use crate::packed_data::PackedDataIterator;
use crate::game_packet::GamePacket;

#[derive(Debug)]
pub enum IntentionPacketIntent {
    Status   = 1,
    Login    = 2,
    Transfer = 3,
}

pub struct IntentionPacket {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub intent: IntentionPacketIntent,
}

impl IntentionPacket {
    pub fn new(mut packet_data: PackedDataIterator) -> Self {
        let protocol_version = packet_data.read_varint();
        let server_address = packet_data.read_string();
        let server_port = packet_data.read_unsigned_short();
        let intent_value = packet_data.read_varint();
        let intent = match intent_value.data() {
            1 => IntentionPacketIntent::Status,
            2 => IntentionPacketIntent::Login,
            3 => IntentionPacketIntent::Transfer,
            v => panic!("Wrong intent value: {}", v), // TODO: there should be a good way to close such connections from inside
        };

        Self {
            protocol_version: protocol_version.data(),
            server_address: server_address.data(),
            server_port: server_port.data(),
            intent,
        }
    }
}

impl<'a> GamePacket<'a> for IntentionPacket {
    fn log(&self) {
        debug!("Recieved Intention packet");
        trace!("Intention packet data:");
        trace!("Protocol version: {}", self.protocol_version);
        trace!("Connection address: {}", self.server_address);
        trace!("Connection port: {}", self.server_port);
        trace!("Intention: {:?}", self.intent);
    }

    fn update_connection(&self, conn: *mut Connection) {
        unsafe {
            (*conn).change_state(
                match self.intent {
                    IntentionPacketIntent::Status   => crate::network_handler::ConnectionState::Status,
                    IntentionPacketIntent::Login    => crate::network_handler::ConnectionState::Login,
                    IntentionPacketIntent::Transfer => crate::network_handler::ConnectionState::Login,
                }
            );
        };
    }
}
