pub struct Long {
    pub data: i64,
}

impl Long {
    pub fn read(stream: &[u8]) -> (i64, usize) {
        let value = {
              ((stream[0] as u64 as i64) << 56)
            + ((stream[1] as u64 as i64) << 48)
            + ((stream[2] as u64 as i64) << 40)
            + ((stream[3] as u64 as i64) << 32)
            + ((stream[4] as u64 as i64) << 24)
            + ((stream[5] as u64 as i64) << 16)
            + ((stream[6] as u64 as i64) << 08)
            + ((stream[7] as u64 as i64) << 00)
        };

        (value, 8)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        let result = vec![
            ((self.data >> 56) as u8),
            ((self.data >> 48) as u8),
            ((self.data >> 40) as u8),
            ((self.data >> 32) as u8),
            ((self.data >> 24) as u8),
            ((self.data >> 16) as u8),
            ((self.data >> 08) as u8),
            ((self.data >> 00) as u8),
        ];
        return result;
    }

    pub fn new(value: i64) -> Self {
        Self {
            data: value,
        }
    }

    pub fn data(&self) -> i64 {
        self.data
    }

    pub fn len(&self) -> usize {
        8
    }
}
