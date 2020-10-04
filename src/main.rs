extern crate pcsc;

mod lib;
mod connection;

use lib::{aid, capdu};
use lib::rapdu::{RAPDU, Status};
use lib::tlv::{TLV};
use std::process;

const CDOL: [u8; 66] = [0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x00, 0x00, 0x00, 0x00, 0x13, 0x37, 0x09, 0x86, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09, 0x86, 0x15, 0x04, 0x28, 0x00, 0x30, 0x90, 0x1B, 0x6A, 0x23, 0x00, 0x00, 0x1E, 0xAB, 0xC1, 0x26, 0xF8, 0x54, 0x99, 0x76, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

fn main() {
    let card = connection::connect();
    match card {
        Some(card) => {
            select_application(&card, aid::MASTERCARD_MAESTRO);
            read_pin_try_counter(&card);
            get_processing_options(&card);
            read_record(&card, 0x01, 0x1C);
        }
        None => {
            println!("No card detected!");
            process::exit(1);
        }
    }
}

fn select_application(card: &pcsc::Card, aid: [u8; 7]) {
    println!("\nC-APDU -> SELECT");
    connection::transmit(card, capdu::select(aid))
        .map(|response| {
            println!("Status: {:02X?}", response.status);
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                println!("R-APDU -> {:02X?}", read_response(card, length));
            }
        });
}

fn read_pin_try_counter(card: &pcsc::Card) {
    println!("\nC-APDU -> GET DATA");
    connection::transmit(card, capdu::get_data(0x9F, 0x17, 0x04))
        .map(|response| {
            println!("Status: {:02X?}", response.status);
            println!("R-APDU -> {:02X?}", response.data);
        });
}

fn read_record(card: &pcsc::Card, record: u8, sfi: u8) {
    println!("\nC-APDU -> READ RECORD");
    connection::transmit(card, capdu::read_record(record, sfi, 0x00))
        .map(|response| {
            if let RAPDU { status: Status::WrongLength { length }, .. } = response {
                connection::transmit(card, capdu::read_record(record, sfi, length))
                    .map(|actual_response| {
                        println!("Status: {:02X?}", actual_response.status);
                        println!("R-APDU -> RECORD {} / SFI {}: {:02X?}", record, sfi, actual_response.data);
                    });
            }
        });
}

fn verify_pin(card: &pcsc::Card, pin: Vec<u8>) {
    println!("\nC-APDU -> VERIFY");
    connection::transmit(card, capdu::verify(pin))
        .map(|response| {
            println!("Status: {:02X?}", response.status);
        });
}

fn get_processing_options(card: &pcsc::Card) {
    println!("\nC-APDU -> GET PROCESSING OPTIONS");
    connection::transmit(card, capdu::get_processing_options())
        .map(|response| {
            println!("Status: {:02X?}", response.status);
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                println!("R-APDU -> {:02X?}", read_response(card, length));
            }
        });
}

fn generate_ac(card: &pcsc::Card, cdol: Vec<u8>) {
    println!("\nC-APDU -> GENERATE AC");
    connection::transmit(card, capdu::generate_ac(cdol))
        .map(|response| {
            println!("Status: {:02X?}", response.status);
            if let RAPDU { status: Status::ResponseAvailable { length }, .. } = response {
                println!("R-APDU -> {:02X?}", read_response(card, length));
            }
        });
}

fn read_response(card: &pcsc::Card, length: u8) -> Vec<u8> {
    println!("\nC-APDU -> GET RESPONSE");
    connection::transmit(card, capdu::get_response(length))
        .map_or(
            Vec::new(),
            |response| {
                println!("Status: {:02X?}", response.status);
                if let RAPDU { status: Status::Ok, data } = response {
                    data
                } else {
                    println!("Unexpected value on READ RESPONSE: {:02X?}", response);
                    Vec::new()
                }
            })
}
