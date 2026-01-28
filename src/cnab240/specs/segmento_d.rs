use super::{num, alfa};
use crate::cnab240::build::FieldSpec;

pub const FIELDS: &[FieldSpec] = &[
    num("banco", 3),
    num("lote", 4),
    num("tipo_registro", 1),
    num("num_sequencial", 5),
    alfa("segmento", 1),
    alfa("cnab_reservado_1", 1),
    num("tipo_movimento", 2),
    num("cod_finalidade", 2),
    num("forma_entrada", 1),
    alfa("cmc7", 34),
    num("tipo_inscricao_emitente", 1),
    num("num_inscricao_emitente", 14),
    num("valor_cheque", 15),
    num("data_captura", 8),
    num("data_deposito", 8),
    num("data_credito", 8),
    alfa("seu_numero", 20),
    alfa("uso_banco", 15),
    num("agencia_devolucao", 5),
    num("conta_devolucao", 12),
    num("valor_juros", 11),
    num("valor_iof", 11),
    num("valor_outros_encargos", 11),
    num("num_contrato", 17),
    num("taxa_juros", 7),
    alfa("cnab_reservado_2", 13),
    alfa("cod_ocorrencias", 10),
];

