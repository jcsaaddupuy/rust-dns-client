use bitvec::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// 0: a standard query (QUERY)
    Query,
    /// 1: an inverse query (IQUERY)
    InverseQuery,
    /// 2: a server status request (STATUS)
    Status,
}

impl Into<u8> for Opcode {
    fn into(self) -> u8 {
        match self {
            Opcode::Query => 0,
            Opcode::InverseQuery => 1,
            Opcode::Status => 2,
        }
    }
}

impl<'a> Opcode {
    pub fn as_bitvec(self) -> BitVec<usize, Msb0> {
        match self {
            Opcode::Query => bitvec![usize, Msb0; 0, 0, 0, 0],
            Opcode::InverseQuery => bitvec![usize, Msb0; 0, 0, 0, 1],
            Opcode::Status => bitvec![usize, Msb0; 0, 0, 1, 0],
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let op = match value {
            0 => Self::Query,
            1 => Self::InverseQuery,
            2 => Self::Status,
            other => anyhow::bail!("Unknown opcode {other}"),
        };
        Ok(op)
    }
}
#[cfg(test)]
mod tests_opcode {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bitvec() {
        let bv: BitVec = bitvec![0, 0, 0, 0];
        assert_eq!(bv.len(), 4);
    }

    #[test]
    fn test_opcode_into() {
        let opcode = Opcode::Query;
        let bv: BitVec<usize, Msb0> = opcode.as_bitvec();
        assert_eq!(bv.len(), 4);
    }

    #[test]
    fn test_all_convert() {
        for i in 0..2 {
            let opcode: Opcode = i.try_into().unwrap();
            let n_opcode: u8 = opcode.into();
            assert_eq!(i, n_opcode);

            let bv: BitVec<usize, Msb0> = opcode.as_bitvec();
            assert_eq!(bv.len(), 4); // one octets
        }
    }
}
