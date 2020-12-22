use crate::utils::Splitable;
use std::borrow::Borrow;

#[derive(Debug)]
pub struct APDU {
    pub name: &'static str,
    cla: u8,
    ins: u8,
    p1: u8,
    p2: u8,
    lc: Option<u8>,
    data: Option<Vec<u8>>,
    le: Option<u8>,
}

#[derive(Debug)]
pub enum CryptogramType {
    AAC,
    ARQC,
    TC,
}

impl CryptogramType {
    pub fn to_reference_control(&self) -> u8 {
        match self {
            CryptogramType::AAC => 0x00,
            CryptogramType::ARQC => 0x80,
            CryptogramType::TC => 0x40
        }
    }
}

impl APDU {
    pub fn new1(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8) -> APDU {
        APDU { name, cla, ins, p1, p2, lc: None, data: None, le: None }
    }

    pub fn new2(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8) -> APDU {
        APDU { name, cla, ins, p1, p2, lc: Some(lc), data: None, le: None }
    }

    pub fn new3(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>) -> APDU {
        APDU { name, cla, ins, p1, p2, lc: Some(lc), data: Some(data), le: None }
    }

    pub fn new4(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>, le: u8) -> APDU {
        APDU { name, cla, ins, p1, p2, lc: Some(lc), data: Some(data), le: Some(le) }
    }

    pub fn to_array(&self) -> Vec<u8> {
        let mut data = [self.cla, self.ins, self.p1, self.p2].to_vec();
        self.lc.map(|value| { data.extend(&[value]) });
        if self.data.is_some() {
            let mut other = self.data.clone().unwrap();
            data.append(&mut other)
        }
        self.le.map(|value| { data.extend(&[value]) });
        data
    }

    pub fn with_length(&self, length: u8) -> APDU {
        APDU::new2(self.name, self.cla, self.ins, self.p1, self.p2, length)
    }
}

pub fn select(aid: Vec<u8>) -> APDU {
    let length = aid.len() as u8;
    APDU::new3("SELECT", 0x00, 0xA4, 0x04, 0x00, length, aid)
}

pub fn get_response(length: u8) -> APDU {
    APDU::new2("GET RESPONSE", 0xA0, 0xC0, 0x00, 0x00, length)
}

pub fn get_data(tag: u16) -> APDU {
    APDU::new1("GET DATA", 0x80, 0xCA, tag.msb(), tag.lsb())
}

pub fn put_data(secure: bool, tag: u16, data: Vec<u8>) -> APDU {
    let class = if secure { 0x84 } else { 0x80 };
    let length = data.len() as u8;
    APDU::new3("PUT DATA", class, 0xDA, tag.msb(), tag.lsb(), length, data)
}

pub fn get_processing_options() -> APDU {
    APDU::new3("GET PROCESSING OPTIONS", 0x80, 0xA8, 0x00, 0x00, 0x02, [0x83, 0x00].to_vec())
}

pub fn read_record(record_id: u8, sfi: u8) -> APDU {
    APDU::new1("READ RECORD", 0x00, 0xB2, record_id, sfi)
}

pub fn external_authenticate(issuer_authentication_data: Vec<u8>) -> APDU {
    let length = issuer_authentication_data.len() as u8;
    APDU::new3("EXTERNAL AUTHENTICATE", 0x00, 0x82, 0x00, 0x00, length, issuer_authentication_data)
}

pub fn generate_ac(cryptogram_type: CryptogramType, cdol_data: Vec<u8>) -> APDU {
    let length = cdol_data.len() as u8;
    let reference_control = cryptogram_type.to_reference_control();
    APDU::new3("GENERATE AC", 0x80, 0xAE, reference_control, 0x00, length, cdol_data)
}

pub fn reset_pin_try_counter(mac: Vec<u8>) -> APDU {
    let length = mac.len() as u8;
    APDU::new3("PIN CHANGE/UNBLOCK", 0x84, 0x24, 0x00, 0x00, length, mac)
}

pub fn offline_change_pin(new_pin: Vec<u8>) -> APDU {
    let length = new_pin.len() as u8;
    APDU::new3("OFFLINE CHANGE PIN", 0x80, 0xD2, 0x00, 0x88, length, new_pin)
}

pub fn verify(pin: Vec<u8>) -> APDU {
    let length = pin.len() as u8;
    APDU::new3("VERIFY", 0x00, 0x20, 0x00, 0x80, length, pin)
}
