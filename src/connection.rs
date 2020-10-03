extern crate pcsc;

use pcsc::*;
use super::lib::{capdu, rapdu};
use super::lib as lib;

pub fn transmit(card: &Card, apdu: impl capdu::APDU) -> Result<rapdu::RAPDU, &'static str> {
    println!("Sending APDU: {:02X?}", apdu.to_array());

    let mut buffer = [0; MAX_BUFFER_SIZE];
    match card.transmit(&apdu.to_array(), &mut buffer) {
        Ok(rapdu) => {
            let status = rapdu::Status::new(rapdu[0], rapdu[1]);
            Ok(rapdu::RAPDU::new(status, &rapdu[2..]))
        }
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            Err("Error transmitting command")
        }
    }
}

pub fn connect() -> Option<Card> {
    // Establish a context
    let context = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    // List available readers.
    let mut readers_buf = [0; 2048];
    let mut readers = match context.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

    // Use the first reader.
    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers are connected.");
            return None;
        }
    };
    println!("Using reader: {:?}", reader);

    // Connect to the card and return it.
    match context.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => Some(card),
        Err(Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return None;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    }
}
