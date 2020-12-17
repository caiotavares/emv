use crate::utils::Extendable;

#[derive(Debug)]
pub struct RAPDU {
    pub status: Status,
    pub data: Vec<u8>,
}

impl RAPDU {
    pub fn new(status: Status, data: &[u8]) -> RAPDU {
        RAPDU { status, data: Vec::from(data) }
    }
}

#[derive(Debug)]
pub enum Status {
    ResponseAvailable { length: u8 },
    WrongLength { length: u8 },
    ReferencedDataNotFound,
    ConditionsOfUseNotSatisfied,
    SecurityConditionNotSatisfied,
    InstructionCodeNotSupported,
    Ok,
    Unknown,
}

impl Status {
    pub fn new(sw1: u8, sw2: u8) -> Status {
        match sw1 {
            0x61 => Status::ResponseAvailable { length: sw2 },
            0x6C => Status::WrongLength { length: sw2 },
            _ => Status::check_sw2(sw1.extend(sw2))
        }
    }

    fn check_sw2(sw: u16) -> Status {
        match sw {
            0x9000 => Status::Ok,
            0x6A88 => Status::ReferencedDataNotFound,
            0x6982 => Status::SecurityConditionNotSatisfied,
            0x6985 => Status::ConditionsOfUseNotSatisfied,
            0x6D00 => Status::InstructionCodeNotSupported,
            _ => {
                println!("Unknown Response Status: {:02X?}", sw);
                Status::Unknown
            }
        }
    }
}
