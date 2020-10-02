extern crate pcsc;

mod lib;
mod aid;

use pcsc::*;
use lib::*;

fn select<'data>(aid: &'data [u8; 7]) -> impl APDU + 'data {
    APDU3::new(0x00, 0xA4, 0x04, 0x00, 0x07, aid)
}

fn get_response(length: u8) -> impl APDU {
    APDU2::new(0xA0, 0xC0, 0x00, 0x00, length)
}

fn main() {
    let card = connect();
    match card {
        Some(card) => run(card),
        None => println!("No card detected!")
    }
}

fn run(card: Card) {
    send(card, select(&aid::MASTERCARD_CREDIT))
}

fn send(card: Card, apdu: impl APDU) {
    println!("Sending APDU: {:02X?}", apdu.to_array());
    let mut status_buffer = [0; MAX_BUFFER_SIZE];
    let status = match card.transmit(&apdu.to_array(), &mut status_buffer) {
        Ok(rapdu) => lib::Status { sw1: rapdu[0], sw2: rapdu[1] },
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };

    println!("Status: {:02X?}", status);

    let mut response_buffer = [0; MAX_BUFFER_SIZE];
    let response = match card.transmit(&get_response(status.sw2).to_array(), &mut response_buffer) {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };
    println!("Response: {:02X?}", response)
}

fn connect() -> Option<Card> {
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

    // List available readers.
    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
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
    match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
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
