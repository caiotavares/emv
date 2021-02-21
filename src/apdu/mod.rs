use capdu::CryptogramType;
use crate::utils::extension::Hexadecimal;

pub mod capdu;
pub mod rapdu;

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
    PinUnblock,
}

impl Command {
    pub fn from_str(str: String) -> Result<Command, String> {
        let parts: Vec<&str> = str.trim().split(' ').collect();
        let name = parts[0].to_lowercase();

        match name.as_str() {
            "select" => {
                match parts[1].to_vec_u8() {
                    Ok(application) => Ok(Command::Select { application }),
                    Err(_) => help(Command::Select { application: vec![] })
                }
            }
            "get_processing_options" => Ok(Command::GetProcessingOptions),
            "generate_ac" => {
                match parts[2].to_vec_u8() {
                    Ok(cdol) => Ok(Command::GenerateAC { cryptogram_type })
                    Err(_) => {}
                }


                Ok(Command::GenerateAC {
                    cryptogram_type: CryptogramType::from_str(parts[1]),
                    cdol,
                })
            }
            "get_data" => Ok(Command::GetData {
                tag: parts[1].to_u16()
            }),
            "put_data" => Ok(Command::PutData {
                tag: parts[1].to_u16(),
                value: parts[2].to_vec_u8(),
            }),
            "read_record" => Ok(Command::ReadRecord {
                record: parts[1].to_u8(),
                sfi: parts[2].to_u8(),
            }),
            "pin_unblock" => Ok(Command::PinUnblock),
            _ => Err(name)
        }
    }
}
