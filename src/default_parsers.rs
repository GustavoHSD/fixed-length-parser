use crate::{Alignment, FixedLengthParser, FixedLengthParserBuilder, error::ParseError};

/// Constrói o parser (240 bytes) do **CNAB240 – Segmento D (layout 01.3D)**.
///
/// Cada `add_field` abaixo respeita o layout oficial do Segmento D:
/// - **Posições** são **1-based** (como no manual).
/// - **Tamanho** = (fim - início + 1).
/// - **Tipo**: `Num` (numérico, geralmente zero à esquerda) ou `Alfa` (alfanumérico, geralmente branco à direita).
///
/// ## Mapeamento de campos (manual → código)
///
/// | Seq (layout) | Campo (manual) | Nome no parser | Posições | Tam | Tipo | Const/Obs | Cód. |
/// |---:|---|---|---:|---:|---|---|---|
/// | 01.3D | Banco (Código do Banco na Compensação) | `banco` | 1–3 | 3 | Num | - | G001 |
/// | 02.3D | Lote (Lote de Serviço) | `lote` | 4–7 | 4 | Num | - | G002 |
/// | 03.3D | Registro (Tipo de Registro) | `tipo_registro` | 8–8 | 1 | Num | **const '3'** | G003 |
/// | 04.3D | Nº Sequencial do Registro no Lote | `num_sequencial` | 9–13 | 5 | Num | - | G038 |
/// | 05.3D | Segmento (Cód. Segmento do Registro Detalhe) | `segmento` | 14–14 | 1 | Alfa | **const 'D'** | G039 |
/// | 06.3D | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_1` | 15–15 | 1 | Alfa | **brancos** | G004 |
/// | 07.3D | Tipo Movimento (Remessa/Retorno) | `tipo_movimento` | 16–17 | 2 | Num | - | K002 |
/// | 08.3D | Código da Finalidade do Movimento | `cod_finalidade` | 18–19 | 2 | Num | - | K003 |
/// | 09.3D | Forma de Entrada (Dados do Cheque) | `forma_entrada` | 20–20 | 1 | Num | - | K004 |
/// | 10.3D | CMC7 (Identificação do Cheque) | `cmc7` | 21–54 | 34 | Alfa | - | K005 |
/// | 11.3D | Tipo de Inscrição do Emitente | `tipo_inscricao_emitente` | 55–55 | 1 | Num | - | K006 |
/// | 12.3D | Número de Inscrição do Emitente | `num_inscricao_emitente` | 56–69 | 14 | Num | - | K007 |
/// | 13.3D | Valor do Cheque | `valor_cheque` | 70–84 | 15 | Num | **13,2 (centavos)** | K008 |
/// | 14.3D | Data da Captura do Cheque no Cliente | `data_captura` | 85–92 | 8 | Num | AAAAMMDD | K009 |
/// | 15.3D | Data para Depósito do Cheque | `data_deposito` | 93–100 | 8 | Num | AAAAMMDD | K010 |
/// | 16.3D | Data Prevista para Débito/Crédito | `data_credito` | 101–108 | 8 | Num | AAAAMMDD | K011 |
/// | 17.3D | Seu Número (atribuído pelo cliente) | `seu_numero` | 109–128 | 20 | Alfa | - | K012 |
/// | 18.3D | Uso exclusivo do Banco | `uso_banco` | 129–143 | 15 | Alfa | **brancos** | G021 |
/// | 19.3D | Agência para Devolução | `agencia_devolucao` | 144–148 | 5 | Num | - | K013 |
/// | 20.3D | Conta para Devolução | `conta_devolucao` | 149–160 | 12 | Num | - | K014 |
/// | 21.3D | Valor de Juros (Op Empréstimo) | `valor_juros` | 161–171 | 11 | Num | **9,2 (centavos)** | K015 |
/// | 22.3D | Valor de IOF (Op Empréstimo) | `valor_iof` | 172–182 | 11 | Num | **9,2 (centavos)** | K016 |
/// | 23.3D | Valor Outros Encargos (Op Empréstimo) | `valor_outros_encargos` | 183–193 | 11 | Num | **9,2 (centavos)** | K017 |
/// | 24.3D | Número do Contrato (Op Empréstimo) | `num_contrato` | 194–210 | 17 | Num | - | K018 |
/// | 25.3D | Taxa de Juros (Op Empréstimo) | `taxa_juros` | 211–217 | 7 | Num | **3,4** | K019 |
/// | 26.3D | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_2` | 218–230 | 13 | Alfa | **brancos** | G004 |
/// | 27.3D | Ocorrências (Códigos das Ocorrências - Detalhe) | `cod_ocorrencias` | 231–240 | 10 | Alfa | - | K020 |
///
/// ## Observações importantes
/// - Campos `Num` aqui estão configurados como **alinhamento à direita** e **preenchimento '0'**.
/// - Campos `Alfa` aqui estão configurados como **alinhamento à esquerda** e **preenchimento ' '** (branco).
/// - Campos de valores (ex.: `valor_cheque`, `valor_juros`, etc.) são representados no arquivo como **inteiro sem separador**
///   (ex.: `13,2` → 15 dígitos no total; `9,2` → 11 dígitos no total).
pub fn cnab240_segment_d_parser() -> Result<FixedLengthParser<240>, ParseError> {
    FixedLengthParserBuilder::<240>::new()
        .add_field("banco", 3, '0', Alignment::Right)?
        .add_field("lote", 4, '0', Alignment::Right)?
        .add_field("tipo_registro", 1, '0', Alignment::Right)?
        .add_field("num_sequencial", 5, '0', Alignment::Right)?
        .add_field("segmento", 1, ' ', Alignment::Left)?
        .add_field("cnab_reservado_1", 1, ' ', Alignment::Left)?
        .add_field("tipo_movimento", 2, '0', Alignment::Right)?
        .add_field("cod_finalidade", 2, '0', Alignment::Right)?
        .add_field("forma_entrada", 1, '0', Alignment::Right)?
        .add_field("cmc7", 34, ' ', Alignment::Left)?
        .add_field("tipo_inscricao_emitente", 1, '0', Alignment::Right)?
        .add_field("num_inscricao_emitente", 14, '0', Alignment::Right)?
        .add_field("valor_cheque", 15, '0', Alignment::Right)?
        .add_field("data_captura", 8, '0', Alignment::Right)?
        .add_field("data_deposito", 8, '0', Alignment::Right)?
        .add_field("data_credito", 8, '0', Alignment::Right)?
        .add_field("seu_numero", 20, ' ', Alignment::Left)?
        .add_field("uso_banco", 15, ' ', Alignment::Left)?
        .add_field("agencia_devolucao", 5, '0', Alignment::Right)?
        .add_field("conta_devolucao", 12, '0', Alignment::Right)?
        .add_field("valor_juros", 11, '0', Alignment::Right)?
        .add_field("valor_iof", 11, '0', Alignment::Right)?
        .add_field("valor_outros_encargos", 11, '0', Alignment::Right)?
        .add_field("num_contrato", 17, '0', Alignment::Right)?
        .add_field("taxa_juros", 7, '0', Alignment::Right)?
        .add_field("cnab_reservado_2", 13, ' ', Alignment::Left)?
        .add_field("cod_ocorrencias", 10, ' ', Alignment::Left)?
        .build()
}

