pub mod segmento_d;
pub mod header_arquivo;
pub mod trailer_arquivo;
pub mod header_lote;
pub mod trailer_lote;

use crate::Alignment;
use super::build::FieldSpec;

pub const fn num(name: &'static str, len: usize) -> FieldSpec {
    FieldSpec { name, len, pad: '0', align: Alignment::Right }
}

pub const fn alfa(name: &'static str, len: usize) -> FieldSpec {
    FieldSpec { name, len, pad: ' ', align: Alignment::Left }
}

