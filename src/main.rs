extern crate hex;
extern crate pcsc;

use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process;

use hex::FromHex;
use structopt::StructOpt;

use cli::{Command, Emv, Mode};

mod cli;

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
        Command::Select { application} => emv::select_application(card, application),
        Command::GetProcessingOptions => emv::get_processing_options(card),
        _ => panic!("OPS")
        // Command::GenerateAC { cryptogram_type, cdol} => emv::generate_ac(card, cryptogram_type, cdol)
        //
        // Some(card) => {
        //     emv::select_application(&card, emv::MASTERCARD_MAESTRO.to_vec());
        //     emv::get_processing_options(&card);
        //     emv::read_record(&card, 0x01, 0x1C);
        //     emv::generate_first_ac(&card, read_input("Input the CDOL1 value: "));
        //     emv::generate_second_ac(true, &card, read_input("Input the CDOL2 value: "));
        //     emv::update_linked_application_v0(&card, emv::MASTERCARD_CREDIT.to_vec(), vec![0x00, 0xA5], read_input("Input the new value: "), read_input("Input the MAC: "));
        // }
    }
}

fn shell(card: pcsc::Card) {
    println!("Shell mode")
}

fn read_input(question: &'static str) -> Vec<u8> {
    let mut buffer = String::new();
    print!("{}", question);
    io::stdout().flush();
    io::stdin().read_line(&mut buffer);
    hex::decode(buffer.trim()).expect("Not a hex string")
}
