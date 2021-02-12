extern crate pcsc;
extern crate hex;

use std::process;
use std::io::{self, Read, Write};
use hex::FromHex;

fn main() {
    let card = emv::connect();
    match card {
        Some(card) => {
            emv::select_application(&card, emv::MASTERCARD_MAESTRO.to_vec());
            emv::get_processing_options(&card);
            emv::read_record(&card, 0x01, 0x1C);
            emv::generate_first_ac(&card, emv::CDOL1.to_vec());
            emv::generate_second_ac(true, &card, read_input("Input the CDOL2 value: "));
            emv::update_linked_application_v0(&card, emv::MASTERCARD_CREDIT.to_vec(), vec![0x00, 0xA5], read_input("Input the new value: "), read_input("Input the MAC: "));
        }
        None => {
            println!("No card detected!");
            process::exit(1);
        }
    }
}

fn read_input(question: &'static str) -> Vec<u8> {
    let mut buffer = String::new();
    print!("{}", question);
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    hex::decode(buffer.trim()).expect("Not a hex string")
}
