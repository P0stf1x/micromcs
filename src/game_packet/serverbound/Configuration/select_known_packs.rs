use tracing::{debug, trace};

use crate::game_packet::clientbound::ServerResponse;
use crate::packed_data::PackedDataIterator;
use crate::game_packet::GamePacket;

pub struct KnownPacksPacket {
}

impl KnownPacksPacket {
    pub fn new(mut packet_data: PackedDataIterator) -> Self {
        debug!("Recieved Client Information packet");
        let packs_amount = packet_data.read_varint().data();
        for _ in 0..packs_amount {
            let namespace = packet_data.read_string();
            let id = packet_data.read_string();
            let version = packet_data.read_string();
            trace!("client know {}:{} v`{}`", namespace.data(), id.data(), version.data())
        }

        Self {}
    }
}

impl<'a> GamePacket<'a> for KnownPacksPacket {
    fn log(&self) {
        // FIXME: we don't store known packs for now, so it's logged in new() function
    }

    fn respond<'b>(&self, mut send_data: Box<dyn FnMut(ServerResponse) + 'b>) {
        unimplemented!()
    }
}
