use tracing::{debug, trace};

use crate::game_packet::clientbound::{self, ServerResponse};
use crate::packed_data::PackedDataIterator;
use crate::game_packet::GamePacket;
use crate::types;

pub struct HelloPacket {
    player_name: String,
    uuid: u128,
}

impl HelloPacket {
    pub fn new(mut packet_data: PackedDataIterator) -> Self {
        packet_data.debug();
        let player_name = packet_data.read_string();
        let uuid = packet_data.read_uuid();

        Self {
            player_name: player_name.data(),
            uuid: uuid.data(),
        }
    }
}

impl<'a> GamePacket<'a> for HelloPacket {
    fn log(&self) {
        debug!("Recieved Hello packet");
        trace!("Hello packet data:");
        trace!("Player name: {}", self.player_name);
        trace!("UUID: {:02x}", self.uuid);
    }

    fn respond<'b>(&self, mut send_data: Box<dyn FnMut(ServerResponse) + 'b>) {
        // TODO: implement encryption and compression

        let response = clientbound::Login::LoginFinishedPacket::new(types::UUID::new(self.uuid)).send();

        send_data(response);
    }
}
