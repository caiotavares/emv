use std::io;
use std::io::Write;

use cli::interface::Command;

mod banner;
pub mod interface;

pub fn announcement() {
    println!("{}", banner::BANNER);
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
