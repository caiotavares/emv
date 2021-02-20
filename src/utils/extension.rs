use hex::FromHex;

pub trait Extendable {
    fn extend(&self, value: u8) -> u16;
}

pub trait Splitable {
    fn msb(&self) -> u8;
    fn lsb(&self) -> u8;
}

pub trait Hexadecimal {
    fn to_u8(&self) -> u8;
    fn to_u16(&self) -> u16;
    fn to_vec_u8(&self) -> Vec<u8>;
}

impl Extendable for u8 {
    fn extend(&self, value: u8) -> u16 {
        let left = (*self as u16) << 8;
        left | (value as u16)
    }
}

impl Splitable for u16 {
    fn msb(&self) -> u8 {
        ((self & 0xFF00) >> 8) as u8
    }

    fn lsb(&self) -> u8 {
        (self & 0x00FF) as u8
    }
}

impl Hexadecimal for &str {
    fn to_u8(&self) -> u8 {
        let data = Vec::from_hex(self).expect("Unable to parse string to u8");
        assert_eq!(data.len(), 1);
        data[0]
    }

    fn to_u16(&self) -> u16 {
        let data = Vec::from_hex(self).expect("Unable to parse string to u16");
        assert_eq!(data.len(), 2);
        ((data[0] as u16) << 8) | (data[1] as u16)
    }

    fn to_vec_u8(&self) -> Vec<u8> {
        Vec::from_hex(self).expect("Unable parse string to Vec<u8>")
    }
}
