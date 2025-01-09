use crate::rr::{record_class::Class, record_type::RecordType};
use bitvec::prelude::*;

use log::debug;

/// RFC 1035 defines DNS headers as 12 bytes long.
const MAX_LABEL_BYTES: usize = 12;

#[derive(Debug, Clone)]
pub struct Question<'a> {
    labels: Vec<&'a str>,
    record_type: RecordType,
    record_qclass: Class,
}
impl<'a> Question<'a> {
    pub(crate) fn new(labels: Vec<&'a str>, record_type: RecordType, record_qclass: Class) -> Self {
        Self {
            labels,
            record_type,
            record_qclass,
        }
    }
}

impl<'a> Question<'a> {
    pub fn as_bitvec(self) -> Result<BitVec<usize, Msb0>, std::io::Error> {
        let mut bv: BitVec<usize, Msb0> = BitVec::<usize, Msb0>::new();

        for label in self.labels {
            debug!("Serializing label {:?}", label);
            // The mapping of domain names to labels is defined in RFC 1035:
            // 2.3.1. Preferred name syntax
            let len = label.len();
            if len >= MAX_LABEL_BYTES {
                let fmt = format!("Label {label} too long");
                return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, fmt));
            }
            let len = u8::try_from(len).unwrap();
            bv.extend_from_bitslice(len.view_bits::<Msb0>());
            label
                .chars()
                .map(|ch| ch.try_into().unwrap())
                .for_each(|byte: u8| bv.extend_from_bitslice(byte.view_bits::<Msb0>()));
        }

        debug!("Serializing record type {:?}", self.record_type);
        bv.extend_from_bitslice(self.record_type.as_bitslice());
        debug!("Serializing record class {:?}", self.record_qclass);
        bv.extend_from_bitslice(self.record_qclass.as_bitslice());

        Ok(bv)
    }
}

#[cfg(test)]
mod tests_question {
    use super::*;

    #[test]
    fn test_all_convert() {
        let record_type = RecordType::A;
        let record_class = Class::IN;

        let question = Question::new(vec!["google", "com"], record_type, record_class);

        let bitvec: BitVec<usize, Msb0> = question.as_bitvec().unwrap();
        let mut expected = bitvec![usize, Msb0;];

        //
        expected.extend_from_bitslice(("google".len() as u8).view_bits::<Msb0>());
        expected.extend_from_bitslice(('g' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('o' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('o' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('g' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('l' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('e' as u8).view_bits::<Msb0>()); 

        //
        expected.extend_from_bitslice(("com".len() as u8).view_bits::<Msb0>());
        expected.extend_from_bitslice(('c' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('o' as u8).view_bits::<Msb0>()); 
        expected.extend_from_bitslice(('m' as u8).view_bits::<Msb0>()); 

        //
        expected.extend_from_bitslice((1 as u16).view_bits::<Msb0>()); 
        expected.extend_from_bitslice((1 as u16).view_bits::<Msb0>()); 

        assert_eq!(bitvec, expected);
    }
}
