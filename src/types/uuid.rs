pub struct UUID {
    data: u128,
}

impl UUID {
    pub fn read(stream: &[u8]) -> (u128, usize) {
        let value = { // this is horrible, but this is faster than writing generic solution
              ((stream[ 0] as u128) << 120)
            + ((stream[ 1] as u128) << 112)
            + ((stream[ 2] as u128) << 104)
            + ((stream[ 3] as u128) <<  96)
            + ((stream[ 4] as u128) <<  88)
            + ((stream[ 5] as u128) <<  80)
            + ((stream[ 6] as u128) <<  72)
            + ((stream[ 7] as u128) <<  64)
            + ((stream[ 8] as u128) <<  56)
            + ((stream[ 9] as u128) <<  48)
            + ((stream[10] as u128) <<  40)
            + ((stream[11] as u128) <<  32)
            + ((stream[12] as u128) <<  24)
            + ((stream[13] as u128) <<  16)
            + ((stream[14] as u128) <<   8)
            + ((stream[15] as u128)       )
        };

        (value, 16)
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        let result = vec![
            ((self.data >> 120) as u8),
            ((self.data >> 112) as u8),
            ((self.data >> 104) as u8),
            ((self.data >>  96) as u8),
            ((self.data >>  88) as u8),
            ((self.data >>  80) as u8),
            ((self.data >>  72) as u8),
            ((self.data >>  64) as u8),
            ((self.data >>  56) as u8),
            ((self.data >>  48) as u8),
            ((self.data >>  40) as u8),
            ((self.data >>  32) as u8),
            ((self.data >>  24) as u8),
            ((self.data >>  16) as u8),
            ((self.data >>   8) as u8),
            ((self.data       ) as u8),
        ];
        return result;
    }

    pub fn new(value: u128) -> Self {
        Self {
            data: value,
        }
    }

    pub fn data(&self) -> u128 {
        self.data
    }

    pub fn len(&self) -> usize {
        16
    }
}
