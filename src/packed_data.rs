use crate::types;

#[derive(Clone)]
pub struct PackedData {
    pub data: Vec<u8>,
}

impl PackedData {
    pub fn into_iter(self) -> PackedDataIterator {
        PackedDataIterator {
            // items_used: 0,
            byte_offset: 0,
            data: self,
        }
    }
}

// Usage:
//     read_type!(read_varint, types::VarInt);
// Produces:
//     read_varint(&mut self) -> types::VarInt {...}
macro_rules! read_type {
    ($name:ident, $type:ty) => {
        pub fn $name(&mut self) -> $type {
            let var = <$type>::from_stream(&self.data.data[self.byte_offset..]);
            self.byte_offset += var.len();
            return var;
        }
    };
}

pub struct PackedDataIterator {
    pub byte_offset: usize,
    pub data: PackedData,
}

impl PackedDataIterator {
    pub fn debug(&mut self) {
        println!("{:02X?}", &self.data.data[self.byte_offset..])
    }

    pub fn rest(self) -> Vec<u8> {
        self.data.data[self.byte_offset..].into()
    }

    read_type!(read_varint, types::VarInt);
    read_type!(read_string, types::MCString);
    read_type!(read_unsigned_short, types::UnsignedShort);
    read_type!(read_long, types::Long);
    read_type!(read_uuid, types::UUID);
    read_type!(read_byte, types::Byte);
    read_type!(read_unsigned_byte, types::UnsignedByte);
    read_type!(read_boolean, types::Boolean);
    read_type!(read_identifier, types::Identifier);
}
