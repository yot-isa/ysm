use super::span::{Span, Spanned, Spanning};
use super::YotType;
use super::Token;
use std::collections::HashMap;
pub use error::Error;
use super::get_opcode;

mod error;

const PUSH_OPCODE: u8 = 0x20;
const JUMP_TO_SUBROUTINE_OPCODE: u8 = 0x72;

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

    pub fn pad(&mut self, size: usize) {
        while self.size() < size {
            self.push(0x00);
        }
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

pub(super) fn emit(
    tokens: &[Spanned<Token>],
    yot_type: YotType,
    initial_stack_pointer: u64,
    exact_binary_size: Option<usize>,
) -> Result<Vec<u8>, Vec<Error>> {
    let mut binary: Binary = Binary::new();
    let mut encountered_label_definitions: HashMap<String, (usize, Span)> = HashMap::new();
    let mut encountered_label_literals: HashMap<usize, Spanned<String>> = HashMap::new();
    let mut errors: Vec<Error> = Vec::new();

    binary.push_address(initial_stack_pointer, yot_type);
    binary.push_address(yot_type as u64 * 3, yot_type);

    for token in tokens.iter() {
        match token {
            Spanned { node: Token::PrimitiveInstruction(instruction_kind), .. } => {
                binary.push(get_opcode(instruction_kind));
            }
            Spanned { node: Token::SubroutineJump(label), span } => {
                encountered_label_literals.insert(binary.size(), label.clone().spanning(*span));
                for _ in 0..yot_type as usize {
                    binary.push(PUSH_OPCODE);
                    binary.push(0x00);
                }
                binary.push(JUMP_TO_SUBROUTINE_OPCODE);
            }
            Spanned { node: Token::DataLiteral(byte_vector), .. } => {
                for byte in byte_vector.iter() {
                    binary.push(PUSH_OPCODE);
                    binary.push(*byte);
                }
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
                encountered_label_literals.insert(binary.size(), label.clone().spanning(*span));
                for _ in 0..yot_type as usize {
                    // Na jutro: tu i wyżej trzeba jakos zamienic na enumy tak jak na branchu ze
                    // scopami
                    binary.push(PUSH_OPCODE);
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

    if let Some(size) = exact_binary_size {
        if binary.size() > size {
            errors.push(Error::BinaryTooLarge {
                current_size: binary.size(),
                requested_size: size,
            });
        } else {
            binary.pad(size);
        }
    }

    if errors.is_empty() {
        Ok(binary.data())
    } else {
        Err(errors)
    }
}
