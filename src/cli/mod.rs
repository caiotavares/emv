use std::io;
use std::io::Write;

use termion::color;

use crate::apdu::capdu::APDU;
use crate::apdu::Command;
use crate::apdu::rapdu::RAPDU;

mod banner;
pub mod interface;

const INPUT_COLOR: color::Rgb = color::Rgb(245, 181, 71);
const OUTPUT_COLOR: color::Rgb = color::Rgb(77, 128, 247);

pub fn announcement() {
    println!("{}", banner::BANNER);
}

pub fn print_input(apdu: &RAPDU) {
    println!("{red}R-APDU: {bytes:02X?}{reset}",
             red = color::Fg(INPUT_COLOR),
             bytes = apdu,
             reset = color::Fg(color::Reset));
}

pub fn print_output(apdu: &APDU) {
    println!("\n{green}C-APDU: {name}: {bytes:02X?}{reset}",
             green = color::Fg(OUTPUT_COLOR),
             name = apdu.name,
             bytes = apdu.to_array(),
             reset = color::Fg(color::Reset));
}

pub fn read_command() -> Option<Command> {
    let mut buffer = String::new();
    print!("> ");
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    let input = buffer.trim();
    match input {
        "\n" => None,
        _ => Command::from_str(String::from(input)).ok(),
    }
}

pub fn read_hex_input(question: &'static str) -> Vec<u8> {
    let mut buffer = String::new();
    print!("{}", question);
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    hex::decode(buffer.trim()).expect("Not a hex string")
}
