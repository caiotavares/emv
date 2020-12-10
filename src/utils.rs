pub trait Extendable {
    fn extend(&self, value: u8) -> u16;
}

pub trait Splitable {
    fn msb(&self) -> u8;
    fn lsb(&self) -> u8;
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

