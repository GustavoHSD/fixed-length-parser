use super::{num, alfa};
use crate::cnab240::build::FieldSpec;

pub const FIELDS: &[FieldSpec] = &[
    num("banco", 3),
    num("lote", 4),
    num("tipo_registro", 1),
    alfa("cnab_reservado_1", 9),
    num("tipo_inscricao_empresa", 1),
    num("num_inscricao_empresa", 14),
    alfa("convenio", 20),
    num("agencia", 5),
    alfa("agencia_dv", 1),
    num("conta", 12),
    alfa("conta_dv", 1),
    alfa("ag_conta_dv", 1),
    alfa("nome_empresa", 30),
    alfa("nome_banco", 30),
    alfa("cnab_reservado_2", 10),
    num("cod_remessa_retorno", 1),
    num("data_geracao", 8),
    num("hora_geracao", 6),
    num("num_sequencial_arquivo", 6),
    num("versao_layout_arquivo", 3),
    num("densidade_gravacao", 5),
    alfa("reservado_banco", 20),
    alfa("reservado_empresa", 20),
    alfa("cnab_reservado_3", 29),
];

