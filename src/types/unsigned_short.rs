pub struct UnsignedShort {
    data: u16,
}

impl UnsignedShort {
    pub fn read(stream: &[u8]) -> (u16, usize) {
        (((stream[0] as u16) << 8) + (stream[1] as u16), 2)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        let result = vec![
            ((self.data >> 8) as u8),
            (self.data as u8),
        ];
        return result;
    }

    pub fn new(value: u16) -> Self {
        Self {
            data: value,
        }
    }

    pub fn data(&self) -> u16 {
        self.data
    }

    pub fn len(&self) -> usize {
        2
    }
}
