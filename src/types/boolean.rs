pub struct Boolean {
    data: bool
}

impl Boolean {
    pub fn read(stream: &[u8]) -> (bool, usize) {
        (stream[0] != 0, 1)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        match self.data {
            true  => vec![1],
            false => vec![0],
        }
    }

    pub fn new(value: bool) -> Self {
        Self {
            data: value
        }
    }

    pub fn data(&self) -> bool {
        self.data
    }

    pub fn len(&self) -> usize {
        1
    }
}
