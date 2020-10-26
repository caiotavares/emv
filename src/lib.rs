mod tlv;
mod capdu;
mod rapdu;
mod connection;

use tlv::TLV;
use rapdu::{RAPDU, Status};
pub use connection::connect;

pub const MASTERCARD_MAESTRO: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x30, 0x60];
pub const MASTERCARD_CREDIT: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10];
pub const CDOL: [u8; 66] = [0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x09, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x86, 0x15, 0x04, 0x28, 0x00, 0x30, 0x90, 0x1B, 0x6A, 0x23, 0x00, 0x00, 0x1E, 0xAB, 0xC1, 0x26, 0xF8, 0x54, 0x99, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub fn select_application(card: &pcsc::Card, aid: [u8; 7]) {
    let apdu = capdu::select(aid);
    println!("\nC-APDU: SELECT: {:02X?}", apdu);
    connection::transmit(card, apdu)
        .map(|response| {
            println!("R-APDU: {:02X?}", response);
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                read_response(card, length)
                    .map(|rapdu| {
                        println!("R-APDU: {:02X?}", tlv::TLV::decode(rapdu.data));
                    });
            }
        });
}

pub fn read_pin_try_counter(card: &pcsc::Card) {
    let apdu = capdu::get_data(0x9F, 0x17, 0x04);
    println!("\nC-APDU: GET DATA: {:02X?}", apdu);
    connection::transmit(card, apdu)
        .map(|response| {
            println!("R-APDU: {:02X?}", response);
            println!("TLV: {:02X?}", TLV::decode(response.data));
        });
}

pub fn read_record(card: &pcsc::Card, record: u8, sfi: u8) {
    let apdu = capdu::read_record(record, sfi, 0x00);
    println!("\nC-APDU: READ RECORD {} / SFI {}: {:02X?}", record, sfi, apdu);
    connection::transmit(card, apdu)
        .map(|response| {
            if let RAPDU { status: Status::WrongLength { length }, .. } = response {
                connection::transmit(card, capdu::read_record(record, sfi, length))
                    .map(|rapdu| {
                        println!("R-APDU: {:02X?}", TLV::decode(rapdu.data));
                    });
            }
        });
}

pub fn verify_pin(card: &pcsc::Card, pin: Vec<u8>) {
    let apdu = capdu::verify(pin);
    println!("\nC-APDU: VERIFY: {:02X?}", apdu);
    connection::transmit(card, apdu)
        .map(|rapdu| {
            println!("R-APDU: {:02X?}", TLV::decode(rapdu.data));
        });
}

pub fn get_processing_options(card: &pcsc::Card) {
    let apdu = capdu::get_processing_options();
    println!("\nC-APDU: GET PROCESSING OPTIONS: {:02X?}", apdu);
    connection::transmit(card, apdu)
        .map(|response| {
            println!("R-APDU: {:02X?}", response);
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                read_response(card, length)
                    .map(|rapdu| {
                        println!("R-APDU: {:02X?}", TLV::decode(rapdu.data));
                    });
            }
        });
}

pub fn generate_ac(card: &pcsc::Card, cdol: Vec<u8>) {
    let apdu = capdu::generate_ac(cdol);
    println!("\nC-APDU: GENERATE AC: {:02X?}", apdu);
    connection::transmit(card, apdu)
        .map(|response| {
            println!("R-APDU: {:02X?}", response);
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                read_response(card, length)
                    .map(|rapdu| {
                        println!("R-APDU: {:02X?}", TLV::decode(rapdu.data));
                    });
            }
        });
}

fn read_response(card: &pcsc::Card, length: u8) -> Option<RAPDU> {
    let apdu = capdu::get_response(length);
    println!("\nC-APDU: GET RESPONSE: {:02X?}", apdu);
    connection::transmit(card, apdu).ok()
}
