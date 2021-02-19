use std::path::PathBuf;

use hex::FromHex;
use structopt::StructOpt;

use crate::CryptogramType;
use crate::utils::*;

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
        cdol: Option<Vec<u8>>,
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
                application: str_to_vec_u8(parts[1])
            },
            "get_processing_options" => Command::GetProcessingOptions,
            "generate_ac" => {
                let mut cdol = None;
                if parts.len() > 2 {
                    cdol = Some(str_to_vec_u8(parts[2]));
                }
                Command::GenerateAC {
                    cryptogram_type: CryptogramType::from_str(parts[1]),
                    cdol,
                }
            }
            "get_data" => Command::GetData {
                tag: str_to_u16(parts[1])
            },
            "put_data" => Command::PutData {
                tag: str_to_u16(parts[1]),
                value: str_to_vec_u8(parts[2]),
            },
            "read_record" => Command::ReadRecord {
                record: str_to_u8(parts[1]),
                sfi: str_to_u8(parts[2]),
            },
            _ => panic!("Unknown command when parsing file")
        }
    }
}
