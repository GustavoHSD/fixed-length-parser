use crate::{FixedLengthParser, error::ParseError};
use super::build::build_parser;

pub fn header_arquivo() -> Result<FixedLengthParser<240>, ParseError> {
    build_parser::<240>(super::specs::header_arquivo::FIELDS)
}

pub fn trailer_arquivo() -> Result<FixedLengthParser<240>, ParseError> {
    build_parser::<240>(super::specs::trailer_arquivo::FIELDS)
}

pub fn header_lote() -> Result<FixedLengthParser<240>, ParseError> {
    build_parser::<240>(super::specs::header_lote::FIELDS)
}

pub fn trailer_lote() -> Result<FixedLengthParser<240>, ParseError> {
    build_parser::<240>(super::specs::trailer_lote::FIELDS)
}

pub fn segmento_d() -> Result<FixedLengthParser<240>, ParseError> {
    build_parser::<240>(super::specs::segmento_d::FIELDS)
}

