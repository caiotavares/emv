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
    Ok,
    Unknown,
}

impl Status {
    pub fn new(sw1: u8, sw2: u8) -> Status {
        match sw1 {
            0x61 => Status::ResponseAvailable { length: sw2 },
            0x6C => Status::WrongLength { length: sw2 },
            _ => Status::check_sw2(sw1, sw2)
        }
    }

    fn check_sw2(sw1: u8, sw2: u8) -> Status {
        match [sw1, sw2] {
            [0x90, 0x00] => Status::Ok,
            _ => Status::Unknown
        }
    }
}
