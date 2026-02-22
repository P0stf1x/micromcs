use crate::types;

pub struct NetworkPacket {
    id: i32,
    data: Vec<u8>,
}

impl NetworkPacket {

    pub fn new(buffer: &[u8]) -> Self {
        let (_, bytes_read_first) = types::VarInt::read(buffer);
        let temp_buffer = buffer.split_at(bytes_read_first as usize).1;

        // Packet ID
        let (packet_id, bytes_read_second) = types::VarInt::read(temp_buffer);
        let data = Vec::from(temp_buffer.split_at(bytes_read_second as usize).1);

        Self {
            id: packet_id,
            data,
        }
    }

    pub fn data_get(&self) -> Vec<u8> {
        self.data.clone()
    }

    pub fn id(&self) -> i32 {
        self.id
    }
}
