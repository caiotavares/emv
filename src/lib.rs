pub use connection::connect;
use rapdu::{RAPDU, Status};
use tlv::TLV;
use capdu::CryptogramType;

use crate::capdu::APDU;

mod tlv;
mod capdu;
mod rapdu;
mod connection;
mod utils;

pub const MASTERCARD_MAESTRO: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x30, 0x60];
pub const MASTERCARD_CREDIT: [u8; 7] = [0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10];

fn send(card: &pcsc::Card, apdu: APDU) {
    connection::transmit(card, &apdu)
        .map(|response| {
            match response {
                RAPDU { status: Status::ResponseAvailable { length }, .. } => {
                    read_response(card, length);
                }
                RAPDU { status: Status::WrongLengthLe { length }, .. } => {
                    connection::transmit(card, &apdu.with_length(length));
                }
                _ => {
                    response;
                }
            }
        });
}

pub fn select_application(card: &pcsc::Card, aid: Vec<u8>) {
    let apdu = capdu::select(aid);
    send(card, apdu)
}

pub fn get_data(card: &pcsc::Card, tag: u16) {
    let apdu = capdu::get_data(tag);
    send(card, apdu)
}

pub fn put_data(card: &pcsc::Card, tag: u16, data: Vec<u8>) {
    let apdu = capdu::put_data(false, tag, data);
    send(card, apdu)
}

pub fn put_data_secure(card: &pcsc::Card, tag: u16, value: Vec<u8>, mac: Vec<u8>) {
    let mut data = value.clone();
    data.extend(mac);
    let apdu = capdu::put_data(true, tag, data);
    send(card, apdu)
}

pub fn read_record(card: &pcsc::Card, record: u8, sfi: u8) {
    let apdu = capdu::read_record(record, sfi);
    send(card, apdu)
}

pub fn verify(card: &pcsc::Card, pin: Vec<u8>) {
    let apdu = capdu::verify(pin);
    send(card, apdu)
}

pub fn get_processing_options(card: &pcsc::Card) {
    let apdu = capdu::get_processing_options();
    send(card, apdu)
}

/// Generate the First AC requesting an ARQC from the ICC
pub fn generate_first_ac(card: &pcsc::Card, cdol1: Vec<u8>) {
    let apdu = capdu::generate_ac(CryptogramType::ARQC, cdol1);
    send(card, apdu)
}

pub fn external_authenticate(card: &pcsc::Card, issuer_authentication_data: Vec<u8>) {
    let apdu = capdu::external_authenticate(issuer_authentication_data);
    send(card, apdu)
}

/// Generate the Second AC approving (TC) or declining (AAC) the transaction
pub fn generate_second_ac(approve: bool, card: &pcsc::Card, cdol2: Vec<u8>) {
    let cryptogram_type = if approve { CryptogramType::TC } else { CryptogramType::AAC };
    let apdu = capdu::generate_ac(cryptogram_type, cdol2);
    send(card, apdu)
}

pub fn unblock_pin(card: &pcsc::Card, mac: Vec<u8>) {
    let apdu = capdu::reset_pin_try_counter(mac);
    send(card, apdu)
}

pub fn application_block(card: &pcsc::Card, mac: Vec<u8>) {
    let apdu = capdu::application_block(mac);
    send(card, apdu)
}

pub fn application_unblock(card: &pcsc::Card, mac: Vec<u8>) {
    let apdu = capdu::application_unblock(mac);
    send(card, apdu)
}

pub fn update_linked_application_v0(card: &pcsc::Card, aid: Vec<u8>, target_data_id: Vec<u8>, value: Vec<u8>, mac: Vec<u8>) {
    let version_number: u8 = 0x00;
    let target_application: u8 = 0xFF;
    let aid_length: u8 = aid.len() as u8;
    let mut data = [version_number, target_application, aid_length].to_vec();
    data.extend(aid);
    data.extend(target_data_id);
    data.extend(value);
    data.extend(mac);
    let apdu = capdu::put_data(true, 0xDF07, data);
    send(card, apdu)
}

pub fn update_linked_application_v1(card: &pcsc::Card, aid: Vec<u8>, id_l_v: Vec<u8>, mac: Vec<u8>) {
    let version_number: u8 = 0x01;
    let target_application: u8 = 0xFF;
    let aid_length: u8 = aid.len() as u8;
    let mut data = [version_number, target_application, aid_length].to_vec();
    data.extend(aid);
    data.extend(id_l_v);
    data.extend(mac);
    let apdu = capdu::put_data(true, 0xDF07, data);
    send(card, apdu)
}

pub fn offline_change_pin(card: &pcsc::Card, new_pin: Vec<u8>) {
    let apdu = capdu::offline_change_pin(new_pin);
    send(card, apdu)
}

fn read_response(card: &pcsc::Card, length: u8) {
    let apdu = capdu::get_response(length);
    send(card, apdu)
}
