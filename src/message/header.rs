use crate::message::{
    opcode::Opcode,
    parser::{take_bit, take_nibble, take_u16},
};
use log::debug;

use bitvec::prelude::*;
use nom::IResult;

use super::{parser::BitInput, response_code::ResponseCode};

/// RFC 1035 defines DNS headers as 12 bytes long.
const EXPECTED_HEADER_SIZE: usize = 12;

#[derive(Debug, Clone, Copy)]
pub struct MessageHeader {
    /// A 16 bit identifier assigned by the program that generates any kind of query.  This identifier is copied the corresponding reply and can be used by the requester to match up replies to outstanding queries.
    pub id: u16,
    /// A one bit field that specifies whether this message is a query (0), or a response (1).
    is_query: bool,
    /// A four bit field that specifies kind of query in this message.  This value is set by the originator of a query and copied into the response.
    opcode: Opcode,
    /// This bit is valid in responses, and specifies that the responding name server is an authority for the domain name in question section. Note that the contents of the answer section may have multiple owner names because of aliases. The AA bit corresponds to the name which matches the query name, or the first owner name in the answer section.
    authoritative_answer: bool,
    /// Specifies that this message was truncated due to length greater than that permitted on the transmission channel.
    truncation: bool,
    /// This bit may be set in a query and is copied into the response.  If RD is set, it directs the name server to pursue the query recursively. Recursive query support is optional.
    recursion_desired: bool,
    /// This be (sic) is set or cleared in a response, and denotes whether recursive query support is available in the name server.
    recursion_available: bool,
    pub resp_code: ResponseCode,
    /// Number of entries in the question section.
    pub question_count: u16,
    /// Number of resource records in the answer section.
    pub answer_count: u16,
    /// Number of name server resource records in the authority records section.
    pub name_server_count: u16,
    /// Number of resource records in the additional records section.
    pub additional_records_count: u16,
}
impl MessageHeader {
    pub fn new(id: u16) -> Self {
        Self {
            id,
            is_query: false, // QR : query (0), or a response (1)
            opcode: Opcode::Query,
            authoritative_answer: false,
            truncation: false,
            recursion_desired: true,
            recursion_available: false,
            resp_code: ResponseCode::NoError, // This doesn't matter for a query
            // In a query, there will be 1 question and no records.
            question_count: 1,
            answer_count: 0,
            name_server_count: 0,
            additional_records_count: 0,
        }
    }
}

impl<'a> MessageHeader {
    pub fn as_bitvec(self) -> BitVec<usize, Msb0> {
        debug!("expected header size {}", 8 * EXPECTED_HEADER_SIZE);
        let mut bv = BitVec::<usize, Msb0>::with_capacity(8 * EXPECTED_HEADER_SIZE);

        bv.extend_from_bitslice(self.id.view_bits::<Msb0>());
        bv.push(self.is_query);
        bv.extend_from_bitslice(self.opcode.as_bitvec().as_bitslice());

        bv.push(self.authoritative_answer);
        bv.push(self.truncation);
        bv.push(self.recursion_desired);
        bv.push(self.recursion_available);
        // the Z field, reserved for future use.
        // Must be zero in all queries and responses.
        bv.extend_from_bitslice(bits![0; 3]);

        //
        bv.extend_from_bitslice(self.resp_code.as_bitvec().as_bitslice());
        //
        bv.extend_from_bitslice(self.question_count.view_bits::<Msb0>());
        bv.extend_from_bitslice(self.answer_count.view_bits::<Msb0>());
        bv.extend_from_bitslice(self.name_server_count.view_bits::<Msb0>());
        bv.extend_from_bitslice(self.additional_records_count.view_bits::<Msb0>());
        // assert_eq!(bv.len(), 8 * EXPECTED_HEADER_SIZE);
        bv
    }
}

impl MessageHeader {
    pub fn deserialize(i: BitInput) -> IResult<(&[u8], usize), Self> {
        use nom::combinator::map_res;
        let (i, id) = take_u16(i).unwrap();
        let (i, qr) = take_bit(i).unwrap();

        let (i, opcode) = map_res(take_nibble, Opcode::try_from)(i).unwrap();
        let (i, aa) = take_bit(i).unwrap();
        let (i, tc) = take_bit(i).unwrap();
        let (i, rd) = take_bit(i).unwrap();
        let (mut i, ra) = take_bit(i).unwrap();
        for _ in 0..3 {
            let z;
            (i, z) = take_bit(i).unwrap();
            assert!(!z);
        }
        let (i, rcode) = map_res(take_nibble, ResponseCode::try_from)(i).unwrap();
        let (i, qdcount) = take_u16(i).unwrap();
        let (i, ancount) = take_u16(i).unwrap();
        let (i, nscount) = take_u16(i).unwrap();
        let (i, arcount) = take_u16(i).unwrap();

        let header = MessageHeader {
            id,
            is_query: qr,
            opcode,
            authoritative_answer: aa,
            truncation: tc,
            recursion_desired: rd,
            recursion_available: ra,
            resp_code: rcode,
            question_count: qdcount,
            answer_count: ancount,
            name_server_count: nscount,
            additional_records_count: arcount,
        };
        Ok((i, header))
    }
}

impl TryFrom<Vec<u8>> for MessageHeader {
    type Error = std::io::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        // Convert the Vec<u8> into a &[u8] for parsing
        let input = &value[..];

        // Here we explicitly annotate the error type to resolve conflicts
        let result: IResult<&[u8], MessageHeader> = nom::bits(MessageHeader::deserialize)(input);

        match result {
            Ok((_, header)) => Ok(header),
            Err(_) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Parsing failed",
            )),
        }
    }
}

#[cfg(test)]
mod tests_header {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bitvec() {
        let header = MessageHeader::new(1);
        let bv = header.as_bitvec();
        assert_eq!(bv.len(), 8 * EXPECTED_HEADER_SIZE);

        let mut expected = bitvec![usize, Msb0;];
        //
        expected.extend_from_bitslice((1 as u16).view_bits::<Msb0>());
        expected.push(false); // query (0), or a response (1)
        expected.extend_from_bitslice(Opcode::Query.as_bitvec().as_bitslice());
        //ok

        expected.push(false); // authoritative_answer
        expected.push(false); // truncation
        expected.push(true); // recursive
        expected.push(false); // recursion available
                              //z
        expected.extend_from_bitslice(bits![0; 3]);
        //
        expected.extend_from_bitslice(ResponseCode::NoError.as_bitvec().as_bitslice());
        expected.extend_from_bitslice((1 as u16).view_bits::<Msb0>());
        expected.extend_from_bitslice((0 as u16).view_bits::<Msb0>());
        expected.extend_from_bitslice((0 as u16).view_bits::<Msb0>());
        expected.extend_from_bitslice((0 as u16).view_bits::<Msb0>());


        // assert_eq!(expected.len(), 8 * EXPECTED_HEADER_SIZE);
        assert_eq!(bv, expected);
    }
}
