extern crate pcsc;

mod lib;
mod connection;

use lib::{aid, capdu};
use lib::rapdu::{RAPDU, Status};
use lib::tlv::{TLV};
use std::process;

fn main() {
    let card = connection::connect();
    match card {
        Some(card) => {
            select_application(&card, aid::MASTERCARD_CREDIT);
            read_pin_try_counter(&card);
        }
        None => {
            println!("No card detected!");
            process::exit(1);
        }
    }
}

fn select_application(card: &pcsc::Card, aid: [u8; 7]) {
    let response = connection::transmit(card, capdu::select(aid)).unwrap_or_else(|err| {
        println!("Error on SELECT application: {}", err);
        process::exit(1);
    });

    println!("Status: {:02X?}", response.status);
    let data = match response {
        RAPDU { status: Status::ResponseAvailable { length }, .. } => (read_response(card, length)),
        _ => {
            println!("Unknown status!");
            Vec::new()
        }
    };
    TLV::new(data);
}

fn read_pin_try_counter(card: &pcsc::Card) {
    let pin = connection::transmit(&card, capdu::get_data(0x9F, 0x17, 0x04)).unwrap_or_else(|err| {
        println!("Error on GET DATA: {}", err);
        process::exit(1);
    });

    TLV::new(pin.data)
        .map(|tlv| {
            println!("Status: {:02X?}", tlv);
        });
    println!("Status: {:02X?}", pin.status);
}

fn read_response(card: &pcsc::Card, length: u8) -> Vec<u8> {
    let response = connection::transmit(card, capdu::get_response(length)).unwrap_or_else(|err| {
        println!("Error trying to READ RESPONSE: {}", err);
        process::exit(1);
    });

    println!("Status: {:02X?}", response.status);
    match response {
        RAPDU { status: Status::Ok, data } => {
            println!("Response: {:02X?}", data);
            data
        }
        _ => {
            println!("Unknown status!");
            Vec::new()
        }
    }
}
