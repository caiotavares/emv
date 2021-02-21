use std::io;
use std::io::Write;

use termion::color;

use crate::apdu::capdu::APDU;
use crate::apdu::Command;
use crate::apdu::rapdu::RAPDU;
use crate::cli::interface::Command;

mod banner;
pub mod interface;

pub fn announcement() {
    println!("{}", banner::BANNER);
}

pub fn read_input() -> Option<Command> {
    let mut buffer = String::new();
    print!("> ");
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    let input = buffer.trim();
    match input {
        "\n" => None,
        "help" | "?" => help(),
        _ => Command::from_str(String::from(input)).ok(),
    }
}

pub fn read_hex_input(question: &'static str) -> Vec<u8> {
    let mut buffer = String::new();
    print!("{}", question);
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    hex::decode(buffer.trim()).expect("Not a hex string!")
}

pub fn print_input(apdu: &RAPDU) {
    println!("{red}R-APDU: {bytes:02X?}{reset}",
             red = color::Fg(color::Red),
             bytes = apdu,
             reset = color::Fg(color::Reset));
}

pub fn print_output(apdu: &APDU) {
    println!("\n{green}C-APDU: {name}: {bytes:02X?}{reset}",
             green = color::Fg(color::Green),
             name = apdu.name,
             bytes = apdu.to_array(),
             reset = color::Fg(color::Reset));
}

fn help() -> Option<Command> {
    println!("No help yet :(");
    None
}
