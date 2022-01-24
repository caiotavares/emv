use std::path::PathBuf;

use hex::FromHex;
use structopt::StructOpt;

use crate::apdu::capdu::CryptogramType;
use crate::utils::extension::Hexadecimal;

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
