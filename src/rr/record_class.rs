use bitvec::{order::Msb0, slice::BitSlice, view::BitView};

#[derive(Debug, Clone, Copy, PartialEq)]
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
        Ok(record_type)
    }
}

impl<'a> From<Class> for &'a u16 {
    fn from(val: Class) -> Self {
        let type_num = match val {
            Class::IN => &1,
            Class::CS => &2,
            Class::CH => &3,
            Class::HS => &4,
        };
        type_num
    }
}
impl From<Class> for u16 {
    fn from(val: Class) -> Self {
        let type_num: &u16 = val.into();
        *type_num
    }
}


impl<'a> Class {
    pub fn as_bitslice(self) -> &'a BitSlice<u16, Msb0> {
        let type_num: &'a u16 = self.into();
        type_num.view_bits::<Msb0>()
    }
}

#[cfg(test)]
mod tests_class {
    use super::*;

    #[test]
    fn test_from_u16() {
        let record_type: Class = (1 as u16).try_into().unwrap();
        assert_eq!(record_type, Class::IN);
    }

    #[test]
    fn test_into_u16() {
        let bitslice: u16 = Class::IN.into();
        assert_eq!(bitslice, 1);
    }

    #[test]
    fn test_all_convert() {
        for i in 1..4 {
            let record_type: Class = i.try_into().unwrap();
            let n_record_type: u16 = record_type.into();
            assert_eq!(i, n_record_type);

            let bitslice: &BitSlice<u16, Msb0> = record_type.clone().as_bitslice();
            assert_eq!(bitslice, (i as u16).view_bits::<Msb0>());
            assert_eq!(bitslice.len(), 16); // two octets
        }
    }
}
