use log::debug;
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

use super::{entry::Entry, header::MessageHeader};

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
    pub question: Vec<Entry<'a>>,
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
            question: vec![Entry::new(labels, record_type, record_class)],
        };
        Ok(ret)
    }
}

impl<'a> TryInto<BitVec<usize, Msb0>> for Message<'a> {
    type Error = std::io::Error;

    fn try_into(self) -> Result<BitVec<usize, Msb0>, Self::Error> {
        let mut bv = BitVec::<usize, Msb0>::new();

        bv.extend_from_bitslice(self.header.as_bitvec().as_bitslice());

        debug!("Serializing Questions");
        for q in self.question {
            debug!("Serializing entry {:?}", q);
            let res: Result<BitVec<usize, Msb0>, std::io::Error> = q.as_bitvec();
            match res {
                Ok(bv_entry) => {
                    bv.extend_from_bitslice(bv_entry.as_bitslice());
                }
                Err(e) => return Err(e),
            }
        }
        Ok(bv)
    }
}

impl<'a> Into<Vec<u8>> for Message<'a> {
    fn into(self) -> Vec<u8> {
        debug!("Serializing Message {:?}", self);
        let mut bv: BitVec<usize, Msb0> = self.try_into().expect("Could not serialize");
        let mut msg_bytes = Vec::with_capacity(MAX_UDP_BYTES);
        bv.read_to_end(&mut msg_bytes).unwrap();
        msg_bytes
    }
}

// // impl From<Vec<u8>> for Message {
// //     fn from(value: Vec<u8>) -> Self {
// //         // let (i, header) = nom::bits::bits(Header::deserialize)(i)?;
// //     }
// // }
