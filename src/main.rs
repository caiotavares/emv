extern crate hex;
extern crate pcsc;

use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::PathBuf;
use std::process;

use hex::FromHex;
use structopt::StructOpt;

use emv::{Command, Mode, Emv};

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
        Command::Select { application } => emv::select_application(card, application),
        Command::GetProcessingOptions => emv::get_processing_options(card),
        Command::GenerateAC { cryptogram_type, cdol } => emv::generate_ac(card, cryptogram_type, cdol),
        Command::PutData { tag, value } => emv::put_data(card, tag, value),
        Command::GetData { tag } => emv::get_data(card, tag),
        _ => panic!("No matching command found")
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
