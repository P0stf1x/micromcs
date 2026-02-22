use tracing::{debug, trace};

use crate::packed_data::PackedDataIterator;
use crate::game_packet::GamePacket;

#[derive(Debug)]
pub enum ChatMode {
    Enabled  = 0,
    Commands = 1,
    Disabled = 2,
}

#[derive(Debug)]
pub enum ParticleStatus {
    All       = 0,
    Decreased = 1,
    Minimal   = 2,
}

pub struct ClientInformationPacket {
    locale: String,
    view_distance: i8,
    chat_mode: ChatMode,
    skin_parts: u8,
    main_hand_is_right: bool,
    text_filtering: bool,
    server_listing: bool,
    particle_status: ParticleStatus,
}

impl ClientInformationPacket {
    pub fn new(mut packet_data: PackedDataIterator) -> Self {
        let locale = packet_data.read_string();
        let view_distance = packet_data.read_byte();
        let chat_mode_value = packet_data.read_varint();
        let chat_mode = match chat_mode_value.data() {
            0 => ChatMode::Enabled,
            1 => ChatMode::Commands,
            2 => ChatMode::Disabled,
            v => panic!("Wrong chat mode value: {}", v),
        };
        let skin_parts  = packet_data.read_unsigned_byte(); // FIXME: it should rather be bitfield or something like that
        let main_hand  = packet_data.read_boolean();
        let text_filtering  = packet_data.read_boolean();
        let server_listing  = packet_data.read_boolean();
        let particle_status_value  = packet_data.read_varint().data();
        let particle_status = match particle_status_value {
            0 => ParticleStatus::All,
            1 => ParticleStatus::Decreased,
            2 => ParticleStatus::Minimal,
            v => panic!("Wrong particle status value: {}", v),
        };

        Self {
            locale: locale.data(),
            view_distance: view_distance.data(),
            chat_mode,
            skin_parts: skin_parts.data(),
            main_hand_is_right: main_hand.data(),
            text_filtering: text_filtering.data(),
            server_listing: server_listing.data(),
            particle_status,
        }
    }
}

impl<'a> GamePacket<'a> for ClientInformationPacket {
    fn log(&self) {
        debug!("Recieved Client Information packet");
        trace!("Client Information packet data:");
        trace!("Locale: {}", self.locale);
        trace!("View distance: {}", self.view_distance);
        trace!("Chat mode: {:?}", self.chat_mode);
        trace!("Skin parts: {:08b}", self.skin_parts);
        trace!("Main hand (is right): {}", self.main_hand_is_right);
        trace!("Text filtering: {}", self.text_filtering);
        trace!("Server listing: {}", self.server_listing);
        trace!("Particle status: {:?}", self.particle_status);
    }

    fn respond<'b>(&self, mut send_data: Box<dyn FnMut(i32, Vec<u8>) + 'b>) {
        unimplemented!()
    }
}
