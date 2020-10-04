extern crate pcsc;

mod lib;
mod connection;

use lib::*;
use lib::rapdu::{RAPDU, Status};
use std::process;

fn main() {
    let card = connection::connect();
    match card {
        Some(card) => select_application(card, aid::MASTERCARD_CREDIT),
        None => {
            println!("No card detected!");
            process::exit(1);
        }
    }
}

fn select_application(card: pcsc::Card, aid: [u8; 7]) {
    let response = connection::transmit(&card, capdu::select(&aid)).unwrap_or_else(|err| {
        println!("Error trying to SELECT application: {}", err);
        process::exit(1);
    });

    println!("Status: {:02X?}", response.status);
    match response {
        RAPDU { status: Status::ResponseAvailable { length }, .. } => (read_response(card, length)),
        _ => { println!("Unknown status!") }
    }
}

fn read_response(card: pcsc::Card, length: u8) {
    let response = connection::transmit(&card, capdu::get_response(length)).unwrap_or_else(|err| {
        println!("Error trying to READ RESPONSE: {}", err);
        process::exit(1);
    });

    println!("Status: {:02X?}", response.status);
    match response {
        RAPDU { status: Status::Ok, data } => { println!("Response: {:02X?}", data); }
        _ => { println!("Unknown status!") }
    }
}
