use tracing::warn;

use crate::{game_packet::clientbound::ServerResponse, network_handler::Connection, packed_data::PackedData};

pub mod serverbound;
pub mod clientbound;

pub fn get_game_packet<'a>(state: crate::network_handler::ConnectionState, packet_id: i32, data: Vec<u8>) -> Box<dyn GamePacket<'a>> {
    let data_stream = PackedData {
        data: Vec::<u8>::from(data),
    };
    let data_iter = data_stream.into_iter();
    use crate::network_handler::ConnectionState::*;
    match (state, packet_id) {
        (Handshaking,   0x00) => Box::new(serverbound::Handshaking::IntentionPacket::new(data_iter)),
        (Status,        0x00) => Box::new(serverbound::Status::StatusPacket::new()),
        (Status,        0x01) => Box::new(serverbound::Status::PingPacket::new(data_iter)),
        (Login,         0x00) => Box::new(serverbound::Login::HelloPacket::new(data_iter)),
        (Login,         0x03) => Box::new(serverbound::Login::LoginAcknowledgedPacket::new()),
        (Configuration, 0x02) => Box::new(serverbound::Configuration::CustomPayloadPacket::new(data_iter)),
        (Configuration, 0x00) => Box::new(serverbound::Configuration::ClientInformationPacket::new(data_iter)),
        (Configuration, 0x03) => Box::new(serverbound::Configuration::AcknowledgeFinishPacket::new()),
        (Configuration, 0x07) => Box::new(serverbound::Configuration::KnownPacksPacket::new(data_iter)),
        _ => panic!("Packet with status {:?} & id {} is not supported", state, packet_id),
    }
}

pub trait GamePacket<'a> {
    fn log(&self) {
        warn!("No logger set up for packet type (sorry, cannot help you with figuring out which packet type it is)")
    }

    fn update_connection(&self, _conn: *mut Connection) {}

    fn respond<'b>(&self, _send_response: Box<dyn FnMut(ServerResponse) + 'b>) {}
}
