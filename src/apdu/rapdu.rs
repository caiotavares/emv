use crate::utils::extension::Extendable;
use crate::tlv::parser::TLV;
use std::fmt;

#[derive(Debug)]
pub struct RAPDU {
    pub status: Status,
    pub raw: Vec<u8>,
    pub data: Vec<TLV>
}

impl RAPDU {
    pub fn new(status: Status, data: &[u8]) -> RAPDU {
        RAPDU { status, raw: Vec::from(data), data: TLV::decode(Vec::from(data)),  }
    }
}

impl fmt::Display for RAPDU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let raw_str: Vec<String> = self.raw.iter().map(|a| format!("{:02X}", a)).collect();
        let data_str: Vec<String> = self.data.iter().map(|a| format!("{}", a)).collect();

        write!(f, "R-APDU: {:?}\n  Raw: 0x{}\n  Data: [\n{}  ]\n", self.status, raw_str.join(""), data_str.join(""))
    }
}

#[derive(Debug)]
pub enum Status {
    ResponseAvailable { length: u8 },
    WrongLengthLe { length: u8 },
    WrongLength,
    ReferencedDataNotFound,
    ConditionsOfUseNotSatisfied,
    SecurityConditionNotSatisfied,
    InstructionCodeNotSupported,
    SelectedFileInvalidated,
    FileNotFound,
    Ok,
    Unknown,
}

impl Status {
    pub fn new(sw1: u8, sw2: u8) -> Status {
        match sw1 {
            0x61 => Status::ResponseAvailable { length: sw2 },
            0x6C => Status::WrongLengthLe { length: sw2 },
            _ => Status::check_sw2(sw1.extend(sw2))
        }
    }

    fn check_sw2(sw: u16) -> Status {
        match sw {
            0x6283 => Status::SelectedFileInvalidated,
            0x6700 => Status::WrongLength,
            0x6982 => Status::SecurityConditionNotSatisfied,
            0x6985 => Status::ConditionsOfUseNotSatisfied,
            0x6A82 => Status::FileNotFound,
            0x6A88 => Status::ReferencedDataNotFound,
            0x6D00 => Status::InstructionCodeNotSupported,
            0x9000 => Status::Ok,
            _ => {
                println!("Unknown Response Status: {:02X?}", sw);
                Status::Unknown
            }
        }
    }
}
