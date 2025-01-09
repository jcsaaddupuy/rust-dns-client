use crate::rr::{record_class::Class, record_type::RecordType};
use bitvec::prelude::*;

use log::debug;
use nom::{combinator::map_res, number::complete::be_u16, IResult};

use super::parser::{take_nibble, take_u16, BitInput};

const MAX_LABEL_BYTES: usize = 64;

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

    pub fn deserialize(i: BitInput<'a>) -> IResult<(&'a [u8], usize), Self> {
        let (i, labels) = Self::parse_labels_then_zero(i).unwrap();

        let (i, record_type) = map_res(take_nibble, RecordType::try_from)(i).unwrap();
        let (i, record_qclass) = map_res(take_nibble, Class::try_from)(i).unwrap();

        Ok((
            i,
            Self {
                labels,
                record_type,
                record_qclass,
            },
        ))
    }
    pub fn parse_labels_then_zero(i: BitInput<'a>) -> IResult<(&'a [u8], usize), Vec<&'a str>> {
        let mut labels = Vec::new();
        let mut ix = i.0;
        loop {
            let (i, label) = Self::parse_label(ix).unwrap();
            ix = i;
            debug!("Found label {}", label);
            let len = label.len();
            labels.push(label);
            if len == 0 {
                return Ok(((i, i.len()), labels));
            }
        }
    }
    pub fn parse_label(i: &'a [u8]) -> IResult<&'a [u8], &'a str> {
        let parse_len = map_res(nom::number::complete::be_u8, |num| {
            if num >= 64 {
                Err(format!(
                    "DNS name labels must be <=63 bytes but this one is {num}"
                ))
            } else {
                Ok(num)
            }
        });
        let parse_label = nom::multi::length_data(parse_len);
        map_res(parse_label, |bytes: &[u8]| std::str::from_utf8(bytes))(i)
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
