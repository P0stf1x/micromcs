use crate::types;

#[derive(Clone)]
pub struct Identifier {
    namespace: String,
    value: String,
}

impl Identifier {
    pub fn read(stream: &[u8]) -> (String, String, usize) {
        let (string_len, varint_len) = types::VarInt::read(stream);
        let identifier = String::from_utf8(stream[varint_len..varint_len+(string_len as usize)].to_vec()).unwrap();
        let (namespace, value) = Self::split_string(identifier);
        (namespace, value, (string_len as usize) + varint_len)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (namespace, value, _) = Self::read(stream);
        Self::new(namespace, value)
    }

    pub fn write(&self) -> Vec<u8> {
        let mut string_len_bytes = types::VarInt::new(self.namespace.len() as i32 + 1 + self.value.len() as i32).write();
        let mut result = Vec::<u8>::with_capacity(string_len_bytes.len() + self.namespace.len() + 1 + self.value.len());
        result.append(&mut string_len_bytes);
        result.append(&mut self.namespace.clone().into());
        result.append(&mut ":".into());
        result.append(&mut self.value.clone().into());
        return result;
    }

    pub fn new(namespace: String, value: String) -> Self {
        Self {
            namespace,
            value,
        }
    }

    pub fn default(str: &str) -> Self {
        let namespace = "minecraft".into();
        let value = str.into();
        Self::new(namespace, value)
    }

    pub fn new_full(str: String) -> Self {
        let (namespace, value) = Self::split_string(str);
        Self::new(namespace, value)
    }

    fn split_string(str: String) -> (String, String) {
        let parts: Vec<&str> = str.split(":").collect();
        match parts.len() {
            1 => (String::from("minecraft"), String::from(parts[0])),
            2 => (String::from(parts[0]), String::from(parts[1])),
            _ => panic!("multiple : chars in identifier"),
        }
    }

    pub fn data(&self) -> (String, String) {
        (self.namespace.clone(), self.value.clone())
    }

    pub fn len(&self) -> usize {
        self.write().len()
    }

    pub fn as_string(&self) -> String {
        format!("{}:{}", self.namespace.clone(), self.value.clone())
    }
}
