use super::span::{Span, Spanned, Spanning};
use super::YotType;
use super::Token;
use std::collections::HashMap;
pub use error::Error;
use super::get_opcode;
use super::InstructionKind;

mod error;

const PUSH_OPCODE: u8 = 0x20;
const SUBROUTINE_JUMP_OPCODE: u8 = 0x72;

#[derive(Copy, Clone)]
struct Offset {
    constant: usize,
    yot_type_dependent: usize,
}

impl Offset {
    pub fn new() -> Offset {
        Offset {
            constant: 0,
            yot_type_dependent: 0,
        }
    }

    pub fn render(&self, yot_type: YotType) -> usize {
        self.constant + self.yot_type_dependent * yot_type as usize
    }
}

impl<'a> std::ops::Add<Offset> for Offset {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Offset {
            constant: self.constant + rhs.constant,
            yot_type_dependent: self.yot_type_dependent + rhs.yot_type_dependent,
        }
    }
}

#[derive(Clone)]
enum Atom {
    Constant(ConstantKind),
    Addressable(Address, AddressableKind)
}

#[derive(Clone)]
enum ConstantKind {
    PrimitiveInstruction(InstructionKind),
    DataLiteral(Vec<u8>),
    AddressLiteral(u64),
}

#[derive(Copy, Clone)]
enum AddressableKind {
    LabelLiteral,
    SubroutineJump,
}

#[derive(Clone)]
enum Address {
    Deferred(Spanned<String>),
    Resolved(Offset),
}

#[derive(Clone)]
struct AtomStream {
    atoms: Vec<Atom>,
    offset: Offset,
}

impl AtomStream {
    pub fn new() -> AtomStream {
        AtomStream {
            atoms: Vec::new(),
            offset: Offset::new(),
        }
    }

    pub fn push_primitive_instruction(&mut self, instruction_kind: InstructionKind) {
        self.atoms.push(Atom::Constant(ConstantKind::PrimitiveInstruction(instruction_kind)));
        self.offset.constant += 1;
    }

    pub fn push_data_literal(&mut self, data: Vec<u8>) {
        self.offset.constant += data.len() * 2;
        self.atoms.push(Atom::Constant(ConstantKind::DataLiteral(data)));
    }

    pub fn push_address_literal(&mut self, address: u64) {
        self.atoms.push(Atom::Constant(ConstantKind::AddressLiteral(address)));
        self.offset.yot_type_dependent += 1;
    }

    pub fn push_label_literal(&mut self, label: String, span: Span) {
        self.atoms.push(Atom::Addressable(Address::Deferred(label.spanning(span)), AddressableKind::LabelLiteral));
        self.offset.yot_type_dependent += 2;
    }

    pub fn push_subroutine_jump(&mut self, label: String, span: Span) {
        self.atoms.push(Atom::Addressable(Address::Deferred(label.spanning(span)), AddressableKind::SubroutineJump));
        self.offset.constant += 1;
        self.offset.yot_type_dependent += 2;
    }

    pub fn offseted(&self, offset: Offset) -> AtomStream {
        AtomStream {
            atoms: self.atoms.iter().map(|atom: &Atom| -> Atom {
                match atom {
                    Atom::Addressable(Address::Resolved(o), kind) => Atom::Addressable(Address::Resolved(offset + *o), *kind),
                    x => x.clone(),
                }
            }).collect::<Vec<Atom>>(),
            offset: self.offset,
        }
    }

    pub fn resolved(self, label_definitions: &HashMap<String, (Offset, Span)>) -> AtomStream {
        AtomStream {
            atoms: self.atoms.iter().map(|atom: &Atom| -> Atom {
                match atom {
                    Atom::Addressable(Address::Deferred(Spanned { node: label, span }), kind) => {
                        if let Some((offset, _)) = label_definitions.get(&label.clone()) {
                            Atom::Addressable(Address::Resolved(*offset), *kind)
                        } else {
                            Atom::Addressable(Address::Deferred(label.clone().spanning(*span)), *kind)
                        }
                    },
                    x => x.clone(),
                }
            }).collect::<Vec<Atom>>(),
            offset: self.offset,
        }
    }

    pub fn extend(&mut self, other: AtomStream) {
        self.atoms.extend(other.atoms);
        self.offset = self.offset + other.offset;
    }

    pub fn atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    pub fn offset(&self) -> Offset {
        self.offset
    }
}

pub(super) fn emit(
    tokens: &[Spanned<Token>],
    yot_type: YotType,
    initial_stack_pointer: u64,
    exact_binary_size: Option<usize>,
) -> Result<Vec<u8>, Vec<Error>> {
    let mut atom_stream: AtomStream = AtomStream::new();

    atom_stream.push_address_literal(initial_stack_pointer);
    atom_stream.push_address_literal(yot_type as u64 * 2);

    let nested_atom_stream = emit_tokens(tokens)?;
    atom_stream.extend(nested_atom_stream.offseted(atom_stream.offset()));

    let mut binary: Vec<u8> = render(&atom_stream.atoms(), yot_type)?;

    if let Some(size) = exact_binary_size {
        pad(&mut binary, size).map_err(|err| vec![err])?;
    }

    Ok(binary)
}

fn emit_tokens(tokens: &[Spanned<Token>]) -> Result<AtomStream, Vec<Error>> {
    let mut errors: Vec<Error> = Vec::new();
    let mut atom_stream: AtomStream = AtomStream::new();
    let mut label_definitions: HashMap<String, (Offset, Span)> = HashMap::new();

    for token in tokens.iter() {
        match token {
            Spanned { node: Token::PrimitiveInstruction(instruction_kind), .. } => {
                atom_stream.push_primitive_instruction(*instruction_kind);
            }
            Spanned { node: Token::SubroutineJump(label), span } => {
                atom_stream.push_subroutine_jump(label.clone(), *span);
            }
            Spanned { node: Token::DataLiteral(byte_vector), .. } => {
                atom_stream.push_data_literal(byte_vector.to_owned());
            }
            Spanned { node: Token::LabelDefinition(label), span } => {
                if let Some((_, previous_span)) = label_definitions.insert(label.to_owned(), (atom_stream.offset(), *span)) {
                    errors.push(Error::LabelDefinedMoreThanOnce {
                        label: label.to_string(),
                        current_label_span: *span,
                        previously_defined_label_span: previous_span,
                    });
                }
            }
            Spanned { node: Token::LabelLiteral(label), span } => {
                atom_stream.push_label_literal(label.to_owned(), *span);
            }
        }
    }

    if errors.is_empty() {
        Ok(atom_stream.resolved(&label_definitions))
    } else {
        Err(errors)
    }
}

fn render(atoms: &[Atom], yot_type: YotType) -> Result<Vec<u8>, Vec<Error>> {
    let mut binary: Vec<u8> = Vec::new();
    let mut errors: Vec<Error> = Vec::new();

    for atom in atoms.iter() {
        match atom {
            Atom::Constant(constant_kind) => {
                match constant_kind {
                    ConstantKind::PrimitiveInstruction(instruction_kind) => {
                        binary.push(get_opcode(instruction_kind));
                    }
                    ConstantKind::DataLiteral(byte_vector) => {
                        for byte in byte_vector.iter() {
                            binary.push(PUSH_OPCODE);
                            binary.push(*byte);
                        }
                    }
                    ConstantKind::AddressLiteral(address) => {
                        for i in 0..yot_type as usize {
                            let index = yot_type as usize - i - 1;
                            let mask = 0xff << (index * 2);
                            binary.push((address & mask >> (index * 2)) as u8);
                        }
                    }
                }
            },
            Atom::Addressable(Address::Resolved(offset), addressable_kind) => {
                let address = offset.render(yot_type);
                match addressable_kind {
                    AddressableKind::LabelLiteral => {
                        for i in 0..yot_type as usize {
                            binary.push(PUSH_OPCODE);
                            let index = yot_type as usize - i - 1;
                            let mask = 0xff << (index * 2);
                            binary.push((address & mask >> (index * 2)) as u8);
                        }
                    }
                    AddressableKind::SubroutineJump => {
                        for i in 0..yot_type as usize {
                            binary.push(PUSH_OPCODE);
                            let index = yot_type as usize - i - 1;
                            let mask = 0xff << (index * 2);
                            binary.push((address & mask >> (index * 2)) as u8);
                        }
                        binary.push(SUBROUTINE_JUMP_OPCODE);
                    }
                }
            }
            Atom::Addressable(Address::Deferred(Spanned { node: label, span }), _) => {
                errors.push(Error::CannotFindLabel {
                    label: label.to_string(),
                    span: *span,
                });
            }
        };
    }

    if errors.is_empty() {
        Ok(binary)
    } else {
        Err(errors)
    }
}

fn pad(binary: &mut Vec<u8>, size: usize) -> Result<(), Error> {
    if binary.len() > size {
        Err(Error::BinaryTooLarge {
            current_size: binary.len(),
            requested_size: size,
        })
    } else {
        while binary.len() < size {
            binary.push(0);
        }
        Ok(())
    }
}
