pub struct Byte {
    data: i8
}

impl Byte {
    pub fn read(stream: &[u8]) -> (i8, usize) {
        (stream[0] as i8, 1)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        vec![self.data as u8]
    }

    pub fn new(value: i8) -> Self {
        Self {
            data: value
        }
    }

    pub fn data(&self) -> i8 {
        self.data
    }

    pub fn len(&self) -> usize {
        1
    }
}
