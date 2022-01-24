use structopt::StructOpt;

use apdu::capdu;
use apdu::capdu::{APDU, CryptogramType};
use apdu::rapdu::{RAPDU, Status};
use cli::interface::{Emv, Mode};
use connection::usb;
use tlv::parser::TLV;

pub mod apdu;
pub mod cli;
pub mod connection;
pub mod tlv;
pub mod utils;

fn send(card: &pcsc::Card, apdu: APDU) {
    usb::transmit(card, &apdu)
        .map(|response| {
            match response {
                RAPDU { status: Status::ResponseAvailable { length }, .. } => {
                    read_response(card, length);
                }
                RAPDU { status: Status::WrongLengthLe { length }, .. } => {
                    usb::transmit(card, &apdu.with_length(length));
                }
                _ => {
                    response;
                }
            }
        });
}

/// Select the provided Application ID
pub fn select_application(card: &pcsc::Card, aid: Vec<u8>) {
    let apdu = capdu::select(aid);
    send(card, apdu)
}

/// Read value from a tag
pub fn get_data(card: &pcsc::Card, tag: u16) {
    let apdu = capdu::get_data(tag);
    send(card, apdu)
}

/// **[SECURE]** Writes the provided value to a tag
pub fn put_data(card: &pcsc::Card, tag: u16, value: Vec<u8>, mac: Vec<u8>) {
    let mut data = value.clone();
    data.extend(mac);
    let apdu = capdu::put_data(true, tag, data);
    send(card, apdu)
}

/// Read information from the Record structure
pub fn read_record(card: &pcsc::Card, record: u8, sfi: u8) {
    let apdu = capdu::read_record(record, sfi);
    send(card, apdu)
}

/// Check if the provided plaintext PIN is correct
pub fn verify(card: &pcsc::Card, pin: Vec<u8>) {
    let apdu = capdu::verify(pin);
    send(card, apdu)
}

/// Get transactional data information
pub fn get_processing_options(card: &pcsc::Card) {
    let apdu = capdu::get_processing_options();
    send(card, apdu)
}

/// Requests a cryptogram from the card, used in two moments:
///
/// - `ARQC`: Before transaction processing, used to authenticate transaction data, also known as First Generate AC
/// - `TC|AAC`: After transaction processing, also known as Second Generate AC
pub fn generate_ac(card: &pcsc::Card, cryptogram_type: CryptogramType, cdol: Vec<u8>) {
    let apdu = capdu::generate_ac(cryptogram_type, cdol);
    send(card, apdu)
}

/// **[SECURE]** Resets the PIN Try Counter
pub fn unblock_pin(card: &pcsc::Card, mac: Vec<u8>) {
    let apdu = capdu::reset_pin_try_counter(mac);
    send(card, apdu)
}

/// **[SECURE]** Blocks the selected application
pub fn application_block(card: &pcsc::Card, mac: Vec<u8>) {
    let apdu = capdu::application_block(mac);
    send(card, apdu)
}

/// **[SECURE]** Unblocks the selected application
pub fn application_unblock(card: &pcsc::Card, mac: Vec<u8>) {
    let apdu = capdu::application_unblock(mac);
    send(card, apdu)
}

/// Reads the response provided by the previous successful command
fn read_response(card: &pcsc::Card, length: u8) {
    let apdu = capdu::get_response(length);
    send(card, apdu)
}
