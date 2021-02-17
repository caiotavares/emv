use structopt::StructOpt;
use std::path::PathBuf;
use hex::FromHex;
use crate::CryptogramType;

#[derive(StructOpt)]
pub struct Emv {
    #[structopt(subcommand)]
    pub mode: Mode,
}

#[derive(StructOpt)]
pub enum Mode {
    Shell,
    Run {
        #[structopt(parse(from_os_str))]
        input: PathBuf
    },
}

#[derive(Debug)]
pub enum Command {
    Select {
        application: Vec<u8>
    },
    GetProcessingOptions,
    ReadRecord {
        record: u8,
        sfi: u8,
    },
    GenerateAC {
        cryptogram_type: CryptogramType,
        cdol: Vec<u8>,
    },
    PutData {
        tag: u16,
        value: Vec<u8>,
    },
    GetData {
        tag: u16
    },
}

impl Command {
    pub fn from_str(str: String) -> Command {
        let parts: Vec<&str> = str.split(' ').collect();
        let name = parts[0];

        match name {
            "select" => Command::Select {
                application: Vec::from_hex(parts[1]).expect("Error reading Application")
            },
            "get_processing_options" => Command::GetProcessingOptions,
            "generate_ac" => Command::GenerateAC {
                cryptogram_type: CryptogramType::from_str(parts[1]),
                cdol: Vec::from_hex(parts[2]).expect("Error reading CDOL value"),
            },
            // "put_data" => Command::PutData { tag: Vec::from_hex(parts[1]).expect("banana") }
            _ => panic!("Unknown command when parsing file")
        }
    }
}
