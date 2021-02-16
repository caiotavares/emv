use structopt::StructOpt;
use std::path::PathBuf;
use hex::FromHex;

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
        cryptogram_type: u8,
        cdol: Vec<u8>,
    },
    PutData {
        tag: String,
        value: String,
    },
    GetData {
        tag: String
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
                cryptogram_type: hex::decode(parts[1]).expect("Error reading Cryptogram Type").first().cloned().expect(""),
                cdol: Vec::from_hex(parts[2]).expect("Error reading CDOL value"),
            },
            _ => panic!("Unknown command when parsing file")
        }
    }
}
