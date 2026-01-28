use super::{num, alfa};
use crate::cnab240::build::FieldSpec;

pub const FIELDS: &[FieldSpec] = &[
    num("banco", 3),
    num("lote", 4),
    num("tipo_registro", 1),

    alfa("operacao", 1),
    num("tipo_servico", 2),
    alfa("cnab_reservado_1", 2),
    num("versao_layout_lote", 3),
    alfa("cnab_reservado_2", 1),

    num("tipo_inscricao_empresa", 1),
    num("num_inscricao_empresa", 14),
    alfa("convenio", 20),

    num("agencia", 5),
    alfa("agencia_dv", 1),
    num("conta", 12),
    alfa("conta_dv", 1),
    alfa("ag_conta_dv", 1),

    alfa("nome_empresa", 30),
    alfa("uso_banco", 20),
    alfa("cnab_reservado_3", 108),
    alfa("cod_ocorrencias_lote", 10),
];

