use bitvec::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum ResponseCode {
    NoError,
    /// The name server was unable to interpret the query
    FormatError,
    /// The name server was unable to process this query due to a problem with the name server.
    ServerFailure,
    /// Meaningful only for
    /// responses from an authoritative name
    /// server, this code signifies that the
    /// domain name referenced in the query does
    /// not exist.
    NameError,
    /// The name server does not support the requested kind of query.
    NotImplemented,
    /// The name server refuses to
    /// perform the specified operation for
    /// policy reasons.  For example, a name
    /// server may not wish to provide the
    /// information to the particular requester,
    /// or a name server may not wish to perform
    /// a particular operation (e.g., zone
    Refused,
}
impl<'a> ResponseCode {
    pub fn as_bitvec(self) -> BitVec<usize, Msb0> {
        match self {
            ResponseCode::NoError => bitvec![usize, Msb0; 0, 0, 0, 0],
            ResponseCode::FormatError => bitvec![usize, Msb0;  0, 0, 0, 1],
            ResponseCode::ServerFailure => bitvec![usize, Msb0; 0, 0, 1, 0],
            ResponseCode::NameError => bitvec![usize, Msb0; 0, 0, 1, 1],
            ResponseCode::NotImplemented => bitvec![usize, Msb0; 0, 1, 0, 0],
            ResponseCode::Refused => bitvec![usize, Msb0; 0, 1, 0, 1],
        }
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let op: ResponseCode = match value {
            0 => Self::NoError,
            1 => Self::FormatError,
            2 => Self::ServerFailure,
            3 => Self::NameError,
            4 => Self::NotImplemented,
            5 => Self::Refused,
            other => anyhow::bail!("Unknown response_code {other}"),
        };
        Ok(op)
    }
}
impl Into<u8> for ResponseCode {
    fn into(self) -> u8 {
        match self {
            Self::NoError => 0,
            Self::FormatError => 1,
            Self::ServerFailure => 2,
            Self::NameError => 3,
            Self::NotImplemented => 4,
            Self::Refused => 5,
        }
    }
}

#[cfg(test)]
mod tests_response_code {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_response_code_into() {
        let response_code = ResponseCode::NoError;
        let bv: BitVec<usize, Msb0> = response_code.as_bitvec();
        assert_eq!(bv.len(), 4);
    }
    #[test]
    fn test_all_convert() {
        for i in 0..2 {
            let response_code: ResponseCode = i.try_into().unwrap();
            let n_response_code: u8 = response_code.into();
            assert_eq!(i, n_response_code);

            let bv: BitVec<usize, Msb0> = response_code.as_bitvec();
            assert_eq!(bv.len(), 4); // one octets
        }
    }
}
