use tracing::debug;

use crate::network_handler::Connection;
use crate::game_packet::GamePacket;

pub struct LoginAcknowledgedPacket {}

impl LoginAcknowledgedPacket {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> GamePacket<'a> for LoginAcknowledgedPacket {
    fn log(&self) {
        debug!("Recieved Login Acknowledged packet");
    }

    fn update_connection(&self, conn: *mut Connection) {
        unsafe {
            (*conn).change_state(
                crate::network_handler::ConnectionState::Configuration
            );
        };
    }
}
