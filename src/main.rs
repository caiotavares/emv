use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process;

use structopt::StructOpt;

use crate::cli::interface::{Command, Emv, Mode};
use crate::connection::usb;

mod apdu;
mod cli;
mod connection;
mod tlv;
mod utils;
mod controller;

fn main() {
    let args: Emv = Emv::from_args();
    let card = usb::connect();
    match card {
        Some(card) => {
            match args.mode {
                Mode::Shell => { shell(card) }
                Mode::Run { input } => run(input, card)
            }
        }
        None => {
            eprintln!("No card detected!");
            process::exit(1);
        }
    }
}

fn shell(card: pcsc::Card) {
    cli::announcement();
    loop {
        cli::read_command()
            .map(|cmd| execute(cmd, &card));
    }
}

fn run(input: PathBuf, card: pcsc::Card) {
    let file = File::open(input).expect("File not found!");
    for line in io::BufReader::new(file).lines() {
        if let Ok(cmd) = line {
            match Command::from_str(cmd) {
                Ok(command) => execute(command, &card),
                Err(error) => eprintln!("Error parsing command {:?}", error),
            }
        }
    }
}

fn execute(command: Command, card: &pcsc::Card) {
    match command {
        Command::Select { application } => {
            controller::select_application(card, application);
        }
        Command::GetProcessingOptions => {
            controller::get_processing_options(card);
        }
        Command::GenerateAC { cryptogram_type, cdol } => {
            let cdol_value = cdol.unwrap_or_else(|| { cli::read_hex_input("Input the CDOL value: ") });
            controller::generate_ac(card, cryptogram_type, cdol_value);
        }
        Command::PutData { tag, value } => {
            controller::put_data(card, tag, value, cli::read_hex_input("Input the MAC: "));
        }
        Command::GetData { tag } => {
            controller::get_data(card, tag);
        }
        Command::ReadRecord { record, sfi } => {
            controller::read_record(&card, record, sfi);
        }
        Command::Verify { pin } => {
            controller::verify(&card, pin);
        }
        Command::PinUnblock => {
            controller::unblock_pin(&card, cli::read_hex_input("Input the MAC: "));
        }
    }
}
