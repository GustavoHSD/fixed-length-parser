pub mod error;
pub mod cnab240;

use std::collections::HashMap;
use crate::error::ParseError;

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: &'static str,
    pub start: usize,
    pub length: usize,
    pub pad_char: char,
    pub alignment: Alignment
}

#[derive(Debug, Default)]
pub struct FixedLengthParser<const N: usize>  {
    fields: Vec<Field>,
}

impl<const N: usize> FixedLengthParser<N> {
    pub fn parse(&self, input: &[u8; N]) -> HashMap<&'static str, String> {
        let mut result = HashMap::new();

        for field in self.fields.iter() {
            let value = String::from_utf8_lossy(&input[field.start..field.start + field.length]);
            result.insert(field.name, value.to_string());
        }
        
        result
    }
}

#[derive(Debug, Default)]
pub struct FixedLengthParserBuilder<const N: usize> {
    fields: Vec<Field>,
    offset: usize
}

impl<const N: usize> FixedLengthParserBuilder<N> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_field(
        mut self,
        name: &'static str,
        length: usize,
        pad_char: char,
        alignment: Alignment
    ) -> Result<Self, ParseError> {
        if self.offset + length > N {
            Err(ParseError::FieldOutOfBound { field: name, position: (self.offset, length), max: N })
        } else {
            self.fields.push(Field {
                name,
                start: self.offset,
                length,
                pad_char,
                alignment
            });
            self.offset += length;
            Ok(self)
        }
    }

    pub fn build(self) -> Result<FixedLengthParser<N>, ParseError> {
        if self.fields.len() > N {
            Err(ParseError::InvalidLength { expected: N, actual: self.fields.len() })
        } else {
            Ok(FixedLengthParser { fields: self.fields })
        }
    }
}



