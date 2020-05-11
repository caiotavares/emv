extern crate pcsc;

use pcsc::*;

// APDUs
pub const SELECT:       &[u8] = b"\x00\xA4\x04\x00";
pub const GET_RESPONSE: &[u8] = b"\xA0\xC0\x00\x00";

// AIDs
pub const MASTERCARD_DEBIT:  &[u8] = b"\xA0\x00\x00\x00\x04\x30\x60";
pub const MASTERCARD_CREDIT: &[u8] = b"\xA0\x00\x00\x00\x04\x10\x10";

fn main() {
    let card = connect();
    match card {
        Some(c) => send_apdu(c, &[SELECT, b"\x07", MASTERCARD_CREDIT].concat()),
        None => {}
    }
}

fn send_apdu(card: Card, apdu: &[u8]) {
    // Send an APDU command.

    println!("Sending APDU: {:02X?}", apdu);
    let mut rapdu_buf = [0; MAX_BUFFER_SIZE];
    let status = match card.transmit(apdu, &mut rapdu_buf) {
        Ok(rapdu) => rapdu,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };

    println!("Status Code: {:02X?}", status);

    // TODO: Get the response length from status code and set the resp_buf to it's size

    let mut resp_buf = [0; MAX_BUFFER_SIZE];
    let response = match card.transmit(&[GET_RESPONSE, b"\x52"].concat(), &mut resp_buf) {
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
