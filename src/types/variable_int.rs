static SEGMENT_BITS: i32 = 0b_0111_1111;
static CONTINUE_BIT: i32 = 0b_1000_0000;

#[derive(Clone, Copy)]
pub struct VarInt {
    data: i32,
}

impl VarInt {
    pub fn read(stream: &[u8]) -> (i32, usize) {
        let mut value: i32 = 0;
        let mut position: u64 = 0;
        let mut bytes_read: usize = 0;
        let mut current_byte: u8;

        loop {
            current_byte = stream[bytes_read];
            bytes_read += 1;

            value |= ((current_byte & (SEGMENT_BITS as u8)) as i32) << position;

            if (current_byte & (CONTINUE_BIT as u8)) == 0 {
                break;
            }

            position += 7;

            if position >= 32 {
                panic!("VarInt is too big")
            }
        }

        return (value, bytes_read);
    }

    pub fn from_stream(stream: &[u8]) -> Self {
        let (value, _) = Self::read(stream);
        Self::new(value)
    }

    pub fn write(&self) -> Vec<u8> {
        let mut result = Vec::<u8>::with_capacity(5);
        let mut value = self.data;
        let mut iteration = 0;

        loop {
            if value & !SEGMENT_BITS == 0 {
                // last bit
                result.push(value as u8);
                break;
            }

            result.push((value as u8 & SEGMENT_BITS as u8) | CONTINUE_BIT as u8);

            iteration += 1;
            if iteration >= 5 {
                panic!("VarInt ({}) resulted in more than 5 bytes", self.data)
            }

            value >>= 7;
            value &= 0x01_FF_FF_FF;
        }

        return result;
    }

    pub fn new(value: i32) -> Self {
        Self {
            data: value
        }
    }

    pub fn data(&self) -> i32 {
        self.data
    }

    pub fn len(&self) -> usize {
        self.write().len()
    }
}

#[cfg(test)]
mod variable_int_tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_read() {
        assert!((VarInt::read(&[0x00]).0                         ==  0));
        assert!((VarInt::read(&[0x01]).0                         ==  1));
        assert!((VarInt::read(&[0x02]).0                         ==  2));
        assert!((VarInt::read(&[0x7F]).0                         ==  127));
        assert!((VarInt::read(&[0x80, 0x01]).0                   ==  128));
        assert!((VarInt::read(&[0xFF, 0x01]).0                   ==  255));
        assert!((VarInt::read(&[0xDD, 0xC7, 0x01]).0             ==  25565));
        assert!((VarInt::read(&[0xFF, 0xFF, 0x7F]).0             ==  2097151));
        assert!((VarInt::read(&[0xFF, 0xFF, 0xFF, 0xFF, 0x07]).0 ==  2147483647));

        assert!((VarInt::read(&[0xFF, 0xFF, 0xFF, 0xFF, 0x0F]).0 == -1));
        assert!((VarInt::read(&[0x80, 0x80, 0x80, 0x80, 0x08]).0 == -2147483648));
    }

    #[test]
    fn test_write_positive() {
        assert!((VarInt::new(0).write()           == [0x00]), "0");
        assert!((VarInt::new(1).write()           == [0x01]), "1");
        assert!((VarInt::new(2).write()           == [0x02]), "2");
        assert!((VarInt::new(127).write()         == [0x7F]), "127");
        assert!((VarInt::new(128).write()         == [0x80, 0x01]), "128");
        assert!((VarInt::new(255).write()         == [0xFF, 0x01]), "255");
        assert!((VarInt::new(25565).write()       == [0xDD, 0xC7, 0x01]), "25565");
        assert!((VarInt::new(2097151).write()     == [0xFF, 0xFF, 0x7F]), "2097151");
        assert!((VarInt::new(2147483647).write()  == [0xFF, 0xFF, 0xFF, 0xFF, 0x07]), "2147483647");
    }

    #[test]
    fn test_write_negative() {
        assert!((VarInt::new(-1).write()          == [0xFF, 0xFF, 0xFF, 0xFF, 0x0F]), "-1");
        assert!((VarInt::new(-2147483648).write() == [0x80, 0x80, 0x80, 0x80, 0x08]), "-2147483648");
    }

    fn test_write_arbitrary(value: i32) -> bool {
        let varint_bytes = VarInt::new(value).write();
        value == VarInt::read(&varint_bytes[0..]).0
    }

    proptest!{
        #[test]
        fn test_write_arbitrary_positive(value in 0..2_147_483_647) {
            prop_assert!(
                test_write_arbitrary(value)
            );
        }

        #[test]
        fn test_write_arbitrary_negative(value in -2_147_483_648..-1) {
            prop_assert!(
                test_write_arbitrary(value)
            );
        }
    }
}
