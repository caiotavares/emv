use std::borrow::Borrow;

use crate::utils::extension::Splitable;

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

impl APDU {
    pub fn new(name: &'static str, cla: u8, ins: u8, p1: u8, p2: u8, lc: Option<u8>, data: Option<Vec<u8>>, le: Option<u8>) -> APDU {
        APDU { name, cla, ins, p1, p2, lc, data, le }
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
        APDU::new(self.name, self.cla, self.ins, self.p1, self.p2, Some(length), None, None)
    }
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

    pub fn from_str(str: &str) -> CryptogramType {
        match str {
            "AAC" => CryptogramType::AAC,
            "ARQC" => CryptogramType::ARQC,
            "TC" => CryptogramType::TC,
            _ => panic!("Unknown cryptogram type")
        }
    }
}

pub fn select(aid: Vec<u8>) -> APDU {
    let length = aid.len() as u8;
    APDU::new("SELECT", 0x00, 0xA4, 0x04, 0x00, Some(length), Some(aid), None)
}

pub fn get_response(length: u8) -> APDU {
    APDU::new("GET RESPONSE", 0x00, 0xC0, 0x00, 0x00, Some(length), None, None)
}

pub fn get_data(tag: u16) -> APDU {
    APDU::new("GET DATA", 0x80, 0xCA, tag.msb(), tag.lsb(), None, None, None)
}

pub fn put_data(secure: bool, tag: u16, data: Vec<u8>) -> APDU {
    let class = if secure { 0x84 } else { 0x80 };
    let length = data.len() as u8;
    APDU::new("PUT DATA", class, 0xDA, tag.msb(), tag.lsb(), Some(length), Some(data), None)
}

pub fn get_processing_options() -> APDU {
    let data = vec!(0x83, 0x00);
    APDU::new("GET PROCESSING OPTIONS", 0x80, 0xA8, 0x00, 0x00, Some(0x02), Some(data), None)
}

pub fn read_record(record_id: u8, sfi: u8) -> APDU {
    APDU::new("READ RECORD", 0x00, 0xB2, record_id, sfi, None, None, None)
}

pub fn external_authenticate(issuer_authentication_data: Vec<u8>) -> APDU {
    let length = issuer_authentication_data.len() as u8;
    APDU::new("EXTERNAL AUTHENTICATE", 0x00, 0x82, 0x00, 0x00, Some(length), Some(issuer_authentication_data), None)
}

pub fn generate_ac(cryptogram_type: CryptogramType, cdol_data: Vec<u8>) -> APDU {
    let length = cdol_data.len() as u8;
    let reference_control = cryptogram_type.to_reference_control();
    APDU::new("GENERATE AC", 0x80, 0xAE, reference_control, 0x00, Some(length), Some(cdol_data), None)
}

pub fn reset_pin_try_counter(mac: Vec<u8>) -> APDU {
    let length = mac.len() as u8;
    APDU::new("PIN CHANGE/UNBLOCK", 0x84, 0x24, 0x00, 0x00, Some(length), Some(mac), None)
}

pub fn offline_change_pin(new_pin: Vec<u8>) -> APDU {
    let length = new_pin.len() as u8;
    APDU::new("OFFLINE CHANGE PIN", 0x80, 0xD2, 0x00, 0x88, Some(length), Some(new_pin), None)
}

pub fn verify(pin: Vec<u8>) -> APDU {
    let length = pin.len() as u8;
    APDU::new("VERIFY", 0x00, 0x20, 0x00, 0x80, Some(length), Some(pin), None)
}

pub fn application_block(mac: Vec<u8>) -> APDU {
    APDU::new("APPLICATION BLOCK", 0x84, 0x1E, 0x00, 0x00, Some(0x08), Some(mac), None)
}

pub fn application_unblock(mac: Vec<u8>) -> APDU {
    APDU::new("APPLICATION UNBLOCK", 0x84, 0x18, 0x00, 0x00, Some(0x08), Some(mac), None)
}
