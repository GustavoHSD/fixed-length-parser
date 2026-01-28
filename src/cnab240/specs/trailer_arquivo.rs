use super::{num, alfa};
use crate::cnab240::build::FieldSpec;

pub const FIELDS: &[FieldSpec] = &[
    num("banco", 3),
    num("lote", 4),
    num("tipo_registro", 1),
    alfa("cnab_reservado_1", 9),
    num("qtd_lotes", 6),
    num("qtd_registros", 6),
    num("qtd_contas_concil", 6),
    alfa("cnab_reservado_2", 205),
];

