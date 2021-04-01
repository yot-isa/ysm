use super::span::{Span, Spanned, Spanning};
use super::YotType;
use super::{Token, DataLiteral};
use std::collections::HashMap;
pub use error::Error;

mod error;

const LITERAL_ADDRESS_OPCODE: u8 = 0x02;
const LITERAL_DATA_OPCODE: u8 = 0x03;
const JUMP_TO_SUBROUTINE_OPCODE: u8 = 0x30;

struct Binary {
    data: Vec<u8>
}

impl Binary {
    pub fn new() -> Binary {
        Binary {
            data: Vec::new(),
        }
    }

    pub fn push(&mut self, value: u8) {
        self.data.push(value);
    }

    pub fn push_address(&mut self, address: u64, yot_type: YotType) {
        for i in 0..yot_type as usize {
            let index = yot_type as usize - i - 1;
            let mask = 0xff << (index * 2);
            self.push((address & mask >> (index * 2)) as u8);
        }
    }

    pub fn set(&mut self, offset: usize, value: u8) {
        self.data[offset] = value;
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

pub(super) fn emit(tokens: &[Spanned<Token>], yot_type: YotType, initial_data_stack_pointer: u64, initial_address_stack_pointer: u64) -> Result<Vec<u8>, Vec<Error>> {
    let mut binary: Binary = Binary::new();
    let mut encountered_label_definitions: HashMap<String, (usize, Span)> = HashMap::new();
    let mut encountered_label_literals: HashMap<usize, Spanned<String>> = HashMap::new();
    let mut errors: Vec<Error> = Vec::new();

    binary.push_address(initial_data_stack_pointer, yot_type);
    binary.push_address(initial_address_stack_pointer, yot_type);
    binary.push_address(yot_type as u64 * 3, yot_type);

    for token in tokens.iter() {
        match token {
            Spanned { node: Token::PrimitiveInstruction(opcode), .. } => {
                binary.push(*opcode);
            }
            Spanned { node: Token::SubroutineJump(label), span } => {
                binary.push(LITERAL_ADDRESS_OPCODE);
                encountered_label_literals.insert(binary.size(), label.clone().spanning(*span));
                for _ in 0..yot_type as usize {
                    binary.push(0x00);
                }
                binary.push(JUMP_TO_SUBROUTINE_OPCODE);
            }
            Spanned { node: Token::DataLiteral(data_literal), .. } => {
                match data_literal {
                    DataLiteral::U8(value) => {
                        binary.push(LITERAL_DATA_OPCODE);
                        binary.push(*value);
                    }
                    DataLiteral::U16(value) => {
                        binary.push(LITERAL_DATA_OPCODE + 0x10);
                        let mut shifted_value: u16 = *value;
                        for _ in 0..2 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                    }
                    DataLiteral::U32(value) => {
                        binary.push(LITERAL_DATA_OPCODE + 0x20);
                        let mut shifted_value: u32 = *value;
                        for _ in 0..4 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                    }
                    DataLiteral::U64(value) => {
                        binary.push(LITERAL_DATA_OPCODE + 0x30);
                        let mut shifted_value: u64 = *value;
                        for _ in 0..8 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                    }
                }
            }
            Spanned { node: Token::AddressLiteral(address_literal), .. } => {
                binary.push(LITERAL_ADDRESS_OPCODE);
                binary.push_address(*address_literal, yot_type);
            }
            Spanned { node: Token::LabelDefinition(label), span } => {
                if let Some((_, previous_span)) = encountered_label_definitions.insert(label.clone(), (binary.size(), *span)) {
                    errors.push(Error::LabelDefinedMoreThanOnce {
                        label: label.to_string(),
                        current_label_span: *span,
                        previously_defined_label_span: previous_span,
                    });
                }
            }
            Spanned { node: Token::LabelLiteral(label), span } => {
                binary.push(LITERAL_ADDRESS_OPCODE);
                encountered_label_literals.insert(binary.size(), label.clone().spanning(*span));
                for _ in 0..yot_type as usize {
                    binary.push(0x00);
                }
            }
        }
    }

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
            binary.set(offset + yot_type as usize - 1 - j, (shifted_address & 0xff) as u8);
            shifted_address >>= 8;
        }
    }

    if errors.is_empty() {
        Ok(binary.data())
    } else {
        Err(errors)
    }
}
