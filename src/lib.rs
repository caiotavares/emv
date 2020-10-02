#[derive(Debug)]
pub struct Status { pub sw1: u8, pub sw2: u8 }

pub trait APDU {
    fn to_array(&self) -> Vec<u8>;
}

#[derive(Debug)]
pub struct APDU1 { cla: u8, ins: u8, p1: u8, p2: u8 }

#[derive(Debug)]
pub struct APDU2 { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8 }

#[derive(Debug)]
pub struct APDU3<'data> { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: &'data [u8] }

#[derive(Debug)]
pub struct APDU4<'a> { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: &'a [u8], le: u8 }

impl APDU1 {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8) -> APDU1 {
        APDU1 { cla, ins, p1, p2 }
    }
}

impl APDU2 {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8) -> APDU2 {
        APDU2 { cla, ins, p1, p2, lc }
    }
}

impl<'data> APDU3<'data> {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: &[u8]) -> APDU3 {
        APDU3 { cla, ins, p1, p2, lc, data }
    }
}

impl<'a> APDU4<'a> {
    pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: &[u8], le: u8) -> APDU4 {
        APDU4 { cla, ins, p1, p2, lc, data, le }
    }
}

impl APDU for APDU1 {
    fn to_array(&self) -> Vec<u8> {
        [self.cla, self.ins, self.p1, self.p2].to_vec()
    }
}

impl APDU for APDU2 {
    fn to_array(&self) -> Vec<u8> {
        [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec()
    }
}

impl<'data> APDU for APDU3<'data> {
    fn to_array(&self) -> Vec<u8> {
        [self.cla, self.ins, self.p1, self.p2, self.lc, 0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10].to_vec()
    }
}

impl<'a> APDU for APDU4<'a> {
    fn to_array(&self) -> Vec<u8> {
        [self.cla, self.ins, self.p1, self.p2, self.lc, self.le].to_vec()
    }
}
