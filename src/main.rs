extern crate hex;
extern crate pcsc;

use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process;

use hex::FromHex;
use structopt::StructOpt;

use emv::{Command, Mode, Emv, CryptogramType};

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
            println!("No card detected!");
            process::exit(1);
        }
    }
}

fn shell(card: pcsc::Card) {
    // Read line by line and dispatch to execute_command
    println!("Shell mode")
}

fn run(input: PathBuf, card: pcsc::Card) {
    let file = File::open(input).expect("deu ruim");
    for line in io::BufReader::new(file).lines() {
        if let Ok(cmd) = line {
            let command = Command::from_str(cmd);
            execute_command(command, &card);
        }
    }
}

fn execute_command(command: Command, card: &pcsc::Card) {
    match command {
        Command::Select { application } => emv::select_application(card, application),
        Command::GetProcessingOptions => emv::get_processing_options(card),
        Command::GenerateAC { cryptogram_type, cdol } => {
            match cryptogram_type {
                CryptogramType::TC => emv::generate_ac(card, cryptogram_type, read_input("Input the CDOL2 value: ")),
                _ => emv::generate_ac(card, cryptogram_type, cdol.unwrap()),
            }
        }
        Command::PutData { tag, value } => emv::put_data_secure(card, tag, value, read_input("Input the MAC: ")),
        Command::GetData { tag } => emv::get_data(card, tag),
        Command::ReadRecord { record, sfi } => emv::read_record(&card, record, sfi),
    }
}


fn read_input(question: &'static str) -> Vec<u8> {
    let mut buffer = String::new();
    print!("{}", question);
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    hex::decode(buffer.trim()).expect("Not a hex string")
}
