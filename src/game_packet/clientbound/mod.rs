#[allow(non_snake_case)]
pub mod Status;

#[allow(non_snake_case)]
pub mod Login;

pub struct ServerResponse {
    pub id: i32,
    pub data: Vec<u8>,
}

impl ServerResponse {
    pub fn new(id: i32, data: Vec<u8>) -> Self {
        Self { id, data }
    }
}
