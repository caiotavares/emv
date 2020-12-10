use crate::utils::Splitable;

pub trait APDU {
    fn to_array(&self) -> Vec<u8>;
    fn name(&self) -> &'static str;
}

#[derive(Debug)]
pub struct APDU1 { name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8 }

#[derive(Debug)]
pub struct APDU2 { name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, le: u8 }

#[derive(Debug)]
pub struct APDU3 { name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8> }

#[derive(Debug)]
pub struct APDU4 { name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>, le: u8 }

impl APDU1 {
    pub fn new(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8) -> APDU1 {
        APDU1 { name, cla, ins, p1, p2 }
    }
}

impl APDU2 {
    pub fn new(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, le: u8) -> APDU2 {
        APDU2 { name, cla, ins, p1, p2, le }
    }
}

impl APDU3 {
    pub fn new(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>) -> APDU3 {
        APDU3 { name, cla, ins, p1, p2, lc, data }
    }
}

impl APDU4 {
    pub fn new(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>, le: u8) -> APDU4 {
        APDU4 { name, cla, ins, p1, p2, lc, data, le }
    }
}

impl APDU for APDU1 {
    fn to_array(&self) -> Vec<u8> {
        [self.cla, self.ins, self.p1, self.p2].to_vec()
    }
    fn name(&self) -> &'static str { self.name }
}

impl APDU for APDU2 {
    fn to_array(&self) -> Vec<u8> {
        [self.cla, self.ins, self.p1, self.p2, self.le].to_vec()
    }
    fn name(&self) -> &'static str { self.name }
}

impl APDU for APDU3 {
    fn to_array(&self) -> Vec<u8> {
        let mut vec = [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec();
        vec.extend(&self.data);
        vec
    }
    fn name(&self) -> &'static str { self.name }
}

impl APDU for APDU4 {
    fn to_array(&self) -> Vec<u8> {
        let mut vec = [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec();
        vec.extend(&self.data);
        vec.push(self.le);
        vec
    }
    fn name(&self) -> &'static str { self.name }
}

pub fn select(aid: [u8; 7]) -> APDU3 {
    APDU3::new("SELECT", 0x00, 0xA4, 0x04, 0x00, 0x07, aid.to_vec())
}

pub fn get_response(length: u8) -> APDU2 {
    APDU2::new("GET RESPONSE", 0xA0, 0xC0, 0x00, 0x00, length)
}

pub fn get_data(tag: u16) -> APDU1 {
    APDU1::new("GET DATA", 0x80, 0xCA, tag.msb(), tag.lsb())
}

pub fn get_data_with_length(tag: u16, length: u8) -> APDU2 {
    APDU2::new("GET DATA", 0x80, 0xCA, tag.msb(), tag.lsb(), length)
}

pub fn put_data(tag: u16, value: Vec<u8>, mac: Vec<u8>) -> APDU3 {
    let mut data = value.clone();
    data.extend(mac);
    let length = data.len() as u8;
    APDU3::new("PUT DATA", 0x84, 0xDA, tag.msb(), tag.lsb(), length, data)
}

pub fn get_processing_options() -> APDU3 {
    APDU3::new("GET PROCESSING OPTIONS", 0x80, 0xA8, 0x00, 0x00, 0x02, [0x83, 0x00].to_vec())
}

pub fn read_record(record_id: u8, sfi: u8) -> APDU1 {
    APDU1::new("READ RECORD", 0x00, 0xB2, record_id, sfi)
}

pub fn read_record_with_length(record_id: u8, sfi: u8, length: u8) -> APDU2 {
    APDU2::new("READ RECORD", 0x00, 0xB2, record_id, sfi, length)
}

pub fn generate_ac(cdol: Vec<u8>) -> APDU3 {
    let length = cdol.len() as u8;
    APDU3::new("GENERATE AC", 0x80, 0xAE, 0x80, 0x00, length, cdol)
}

pub fn reset_pin_try_counter(mac: Vec<u8>) -> APDU3 {
    let length = mac.len() as u8;
    APDU3::new("PIN CHANGE/UNBLOCK", 0x84, 0x24, 0x00, 0x00, length, mac)
}

pub fn verify(pin: Vec<u8>) -> APDU3 {
    let length = pin.len() as u8;
    APDU3::new("VERIFY", 0x00, 0x20, 0x00, 0x80, length, pin)
}
