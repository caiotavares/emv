extern crate hex;
extern crate pcsc;

use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process;

use hex::FromHex;
use structopt::StructOpt;

use emv::cli::{Command, Emv, Mode};
use emv::{cli, CryptogramType};

fn main() {
    emv::announcement();
    let card = emv::connect();
    match card {
        Some(card) => {
            let args: Emv = Emv::from_args();
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
    loop {
        cli::read_command()
            .map(|cmd| execute_command(cmd, &card));
    }
}

fn run(input: PathBuf, card: pcsc::Card) {
    let file = File::open(input).expect("File not found!");
    for line in io::BufReader::new(file).lines() {
        if let Ok(cmd) = line {
            match Command::from_str(cmd) {
                Ok(command) => execute_command(command, &card),
                Err(error) => eprintln!("Error interpreting command. Error {:?}", error),
            }
        }
    }
}

fn execute_command(command: Command, card: &pcsc::Card) {
    match command {
        Command::Select { application } => emv::select_application(card, application),
        Command::GetProcessingOptions => emv::get_processing_options(card),
        Command::GenerateAC { cryptogram_type, cdol } => {
            match cryptogram_type {
                CryptogramType::TC => emv::generate_ac(card, cryptogram_type, cli::read_hex_input("Input the CDOL2 value: ")),
                _ => emv::generate_ac(card, cryptogram_type, cdol.unwrap()),
            }
        }
        Command::PutData { tag, value } => emv::put_data_secure(card, tag, value, cli::read_hex_input("Input the MAC: ")),
        Command::GetData { tag } => emv::get_data(card, tag),
        Command::ReadRecord { record, sfi } => emv::read_record(&card, record, sfi),
    }
}


