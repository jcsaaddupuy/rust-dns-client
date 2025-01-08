use std::str::FromStr;

use bitvec::{order::Msb0, slice::BitSlice, view::BitView};

pub enum RecordType {
    A,     // 1 a host address
    NS,    // 2 an authoritative name server
    MD,    // 3 a mail destination (Obsolete - use MX)
    MF,    // 4 a mail forwarder (Obsolete - use MX)
    CNAME, // 5 the canonical name for an alias
    SOA,   // 6 marks the start of a zone of authority
    MB,    // 7 a mailbox domain name (EXPERIMENTAL)
    MG,    // 8 a mail group member (EXPERIMENTAL)
    MR,    // 9 a mail rename domain name (EXPERIMENTAL)
    NULL,  //  10 a null RR (EXPERIMENTAL)
    WKS,   // 11 a well known service description
    PTR,   // 12 a domain name pointer
    HINFO, // 13 host information
    MINFO, // 14 mailbox or mail list information
    MX,    // 15 mail exchange
    TXT,   // 16 text strings
    AAAA,  // 28
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
        return Ok(record_type);
    }
}

impl<'a> Into<&'a u16> for RecordType {
    fn into(self) -> &'a u16 {
        let type_num = match self {
            Self::A => &1,
            Self::NS => &2,
            Self::MD => &3,
            Self::MF => &4,
            Self::CNAME => &5,
            Self::SOA => &6,
            Self::MB => &7,
            Self::MG => &8,
            Self::MR => &9,
            Self::NULL => &10,
            Self::WKS => &11,
            Self::PTR => &12,
            Self::HINFO => &13,
            Self::MINFO => &14,
            Self::MX => &15,
            Self::TXT => &16,
            Self::AAAA => &28,
        };
        return type_num;
    }
}

impl Into<u16> for RecordType {
    fn into(self) -> u16 {
        let type_num: &u16 = self.into();
        return *type_num;
    }
}

impl<'a> Into<&'a BitSlice<u16, Msb0>> for RecordType {
    fn into(self) -> &'a BitSlice<u16, Msb0> {
        let type_num: &'a u16 = self.into();
        return type_num.view_bits::<Msb0>();
    }
}

#[derive(Debug, PartialEq)]
pub enum Class {
    IN, //1 the Internet
    CS, //2 the CSNET class (Obsolete - used only for examples in some obsolete RFCs)
    CH, //3 the CHAOS class
    HS, //4 Hesiod [Dyer 87]
}

impl TryFrom<u16> for Class {
    type Error = anyhow::Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let record_type = match value {
            1 => Self::IN,
            2 => Self::CS,
            3 => Self::CH,
            4 => Self::HS,
            _ => anyhow::bail!("Invalid record type number {value:b}"),
        };
        return Ok(record_type);
    }
}

impl<'a> Into<&'a u16> for Class {
    fn into(self) -> &'a u16 {
        let type_num = match self {
            Self::IN => &1,
            Self::CS => &2,
            Self::CH => &3,
            Self::HS => &4,
        };
        return type_num;
    }
}
impl Into<u16> for Class {
    fn into(self) -> u16 {
        let type_num: &u16 = self.into();
        return *type_num;
    }
}

impl<'a> Into<&'a BitSlice<u16, Msb0>> for Class {
    fn into(self) -> &'a BitSlice<u16, Msb0> {
        let type_num: &'a u16 = self.into();
        return type_num.view_bits::<Msb0>();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_class_from_u16() {
        let c: u16 = 1;
        let class: Class = c.try_into().unwrap();
        assert_eq!(class, Class::IN);

        let c: u16 = 2;
        let class: Class = c.try_into().unwrap();
        assert_eq!(class, Class::CS);

        let c: u16 = 3;
        let class: Class = c.try_into().unwrap();
        assert_eq!(class, Class::CH);

        let c: u16 = 4;
        let class: Class = c.try_into().unwrap();
        assert_eq!(class, Class::HS);
    }

    #[test]
    fn test_class_into_u16() {
        let class: Class = Class::IN;
        let c: u16 = class.into();
        assert_eq!(c, 1);

        let class: Class = Class::CS;
        let c: u16 = class.into();
        assert_eq!(c, 2);

        let class: Class = Class::CH;
        let c: u16 = class.into();
        assert_eq!(c, 3);

        let class: Class = Class::HS;
        let c: u16 = class.into();
        assert_eq!(c, 4);
    }
}
