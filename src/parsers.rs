
/// Modulo de construção dos parsers de custodia para o Layout Padrão Febraban 240 posições V10.9
pub mod custodia_de_cheques {
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

    /// Constrói o parser (240 bytes) do **CNAB240 – Header de Lote (Registro 1, layout 01.1)**.
    ///
    /// Este parser segue o layout informado (01.1 … 20.1):
    /// - **Posições** do manual são **1-based**.
    /// - Campos `Num` → alinhamento à direita, preenchimento `'0'`.
    /// - Campos `Alfa` → alinhamento à esquerda, preenchimento `' '`.
    ///
    /// ## Mapeamento de campos (manual → código)
    ///
    /// | Seq (layout) | Campo (manual) | Nome no parser | Posições | Tam | Tipo | Const/Obs | Cód. |
    /// |---:|---|---|---:|---:|---|---|---|
    /// | 01.1 | Banco (Código do Banco na Compensação) | `banco` | 1–3 | 3 | Num | - | G001 |
    /// | 02.1 | Lote (Lote de Serviço) | `lote` | 4–7 | 4 | Num | - | G002 |
    /// | 03.1 | Registro (Tipo de Registro) | `tipo_registro` | 8–8 | 1 | Num | **const '1'** | G003 |
    /// | 04.1 | Operação (Tipo de Operação) | `operacao` | 9–9 | 1 | Alfa | **'R' ou 'T'** | G028 |
    /// | 05.1 | Serviço (Tipo de Serviço) | `tipo_servico` | 10–11 | 2 | Num | **const '06'** | G025 |
    /// | 06.1 | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_1` | 12–13 | 2 | Alfa | **brancos** | G004 |
    /// | 07.1 | Layout do Lote (Versão do Layout) | `versao_layout_lote` | 14–16 | 3 | Num | **const '010'** | G030 |
    /// | 08.1 | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_2` | 17–17 | 1 | Alfa | **brancos** | G004 |
    /// | 09.1 | Tipo de Inscrição da Empresa | `tipo_inscricao_empresa` | 18–18 | 1 | Num | - | G005 |
    /// | 10.1 | Nº de Inscrição da Empresa | `num_inscricao_empresa` | 19–32 | 14 | Num | - | G006 |
    /// | 11.1 | Convênio (Código do Convênio no Banco) | `convenio` | 33–52 | 20 | Alfa | - | G007 |
    /// | 12.1 | Agência (Agência Mantenedora) | `agencia` | 53–57 | 5 | Num | - | G008 |
    /// | 13.1 | DV da Agência | `agencia_dv` | 58–58 | 1 | Alfa | - | G009 |
    /// | 14.1 | Conta Corrente (Número) | `conta` | 59–70 | 12 | Num | - | G010 |
    /// | 15.1 | DV da Conta | `conta_dv` | 71–71 | 1 | Alfa | - | G011 |
    /// | 16.1 | DV da Ag/Conta | `ag_conta_dv` | 72–72 | 1 | Alfa | - | G012 |
    /// | 17.1 | Nome da Empresa | `nome_empresa` | 73–102 | 30 | Alfa | - | G013 |
    /// | 18.1 | Uso reservado ao Banco remetente | `uso_banco` | 103–122 | 20 | Alfa | - | G021 |
    /// | 19.1 | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_3` | 123–230 | 108 | Alfa | **brancos** | G004 |
    /// | 20.1 | Ocorrências (Códigos das Ocorrências - Lote) | `cod_ocorrencias_lote` | 231–240 | 10 | Alfa | - | K001 |
    ///
    /// ## Observação
    /// - Assim como no Segmento D, o builder **não impõe** constantes (`'1'`, `'06'`, `'010'`) nem “brancos CNAB”.
    ///   Se você quiser garantir isso, a checagem pode ser feita em validação pós-parse.
    pub fn cnab240_header_lote_registro_1_parser() -> Result<FixedLengthParser<240>, ParseError> {
        FixedLengthParserBuilder::<240>::new()
            .add_field("banco", 3, '0', Alignment::Right)?                 
            .add_field("lote", 4, '0', Alignment::Right)?                  
            .add_field("tipo_registro", 1, '0', Alignment::Right)?        

            .add_field("operacao", 1, ' ', Alignment::Left)?               
            .add_field("tipo_servico", 2, '0', Alignment::Right)?          
            .add_field("cnab_reservado_1", 2, ' ', Alignment::Left)?       
            .add_field("versao_layout_lote", 3, '0', Alignment::Right)?    
            .add_field("cnab_reservado_2", 1, ' ', Alignment::Left)?       

            .add_field("tipo_inscricao_empresa", 1, '0', Alignment::Right)?
            .add_field("num_inscricao_empresa", 14, '0', Alignment::Right)?
            .add_field("convenio", 20, ' ', Alignment::Left)?              

            .add_field("agencia", 5, '0', Alignment::Right)?               
            .add_field("agencia_dv", 1, ' ', Alignment::Left)?             
            .add_field("conta", 12, '0', Alignment::Right)?                
            .add_field("conta_dv", 1, ' ', Alignment::Left)?               
            .add_field("ag_conta_dv", 1, ' ', Alignment::Left)?            

            .add_field("nome_empresa", 30, ' ', Alignment::Left)?          
            .add_field("uso_banco", 20, ' ', Alignment::Left)?             
            .add_field("cnab_reservado_3", 108, ' ', Alignment::Left)?      
            .add_field("cod_ocorrencias_lote", 10, ' ', Alignment::Left)?  
            .build()
    }

    /// Constrói o parser (240 bytes) do **CNAB240 – Trailer de Lote (Registro 5, layout 01.5)**.
    ///
    /// Layout conforme especificação enviada (01.5 … 12.5):
    /// - **Posições** do manual são **1-based**.
    /// - Campos `Num` → alinhamento à direita, preenchimento `'0'`.
    /// - Campos `Alfa` → alinhamento à esquerda, preenchimento `' '`.
    ///
    /// ## Mapeamento de campos (manual → código)
    ///
    /// | Seq (layout) | Campo (manual) | Nome no parser | Posições | Tam | Tipo | Const/Obs | Cód. |
    /// |---:|---|---|---:|---:|---|---|---|
    /// | 01.5 | Banco (Código do Banco na Compensação) | `banco` | 1–3 | 3 | Num | - | G001 |
    /// | 02.5 | Lote (Lote de Serviço) | `lote` | 4–7 | 4 | Num | - | G002 |
    /// | 03.5 | Registro (Trailer de Lote) | `tipo_registro` | 8–8 | 1 | Num | **const '5'** | G003 |
    /// | 04.5 | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_1` | 9–17 | 9 | Alfa | **brancos** | G004 |
    /// | 05.5 | Qtdade de registros do Lote | `qtd_registros_lote` | 18–23 | 6 | Num | - | G057 |
    /// | 06.5 | Valor Total dos Cheques do Lote | `valor_total_cheques` | 24–41 | 18 | Num | **16,2 (centavos)** | K021 |
    /// | 07.5 | Quantidade de Cheques do Lote | `qtd_cheques_lote` | 42–47 | 6 | Num | - | K022 |
    /// | 08.5 | Valor Total de Juros | `valor_total_juros` | 48–65 | 18 | Num | **16,2 (centavos)** | K023 |
    /// | 09.5 | Valor Total de IOF | `valor_total_iof` | 66–80 | 15 | Num | **13,2 (centavos)**, **zeros** | K024 |
    /// | 10.5 | Valor Total de Outros Encargos | `valor_total_outros_encargos` | 81–95 | 15 | Num | **13,2 (centavos)**, **zeros** | K025 |
    /// | 11.5 | CNAB (Uso Exclusivo FEBRABAN/CNAB) | `cnab_reservado_2` | 96–230 | 135 | Alfa | **brancos** | G004 |
    /// | 12.5 | Ocorrências (Códigos das Ocorrências - Lote) | `cod_ocorrencias_lote` | 231–240 | 10 | Alfa | - | K001 |
    ///
    /// ## Observações
    /// - Campos de valores são representados no arquivo como **inteiro sem separador**:
    ///   - `16,2` → 18 dígitos no total (ex.: `00000000000001234567` = 123.456,7? não; na real: 123.456,67).
    ///   - `13,2` → 15 dígitos no total.
    /// - O manual cita “Zeros” em IOF e Outros Encargos: aqui é só **padding com '0'**; validar “deve ser zero”
    ///   é regra de negócio (pós-parse), não do builder.
    pub fn cnab240_trailer_lote_registro_5_parser() -> Result<FixedLengthParser<240>, ParseError> {
        FixedLengthParserBuilder::<240>::new()
            .add_field("banco", 3, '0', Alignment::Right)?
            .add_field("lote", 4, '0', Alignment::Right)?
            .add_field("tipo_registro", 1, '0', Alignment::Right)?
            .add_field("cnab_reservado_1", 9, ' ', Alignment::Left)?
            .add_field("qtd_registros_lote", 6, '0', Alignment::Right)?
            .add_field("valor_total_cheques", 18, '0', Alignment::Right)?
            .add_field("qtd_cheques_lote", 6, '0', Alignment::Right)?
            .add_field("valor_total_juros", 18, '0', Alignment::Right)?
            .add_field("valor_total_iof", 15, '0', Alignment::Right)?
            .add_field("valor_total_outros_encargos", 15, '0', Alignment::Right)?
            .add_field("cnab_reservado_2", 135, ' ', Alignment::Left)?
            .add_field("cod_ocorrencias_lote", 10, ' ', Alignment::Left)?
            .build()
    }
}
