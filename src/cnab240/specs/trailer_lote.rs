use super::{num, alfa};
use crate::cnab240::build::FieldSpec;

pub const FIELDS: &[FieldSpec] = &[
    num("banco", 3),
    num("lote", 4),
    num("tipo_registro", 1),
    alfa("cnab_reservado_1", 9),

    num("qtd_registros_lote", 6),
    num("valor_total_cheques", 18),
    num("qtd_cheques_lote", 6),
    num("valor_total_juros", 18),
    num("valor_total_iof", 15),
    num("valor_total_outros_encargos", 15),

    alfa("cnab_reservado_2", 135),
    alfa("cod_ocorrencias_lote", 10),
];

