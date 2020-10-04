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
        Ok,
        Unknown,
    }

    impl Status {
        pub fn new(sw1: u8, sw2: u8) -> Status {
            match sw1 {
                0x61 => Status::ResponseAvailable { length: sw2 },
                _ => Status::check_sw2(sw1, sw2)
            }
        }

        fn check_sw2(sw1: u8, sw2: u8) -> Status {
            let combined_status = [sw1, sw2];
            match combined_status {
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
    struct APDU2 { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8 }

    #[derive(Debug)]
    struct APDU3<'data> { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: &'data [u8] }

    #[derive(Debug)]
    struct APDU4<'data> { cla: u8, ins: u8, p1: u8, p2: u8, lc: u8, data: &'data [u8], le: u8 }

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

    impl<'data> APDU4<'data> {
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
            let mut vec = [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec();
            vec.extend_from_slice(self.data);
            vec
        }
    }

    impl<'data> APDU for APDU4<'data> {
        fn to_array(&self) -> Vec<u8> {
            let mut vec = [self.cla, self.ins, self.p1, self.p2, self.lc].to_vec();
            vec.extend_from_slice(self.data);
            vec.push(self.le);
            vec
        }
    }

    pub fn select<'data>(aid: &'data [u8; 7]) -> impl APDU + 'data {
        APDU3::new(0x00, 0xA4, 0x04, 0x00, 0x07, aid)
    }

    pub fn get_response(length: u8) -> impl APDU {
        APDU2::new(0xA0, 0xC0, 0x00, 0x00, length)
    }
}
