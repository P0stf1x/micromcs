use crate::types;

#[derive(Clone)]
pub struct MCString {
    data: Vec<u8>,
}

impl MCString {
    pub fn read(stream: &[u8]) -> (String, usize) {
        let (string_len, varint_len) = types::VarInt::read(stream);
        let string_data = String::from_utf8(stream[varint_len..varint_len+(string_len as usize)].to_vec()).unwrap();
        return (string_data, varint_len + string_len as usize);
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (string_data, _) = Self::read(stream);
        Self::new(string_data)
    }

    pub fn write(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(self.data.len() + 3);
        result.append(&mut types::VarInt::new(self.data.len() as i32).write());
        result.append(&mut (self.data.clone()));
        return result;
    }

    pub fn new(string: String) -> Self {
        Self {
            data: string.into(),
        }
    }

    pub fn data(&self) -> String {
        unsafe {
            String::from_utf8_unchecked(self.data.clone())
        }
    }

    pub fn len(&self) -> usize {
        self.write().len()
    }
}
