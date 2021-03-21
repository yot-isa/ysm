use super::span::{Location, Span, Spanned, Spanning};
use super::instruction::YotType;
use super::{Token, DataLiteral};
use std::collections::HashMap;

pub(super) fn emit(tokens: &[Spanned<Token>], yot_type: YotType) -> Result<Vec<u8>, ()> {
    let mut binary: Vec<u8> = Vec::new();
    let mut i: usize = 0;
    let mut encountered_label_definitions: HashMap<String, usize> = HashMap::new();
    let mut encountered_label_literals: HashMap<usize, String> = HashMap::new();

    for token in tokens.iter() {
        match token {
            Spanned { node: Token::PrimitiveInstruction(opcode), .. } => {
                binary.push(*opcode);
                i += 1;
            }
            Spanned { node: Token::SubroutineJump(label), .. } => {
                binary.push(0x02); // lit^
                i += 1;
                encountered_label_literals.insert(i, label.clone());
                for _ in 0..yot_type as usize {
                    binary.push(0x00); // value
                }
                binary.push(0x30); // jsr
                i += yot_type as usize + 1;
            }
            Spanned { node: Token::DataLiteral(data_literal), .. } => {
                match data_literal {
                    DataLiteral::U8(value) => {
                        binary.push(0x03); // lit1
                        binary.push(*value);
                        i += 2;
                    }
                    DataLiteral::U16(value) => {
                        binary.push(0x13); // lit2
                        let mut shifted_value: u16 = *value;
                        for _ in 0..2 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                        i += 3;
                    }
                    DataLiteral::U32(value) => {
                        binary.push(0x23); // lit4
                        let mut shifted_value: u32 = *value;
                        for _ in 0..4 {
                            binary.push((shifted_value & 0xff) as u8);
                            shifted_value >>= 8;
                        }
                        i += 5;
                    }
                    DataLiteral::U64(value) => {
                        binary.push(0x33); // lit8
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
                binary.push(0x02); // lit^
                let mut shifted_value: u64 = *address_literal;
                for _ in 0..yot_type as usize {
                    binary.push((shifted_value & 0xff) as u8);
                    shifted_value >>= 8;
                }
                i += 1 + yot_type as usize;
            }
            Spanned { node: Token::LabelDefinition(label), .. } => {
                encountered_label_definitions.insert(label.clone(), i);
            }
            Spanned { node: Token::LabelLiteral(label), .. } => {
                binary.push(0x02);
                i += 1;
                encountered_label_literals.insert(i, label.clone());
                for _ in 0..yot_type as usize {
                    binary.push(0x00);
                }
                i += yot_type as usize;
            }
        }
    }

    let i: usize = i;

    for (offset, label) in encountered_label_literals.into_iter() {
        let mut shifted_address: usize = *encountered_label_definitions.get(&label).unwrap();
        println!("{:?} => {:?} => {:?}", offset, label, shifted_address);
        for j in 0..yot_type as usize {
            binary[offset + yot_type as usize - 1 - j] = (shifted_address & 0xff) as u8;
            shifted_address >>= 8;
        }
    }

    Ok(binary)
}
