use tracing::{debug, trace};

use crate::types;
use crate::packed_data::PackedDataIterator;
use crate::game_packet::GamePacket;

pub struct CustomPayloadPacket {
    identifier: types::Identifier,
    data: Vec<u8>,
}

impl CustomPayloadPacket {
    pub fn new(mut packet_data: PackedDataIterator) -> Self {
        let identifier = packet_data.read_identifier();
        let data = packet_data.rest();

        Self {
            identifier,
            data,
        }
    }
}

impl<'a> GamePacket<'a> for CustomPayloadPacket {
    fn log(&self) {
        debug!("Recieved Client Information packet");
        trace!("Custom payload ({}):\n{:02X?}", self.identifier.as_string(), self.data);
        // TODO: I think the best way of doing this would be to create a dictionary of namespaces, and each can register and provide handler
    }
}
