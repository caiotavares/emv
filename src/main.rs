extern crate pcsc;

use std::process;

fn main() {
    let card = emv::connect();
    match card {
        Some(card) => {
            emv::select_application(&card, emv::MASTERCARD_MAESTRO);
            emv::get_data(&card, 0xDF07);
        }
        None => {
            println!("No card detected!");
            process::exit(1);
        }
    }
}
