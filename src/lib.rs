pub enum Error {
    UnknownTLV
}

trait Extendable {
    fn extend(&self, value: u8) -> u16;
}

impl Extendable for u8 {
    fn extend(&self, value: u8) -> u16 {
        let left = (*self as u16) << 8;
        left | (value as u16)
    }
}

pub mod aid {
    pub const MASTERCARD_MAESTRO: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x30, 0x60];
    pub const MASTERCARD_CREDIT: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10];
}

pub mod rapdu {
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
}

pub mod capdu {
    pub trait APDU {
        fn to_array(&self) -> Vec<u8>;
    }

    #[derive(Debug)]
    struct APDU1 { cla: u8, ins: u8, p1: u8, p2: u8 }

    #[derive(Debug)]
    struct APDU2 { cla: u8, ins: u8, p1: u8, p2: u8, le: u8 }

    #[derive(Debug)]
    struct APDU3 { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8> }

    #[derive(Debug)]
    struct APDU4 { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>, le: u8 }

    impl APDU1 {
        pub fn new(cla: u8, ins: u8, p1: u8, p2: u8) -> APDU1 {
            APDU1 { cla, ins, p1, p2 }
        }
    }

    impl APDU2 {
        pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, le: u8) -> APDU2 {
            APDU2 { cla, ins, p1, p2, le }
        }
    }

    impl APDU3 {
        pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>) -> APDU3 {
            APDU3 { cla, ins, p1, p2, lc, data }
        }
    }

    impl APDU4 {
        pub fn new(cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: Vec<u8>, le: u8) -> APDU4 {
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
            [self.cla, self.ins, self.p1, self.p2, self.le].to_vec()
        }
    }

    impl APDU for APDU3 {
        fn to_array(&self) -> Vec<u8> {
            let mut vec = [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec();
            vec.extend(&self.data);
            vec
        }
    }

    impl APDU for APDU4 {
        fn to_array(&self) -> Vec<u8> {
            let mut vec = [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec();
            vec.extend(&self.data);
            vec.push(self.le);
            vec
        }
    }

    pub fn select(aid: [u8; 7]) -> impl APDU {
        APDU3::new(0x00, 0xA4, 0x04, 0x00, 0x07, aid.to_vec())
    }

    pub fn get_response(length: u8) -> impl APDU {
        APDU2::new(0xA0, 0xC0, 0x00, 0x00, length)
    }

    pub fn get_data(id1: u8, id2: u8, length: u8) -> impl APDU {
        APDU2::new(0x80, 0xCA, id1, id2, length)
    }

    pub fn get_processing_options() -> impl APDU {
        APDU3::new(0x80, 0xA8, 0x00, 0x00, 0x02, [0x83, 0x00].to_vec())
    }

    pub fn read_record(record_id: u8, sfi: u8, length: u8) -> impl APDU {
        APDU2::new(0x00, 0xB2, record_id, sfi, length)
    }

    pub fn generate_ac(cdol: Vec<u8>) -> impl APDU {
        let length = cdol.len() as u8;
        APDU3::new(0x80, 0xAE, 0x80, 0x00, length, cdol)
    }

    pub fn unblock_pin(mac: Vec<u8>) -> impl APDU {
        let length = mac.len() as u8;
        APDU3::new(0x84, 0x24, 0x00, 0x00, length, mac)
    }

    pub fn verify(pin: Vec<u8>) -> impl APDU {
        let length = pin.len() as u8;
        APDU3::new(0x00, 0x20, 0x00, 0x80, length, pin)
    }
}

pub mod tlv {
    use super::Error;

    #[derive(Debug)]
    pub enum Tag {
        TemplateFCI,
        ProprietaryTemplateFCI,
        IssuerDiscretionaryDataFCI,
        DedicatedFileName,
        ApplicationLabel,
        ApplicationPriorityIndicator,
        LanguagePreference,
        IssuerCodeTableIndex,
        ApplicationPreferredName,
        PinTryCounter,
        LogEntry,
        UnknownTag,
    }

    impl Tag {
        pub fn from(value: u16) -> Option<Tag> {
            match value {
                0x6F => Some(Tag::TemplateFCI),
                0xA5 => Some(Tag::ProprietaryTemplateFCI),
                0x84 => Some(Tag::DedicatedFileName),
                0x50 => Some(Tag::ApplicationLabel),
                0x87 => Some(Tag::ApplicationPriorityIndicator),
                0xBF0C => Some(Tag::IssuerDiscretionaryDataFCI),
                0x5F2D => Some(Tag::LanguagePreference),
                0x9F11 => Some(Tag::IssuerCodeTableIndex),
                0x9F12 => Some(Tag::ApplicationPreferredName),
                0x9F17 => Some(Tag::PinTryCounter),
                0x9F4D => Some(Tag::LogEntry),
                0x9F5D => Some(Tag::UnknownTag),
                0x9F6E => Some(Tag::UnknownTag),
                _ => None
            }
        }
    }

    #[derive(Debug)]
    pub struct TLV {
        tag: Tag,
        length: u8,
        value: Vec<u8>,
    }
}
