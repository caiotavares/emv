extern crate pcsc;

use std::process;

fn main() {
    let card = emv::connect();
    match card {
        Some(card) => {
            emv::select_application(&card, emv::MASTERCARD_MAESTRO);
            emv::read_pin_try_counter(&card);
            emv::get_processing_options(&card);
            emv::read_record(&card, 0x01, 0x1C);
        }
        None => {
            println!("No card detected!");
            process::exit(1);
        }
    }
}
