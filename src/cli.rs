use structopt::StructOpt;
use std::path::PathBuf;

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

enum Commands {
    Select {
        application: String
    },
    GetProcessingOptions,
    ReadRecord {
        record: u8,
        sfi: u8,
    },
    GenerateAC {
        cryptogram_type: u8,
        cdol: String,
    },
    PutData {
        tag: String,
        value: String,
    },
    GetData {
        tag: String
    },
}
