use super::span::{Location, Span, Spanned, Spanning};
use super::instruction::YotType;
use super::{Token, DataLiteral};
use std::collections::HashMap;
use error::Error;

mod error;

const LITERAL_ADDRESS_OPCODE: u8 = 0x02;
const LITERAL_DATA_OPCODE: u8 = 0x03;
const JUMP_TO_SUBROUTINE_OPCODE: u8 = 0x30;

pub(super) fn emit(tokens: &[Spanned<Token>], yot_type: YotType) -> Result<Vec<u8>, Vec<Error>> {
    let mut binary: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    let mut encountered_label_definitions: HashMap<String, (usize, Span)> = HashMap::new();
    let mut encountered_label_literals: HashMap<usize, Spanned<String>> = HashMap::new();
    let mut errors: Vec<Error> = Vec::new();

    for token in tokens.iter() {
        match token {
            Spanned { node: Token::PrimitiveInstruction(opcode), .. } => {
                binary.push(*opcode);
                i += 1;
            }
            Spanned { node: Token::SubroutineJump(label), span } => {
                binary.push(LITERAL_ADDRESS_OPCODE);
                i += 1;
                encountered_label_literals.insert(i, label.clone().spanning(*span));
                for _ in 0..yot_type as usize {
                    binary.push(0x00);
                }
                binary.push(JUMP_TO_SUBROUTINE_OPCODE);
                i += yot_type as usize + 1;
            }
            Spanned { node: Token::DataLiteral(data_literal), .. } => {
                match data_literal {
                    DataLiteral::U8(value) => {
                        binary.push(LITERAL_DATA_OPCODE);
                        binary.push(*value);
                        i += 2;
                    }
                    DataLiteral::U16(value) => {
                        binary.push(LITERAL_DATA_OPCODE + 0x10);
                        let mut shifted_value: u16 = *value;
                        for _ in 0..2 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                        i += 3;
                    }
                    DataLiteral::U32(value) => {
                        binary.push(LITERAL_DATA_OPCODE + 0x20);
                        let mut shifted_value: u32 = *value;
                        for _ in 0..4 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                        i += 5;
                    }
                    DataLiteral::U64(value) => {
                        binary.push(LITERAL_DATA_OPCODE + 0x30);
                        let mut shifted_value: u64 = *value;
                        for _ in 0..8 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                        i += 9;
                    }
                }
            }
            Spanned { node: Token::AddressLiteral(address_literal), .. } => {
                binary.push(LITERAL_ADDRESS_OPCODE);
                let mut shifted_value: u64 = *address_literal;
                for _ in 0..yot_type as usize {
                    binary.push((shifted_value & 0xff) as u8);
                    shifted_value >>= 8;
                }
                i += 1 + yot_type as usize;
            }
            Spanned { node: Token::LabelDefinition(label), span } => {
                if let Some((_, previous_span)) = encountered_label_definitions.insert(label.clone(), (i, *span)) {
                    errors.push(Error::LabelDefinedMoreThanOnce {
                        label: label.to_string(),
                        current_label_span: *span,
                        previously_defined_label_span: previous_span,
                    });
                }
            }
            Spanned { node: Token::LabelLiteral(label), span } => {
                binary.push(LITERAL_ADDRESS_OPCODE);
                i += 1;
                encountered_label_literals.insert(i, label.clone().spanning(*span));
                for _ in 0..yot_type as usize {
                    binary.push(0x00);
                }
                i += yot_type as usize;
            }
        }
    }

    let i: usize = i;

    for (offset, Spanned { node: label, span }) in encountered_label_literals.into_iter() {
        let mut shifted_address: usize = match encountered_label_definitions.get(&label) {
            Some((address, _)) => *address,
            None => {
                errors.push(Error::CannotFindLabel {
                    label,
                    span,
                });
                continue;
            }
        };
        for j in 0..yot_type as usize {
            binary[offset + yot_type as usize - 1 - j] = (shifted_address & 0xff) as u8;
            shifted_address >>= 8;
        }
    }

    if errors.is_empty() {
        Ok(binary)
    } else {
        Err(errors)
    }
}
