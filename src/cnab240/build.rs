use crate::{Alignment, FixedLengthParser, FixedLengthParserBuilder, error::ParseError};

#[derive(Clone, Copy)]
pub struct FieldSpec {
    pub name: &'static str,
    pub len: usize,
    pub pad: char,
    pub align: Alignment,
}

pub fn build_parser<const N: usize>(fields: &[FieldSpec]) -> Result<FixedLengthParser<N>, ParseError> {
    let mut b = FixedLengthParserBuilder::<N>::new();
    for f in fields {
        b = b.add_field(f.name, f.len, f.pad, f.align)?;
    }
    b.build()
}

