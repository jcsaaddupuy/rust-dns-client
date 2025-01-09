use nom::{bits::complete::take, IResult};

pub type BitInput<'a> = (&'a [u8], usize);

/// Take 4 bits from the BitInput.
/// Parse into a uint with most significant bit first.
/// Add 0000 as padding to the most significant bits to the output number to make it
/// fit into a u8.
pub fn take_nibble(i: BitInput) -> IResult<BitInput, u8> {
    take(4u8)(i)
}

/// Take 16 bits from the BitInput, parse into a uint with most significant bit first..
pub fn take_u16(i: BitInput) -> IResult<BitInput, u16> {
    take(16u8)(i)
}

/// Takes one bit from the BitInput.
pub fn take_bit(i: BitInput) -> IResult<BitInput, bool> {
    let (i, bit): (BitInput, u8) = take(1u8)(i)?;
    Ok((i, bit != 0))
}
