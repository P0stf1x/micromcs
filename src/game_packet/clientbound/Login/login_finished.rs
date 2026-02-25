use crate::game_packet::clientbound::ServerResponse;
use crate::types;

pub struct LoginFinishedPacket {
    player_uuid: types::UUID,
}

impl LoginFinishedPacket {
    pub fn new(player_uuid: types::UUID) -> Self {
        Self {
            player_uuid,
        }
    }

    pub fn send(&self) -> ServerResponse {
        let url = format!("https://sessionserver.mojang.com/session/minecraft/profile/{}", self.player_uuid.format());
        let result = reqwest::blocking::get(url);
        let player_data: types::GameProfile = result.unwrap().json().expect("msg");
        return ServerResponse::new(
            0x02,
            player_data.write()
        );
    }
}
