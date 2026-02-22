use tracing::debug;

use crate::game_packet::GamePacket;
use crate::types;

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

    fn respond<'b>(&self, mut send_data: Box<dyn FnMut(i32, Vec<u8>) + 'b>) {
        let sample_img = "data:image/png;base64,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";
        // temporary example taken from https://minecraft.wiki/w/Java_Edition_protocol/Server_List_Ping#Status_Response
        let json_data = format!("{{
    \"version\": {{
        \"name\": \"1.21.11\",
        \"protocol\": 774
    }},
    \"players\": {{
        \"max\": 20,
        \"online\": 1,
        \"sample\": [
            {{
                \"name\": \"thinkofdeath\",
                \"id\": \"4566e69f-c907-48ee-8d71-d7ba5aa00d20\"
            }}
        ]
    }},
    \"description\": {{
        \"text\": \"Hello, world!\"
    }},
    \"favicon\": \"{}\",
    \"enforcesSecureChat\": false
}}", sample_img);

        send_data(0x00, types::MCString::new(json_data).write());
    }
}
