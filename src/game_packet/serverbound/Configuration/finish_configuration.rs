use tracing::debug;

use crate::game_packet::{GamePacket, clientbound::ServerResponse};

pub struct AcknowledgeFinishPacket {}

impl AcknowledgeFinishPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> GamePacket<'a> for AcknowledgeFinishPacket {
    fn log(&self) {
        debug!("Recieved Acknowledge Finish packet");
    }

    fn update_connection(&self, conn: *mut crate::network_handler::Connection) {
        unsafe {
            (*conn).change_state(crate::network_handler::ConnectionState::Play);
        }
    }

    fn respond<'b>(&self, mut send_response: Box<dyn FnMut(ServerResponse) + 'b>) {
        unimplemented!()
    }
}
