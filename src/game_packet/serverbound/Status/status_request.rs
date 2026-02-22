use tracing::debug;

use crate::game_packet::clientbound::ServerResponse;
use crate::game_packet::{GamePacket, clientbound};

pub struct StatusPacket;

impl StatusPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> GamePacket<'a> for StatusPacket {
    fn log(&self) {
        debug!("Recieved Status packet");
    }

    fn respond<'b>(&self, mut send_data: Box<dyn FnMut(ServerResponse) + 'b>) {
        let response = clientbound::Status::StatusResponsePacket::new().send();

        send_data(response);
    }
}
