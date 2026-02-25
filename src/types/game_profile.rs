use serde::Deserialize;

use crate::types;

#[derive(Clone, Deserialize)]
pub struct ProfileProperty {
    name: String,
    value: String,
    signature: Option<String>,
}

impl ProfileProperty {
    pub fn new(name: String, value: String, signature: Option<String>) -> Self {
        Self {
            name,
            value,
            signature,
        }
    }

    pub fn write(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(self.name.len() + self.value.len() + self.signature.clone().unwrap_or_default().len() + 9);
        result.append(&mut types::MCString::new(self.name.clone()).write());
        result.append(&mut types::MCString::new(self.value.clone()).write());
        if let Some(signature) = self.signature.clone() {
            result.append(&mut vec![0x01]);
            result.append(&mut types::MCString::new(signature).write());
        } else {
            result.append(&mut vec![0x00]);
        };

        return result;
    }
}

#[derive(Clone, Deserialize)]
pub struct GameProfile {
    id: String,
    name: String,
    properties: Vec<ProfileProperty>
}

impl GameProfile {
    pub fn read(stream: &[u8]) -> (types::UUID, String, Vec<ProfileProperty>, usize) {
        let mut read_head = 0;

        let (uuid, read) = types::UUID::read(stream);
        read_head += read;

        let (name, read) = types::MCString::read(&stream[read_head..]);
        read_head += read;

        let (properties_len, read) = types::VarInt::read(&stream[read_head..]);
        read_head += read;

        let mut properties = Vec::<ProfileProperty>::with_capacity(properties_len as usize);
        for _ in 0..properties_len {
            let (property_name, read) = types::MCString::read(&stream[read_head..]);
            read_head += read;

            let (property_value, read) = types::MCString::read(&stream[read_head..]);
            read_head += read;

            let (property_has_signature, read) = types::Boolean::read(stream);
            read_head += read;

            let property_signature = if property_has_signature {
                let (property_signature, read) = types::MCString::read(&stream[read_head..]);
                read_head += read;
                Some(property_signature)
            } else {
                None
            };
            properties.push(
                ProfileProperty::new(property_name, property_value, property_signature)
            );
        }
        return (types::UUID::new(uuid), name, properties, read_head)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (uuid, name, properties, _) = Self::read(stream);
        Self::new(uuid, name, properties)
    }

    pub fn write(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(self.name.len() + self.id.len() + 3); // also properties
        result.append(&mut types::UUID::new(u128::from_str_radix(&self.id, 16).unwrap()).write());
        result.append(&mut types::MCString::new(self.name.clone()).write());
        result.append(&mut types::VarInt::new(self.properties.len() as i32).write());
        for property in &self.properties {
            result.append(&mut property.write());
        }
        return result;
    }

    pub fn new(uuid: types::UUID, name: String, properties: Vec<ProfileProperty>) -> Self {
        Self {
            id: uuid.format(),
            name,
            properties,
        }
    }

    pub fn data(&self) -> (types::UUID, String, Vec<ProfileProperty>) {
        (types::UUID::new(u128::from_str_radix(&self.id, 16).unwrap()), self.name.clone(), self.properties.clone())
    }

    pub fn len(&self) -> usize {
        self.write().len()
    }
}
