mod tlv;
mod capdu;
mod rapdu;
mod connection;
mod utils;

use tlv::TLV;
use rapdu::{RAPDU, Status};
pub use connection::connect;

pub const MASTERCARD_MAESTRO: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x30, 0x60];
pub const MASTERCARD_CREDIT: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10];
pub const CDOL: [u8; 66] = [0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x09, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x86, 0x15, 0x04, 0x28, 0x00, 0x30, 0x90, 0x1B, 0x6A, 0x23, 0x00, 0x00, 0x1E, 0xAB, 0xC1, 0x26, 0xF8, 0x54, 0x99, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

pub fn select_application(card: &pcsc::Card, aid: [u8; 7]) {
    let apdu = capdu::select(aid);
    connection::transmit(card, apdu)
        .map(|response| {
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                read_response(card, length);
            }
        });
}

pub fn get_data(card: &pcsc::Card, tag: u16) {
    let apdu = capdu::get_data(tag);
    connection::transmit(card, apdu)
        .map(|response| {
            if let RAPDU { status: Status::WrongLength { length }, .. } = response {
                connection::transmit(card, capdu::get_data_with_length(tag, length));
            }
        });
}

pub fn read_record(card: &pcsc::Card, record: u8, sfi: u8) {
    let apdu = capdu::read_record(record, sfi);
    connection::transmit(card, apdu)
        .map(|response| {
            if let RAPDU { status: Status::WrongLength { length }, .. } = response {
                connection::transmit(card, capdu::read_record_with_length(record, sfi, length));
            }
        });
}

pub fn verify(card: &pcsc::Card, pin: Vec<u8>) {
    let apdu = capdu::verify(pin);
    connection::transmit(card, apdu);
}

pub fn get_processing_options(card: &pcsc::Card) {
    let apdu = capdu::get_processing_options();
    connection::transmit(card, apdu)
        .map(|response| {
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                read_response(card, length);
            }
        });
}

pub fn generate_ac(card: &pcsc::Card, cdol: Vec<u8>) {
    let apdu = capdu::generate_ac(cdol);
    connection::transmit(card, apdu)
        .map(|response| {
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                read_response(card, length);
            }
        });
}

fn read_response(card: &pcsc::Card, length: u8) -> Option<RAPDU> {
    let apdu = capdu::get_response(length);
    connection::transmit(card, apdu).ok()
}
