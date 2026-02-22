pub struct UnsignedByte {
    data: u8
}

impl UnsignedByte {
    pub fn read(stream: &[u8]) -> (u8, usize) {
        (stream[0] as u8, 1)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        vec![self.data]
    }

    pub fn new(value: u8) -> Self {
        Self {
            data: value
        }
    }

    pub fn data(&self) -> u8 {
        self.data
    }

    pub fn len(&self) -> usize {
        1
    }
}
