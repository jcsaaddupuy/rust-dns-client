use log::debug;
use nom::IResult;
use std::io::Read;

use crate::rr::{record_class::Class, record_type::RecordType};
use bitvec::prelude::*;

/// Defined by the spec
/// UDP messages    512 octets or less
pub(crate) const MAX_UDP_BYTES: usize = 512;

/// Defined by the spec
/// labels          63 octets or less
const MAX_LABEL_BYTES: usize = 63;

/// Defined by the spec
/// names           255 octets or less
const MAX_NAME_BYTES: usize = 255;

use super::{header::MessageHeader, parser::BitInput, question::Question};

pub struct Record {
    pub name: String,
    pub class: Class,
}

#[derive(Debug, Clone)]
pub struct Message<'a> {
    /// The header section is always present.  The header includes fields that
    /// specify which of the remaining sections are present, and also specify
    /// whether the message is a query or a response, a standard query or some
    /// other opcode, etc.
    pub header: MessageHeader,
    // The question section contains fields that describe a
    // question to a name server.  These fields are a query type (QTYPE), a
    // query class (QCLASS), and a query domain name (QNAME).
    pub question: Vec<Question<'a>>,
}

impl<'a> Message<'a> {
    pub fn new(
        id: u16,
        domain_name: &'a str,
        record_type: RecordType,
        record_class: Class,
    ) -> Result<Self, anyhow::Error> {
        let name_len = domain_name.len();
        if name_len > MAX_NAME_BYTES {
            anyhow::bail!(
                "Domain name is {name_len} bytes, which is over the max of {MAX_NAME_BYTES}"
            );
        }
        let labels: Vec<&str> = domain_name.split('.').collect();
        if labels.iter().any(|label| label.len() > MAX_LABEL_BYTES) {
            anyhow::bail!(
                "One of the labels in your domain is over the max of {MAX_LABEL_BYTES} bytes"
            );
        }
        debug!("labels : {:?}", labels);

        let ret = Message {
            header: MessageHeader::new(id),
            question: vec![Question::new(labels, record_type, record_class)],
        };
        Ok(ret)
    }

    pub fn as_bitvec(self) -> Result<BitVec<usize, Msb0>, std::io::Error> {
        let mut bv = BitVec::<usize, Msb0>::new();

        bv.extend_from_bitslice(self.header.as_bitvec().as_bitslice());

        for q in self.question {
            debug!("Serializing question {:?}", q);
            match q.as_bitvec() {
                Ok(bv_question) => {
                    bv.extend_from_bitslice(bv_question.as_bitslice());
                }
                Err(e) => return Err(e),
            }
        }

        Ok(bv)
    }

    pub fn as_vec(self) -> Vec<u8> {
        debug!("Serializing Message {:?}", self);
        let mut bv: BitVec<usize, Msb0> = self.as_bitvec().expect("Could not serialize");
        let mut msg_bytes = Vec::with_capacity(MAX_UDP_BYTES);
        bv.read_to_end(&mut msg_bytes).unwrap();
        msg_bytes
    }

    pub fn deserialize(i: BitInput<'a>) -> IResult<(&'a [u8], usize), Self> {
        let i = nom::bits::bits::<
            &[u8],
            MessageHeader,
            nom::error::Error<(&[u8], usize)>,
            nom::error::Error<_>,
            _,
        >(MessageHeader::deserialize)(i.0)
        .unwrap();
        let header = i.1;

        let i = nom::bits::bits::<
            &[u8],
            Question,
            nom::error::Error<(&[u8], usize)>,
            nom::error::Error<_>,
            _,
        >(Question::deserialize)(i.0)
        .unwrap();
        let question = i.1;

        let mut questions = Vec::new();
        questions.push(question);

        return Ok((
            (i.0, i.0.len()),
            Self {
                header,
                question: questions,
            },
        ));
    }

    // pub fn deserialize_x(i: &[u16]) -> IResult<(&[u16], usize), Self> {
    //     let x = MessageHeader::deserialize_x(i).unwrap();
    //     let header = x.1;

    //     // let i = Question::deserialize(i.0).unwrap();
    //     // let question = i.1;

    //     let questions = Vec::new();
    //     // questions.push(question);

    //     return Ok((
    //         x.0,
    //         Self {
    //             header,
    //             question: questions,
    //         },
    //     ));
    // }
}
