use crate::utils::extension::Extendable;

#[derive(Debug)]
pub enum Tag {
    ApplicationCryptogram,
    ApplicationCurrencyCode,
    ApplicationEffectiveDate,
    ApplicationExpirationDate,
    ApplicationFileLocator,
    ApplicationInterchangeProfile,
    ApplicationLabel,
    ApplicationPreferredName,
    ApplicationPrimaryAccountNumber,
    ApplicationPrimaryAccountNumberSequenceNumber,
    ApplicationPriorityIndicator,
    ApplicationTransactionCounter,
    ApplicationUsageControl,
    CardholderVerificationMethodList,
    CardRiskManagementDataObjectList1,
    CardRiskManagementDataObjectList2,
    CryptogramInformationData,
    DedicatedFileName,
    EMVProprietaryTemplate,
    FileControlInformationIssuerDiscretionaryData,
    FileControlInformationProprietaryTemplate,
    FileControlInformationTemplate,
    IssuerActionCodeDefault,
    IssuerActionCodeDenial,
    IssuerActionCodeOnline,
    IssuerApplicationData,
    IssuerCodeTableIndex,
    IssuerCountryCode,
    LanguagePreference,
    LogEntry,
    PinTryCounter,
    ResponseMessageTemplateFormat2,
    StaticDataAuthenticationTagList,
    UnknownTag,
}

impl Tag {
    pub fn from_u8(value: u8) -> Option<Tag> {
        match value {
            0x50 => Some(Tag::ApplicationLabel),
            0x5A => Some(Tag::ApplicationPrimaryAccountNumber),
            0x6F => Some(Tag::FileControlInformationTemplate),
            0x77 => Some(Tag::ResponseMessageTemplateFormat2),
            0x82 => Some(Tag::ApplicationInterchangeProfile),
            0x84 => Some(Tag::DedicatedFileName),
            0x87 => Some(Tag::ApplicationPriorityIndicator),
            0x8C => Some(Tag::CardRiskManagementDataObjectList1),
            0x8D => Some(Tag::CardRiskManagementDataObjectList2),
            0x8E => Some(Tag::CardholderVerificationMethodList),
            0x94 => Some(Tag::ApplicationFileLocator),
            0xA5 => Some(Tag::FileControlInformationProprietaryTemplate),
            _ => None
        }
    }

    pub fn from_u16(value: u16) -> Option<Tag> {
        match value {
            0x5F24 => Some(Tag::ApplicationExpirationDate),
            0x5F25 => Some(Tag::ApplicationEffectiveDate),
            0x5F28 => Some(Tag::IssuerCountryCode),
            0x5F2D => Some(Tag::LanguagePreference),
            0x5F34 => Some(Tag::ApplicationPrimaryAccountNumberSequenceNumber),
            0x7081 => Some(Tag::EMVProprietaryTemplate),
            0x9F07 => Some(Tag::ApplicationUsageControl),
            0x9F0D => Some(Tag::IssuerActionCodeDefault),
            0x9F0E => Some(Tag::IssuerActionCodeDenial),
            0x9F0F => Some(Tag::IssuerActionCodeOnline),
            0x9F10 => Some(Tag::IssuerApplicationData),
            0x9F11 => Some(Tag::IssuerCodeTableIndex),
            0x9F12 => Some(Tag::ApplicationPreferredName),
            0x9F17 => Some(Tag::PinTryCounter),
            0x9F26 => Some(Tag::ApplicationCryptogram),
            0x9F27 => Some(Tag::CryptogramInformationData),
            0x9F36 => Some(Tag::ApplicationTransactionCounter),
            0x9F42 => Some(Tag::ApplicationCurrencyCode),
            0x9F4A => Some(Tag::StaticDataAuthenticationTagList),
            0x9F4D => Some(Tag::LogEntry),
            0x9F5D => Some(Tag::UnknownTag),
            0x9F6E => Some(Tag::UnknownTag),
            0xBF0C => Some(Tag::FileControlInformationIssuerDiscretionaryData),
            _ => None
        }
    }

    pub fn is_template(&self) -> bool {
        match self {
            Tag::EMVProprietaryTemplate => true,
            Tag::ResponseMessageTemplateFormat2 => true,
            Tag::FileControlInformationTemplate => true,
            Tag::FileControlInformationProprietaryTemplate => true,
            Tag::FileControlInformationIssuerDiscretionaryData => true,
            _ => false
        }
    }
}

#[derive(Debug)]
pub struct TLV {
    tag: Tag,
    length: u8,
    value: Vec<u8>,
}

impl TLV {
    pub fn parse(data: Vec<u8>) -> Result<(TLV, Vec<u8>), &'static str> {
        if data.len() < 2 {
            return Err("Not enough data to parse TLV!");
        }

        let mut iter = data.iter();
        let first_byte = iter.next().unwrap().clone();

        Tag::from_u8(first_byte)
            .or_else(|| { Tag::from_u16(first_byte.extend(iter.next().unwrap().clone())) })
            .map_or(
                Err("Unknown TLV tag!"),
                |tag| {
                    let length = iter.next().unwrap().clone();
                    let value: Vec<u8> = iter.as_slice()[0..(usize::from(length))].to_vec();
                    Ok((TLV { tag, length, value }, iter.as_slice()[(usize::from(length))..].to_vec()))
                })
    }

    pub fn decode(data: Vec<u8>) -> Vec<TLV> {
        let mut result: Vec<TLV> = Vec::new();
        let mut data = data;

        loop {
            let (tlv, remainder) = TLV::parse(data).unwrap();
            if !tlv.tag.is_template() {
                result.push(tlv);
                if remainder.len() == 0 {
                    break;
                } else { data = remainder; }
            } else { data = tlv.value; }
        }
        result
    }
}
