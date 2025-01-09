use std::str::FromStr;

use bitvec::{order::Msb0, slice::BitSlice, view::BitView};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecordType {
    A = 1,     // 1 a host address
    NS,        // 2 an authoritative name server
    MD,        // 3 a mail destination (Obsolete - use MX)
    MF,        // 4 a mail forwarder (Obsolete - use MX)
    CNAME,     // 5 the canonical name for an alias
    SOA,       // 6 marks the start of a zone of authority
    MB,        // 7 a mailbox domain name (EXPERIMENTAL)
    MG,        // 8 a mail group member (EXPERIMENTAL)
    MR,        // 9 a mail rename domain name (EXPERIMENTAL)
    NULL,      //  10 a null RR (EXPERIMENTAL)
    WKS,       // 11 a well known service description
    PTR,       // 12 a domain name pointer
    HINFO,     // 13 host information
    MINFO,     // 14 mailbox or mail list information
    MX,        // 15 mail exchange
    TXT,       // 16 text strings
    AAAA = 28, // 28
}
impl FromStr for RecordType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rt = match s.to_uppercase().as_str() {
            "A" => Self::A,
            "NS" => Self::NS,
            "MD" => Self::MD,
            "MF" => Self::MF,
            "CNAME" => Self::CNAME,
            "SOA" => Self::SOA,
            "MB" => Self::MB,
            "MG" => Self::MG,
            "MR" => Self::MR,
            "NULL" => Self::NULL,
            "WKS" => Self::WKS,
            "PTR" => Self::PTR,
            "HINFO" => Self::HINFO,
            "MINFO" => Self::MINFO,
            "MX" => Self::MX,
            "TXT" => Self::TXT,
            "AAAA" => Self::AAAA,
            other => return Err(format!("{other} is not a supported as DNS record type")),
        };
        Ok(rt)
    }
}

impl RecordType {}

impl TryFrom<u16> for RecordType {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let record_type = match value {
            1 => Self::A,
            2 => Self::NS,
            3 => Self::MD,
            4 => Self::MF,
            5 => Self::CNAME,
            6 => Self::SOA,
            7 => Self::MB,
            8 => Self::MG,
            9 => Self::MR,
            10 => Self::NULL,
            11 => Self::WKS,
            12 => Self::PTR,
            13 => Self::HINFO,
            14 => Self::MINFO,
            15 => Self::MX,
            16 => Self::TXT,
            28 => Self::AAAA,
            _ => anyhow::bail!("Invalid record type number {value:b}"),
        };
        Ok(record_type)
    }
}

impl<'a> From<RecordType> for &'a u16 {
    fn from(val: RecordType) -> Self {
        let type_num = match val {
            RecordType::A => &1,
            RecordType::NS => &2,
            RecordType::MD => &3,
            RecordType::MF => &4,
            RecordType::CNAME => &5,
            RecordType::SOA => &6,
            RecordType::MB => &7,
            RecordType::MG => &8,
            RecordType::MR => &9,
            RecordType::NULL => &10,
            RecordType::WKS => &11,
            RecordType::PTR => &12,
            RecordType::HINFO => &13,
            RecordType::MINFO => &14,
            RecordType::MX => &15,
            RecordType::TXT => &16,
            RecordType::AAAA => &28,
        };
        type_num
    }
}

impl From<RecordType> for u16 {
    fn from(val: RecordType) -> Self {
        let type_num: &u16 = val.into();
        *type_num
    }
}

impl<'a> RecordType {
    pub fn as_bitslice(self) -> &'a BitSlice<u16, Msb0> {
        let type_num: &'a u16 = self.into();
        type_num.view_bits::<Msb0>()
    }
}

#[cfg(test)]
mod tests_recordtype {
    use super::*;

    #[test]
    fn test_from_u16() {
        let record_type: RecordType = (1 as u16).try_into().unwrap();
        assert_eq!(record_type, RecordType::A);
    }

    #[test]
    fn test_into_u16() {
        let bitslice: u16 = RecordType::A.into();
        assert_eq!(bitslice, 1);
    }

    #[test]
    fn test_as_bitslice() {
        let bitslice = RecordType::A.as_bitslice();
        assert_eq!(bitslice, (1 as u16).view_bits::<Msb0>());
    }

    #[test]
    fn test_all_convert() {
        for i in 1..16 {
            let record_type: RecordType = i.try_into().unwrap();
            let n_record_type: u16 = record_type.into();
            assert_eq!(i, n_record_type);

            let bitslice: &BitSlice<u16, Msb0> = record_type.as_bitslice();
            assert_eq!(bitslice, (i as u16).view_bits::<Msb0>());
            assert_eq!(bitslice.len(), 16); // two octets
        }

        let i: u16 = 28;
        let record_type: RecordType = i.try_into().unwrap();
        let n_record_type: u16 = record_type.into();
        assert_eq!(i, n_record_type);

        let bitslice: &BitSlice<u16, Msb0> = record_type.as_bitslice();
        assert_eq!(bitslice, (i as u16).view_bits::<Msb0>());
        assert_eq!(bitslice.len(), 16); // two octets
    }
}
