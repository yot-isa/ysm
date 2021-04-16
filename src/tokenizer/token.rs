use super::{Span, Spanned, Spanning};
use super::InstructionKind;
use crate::impl_spanning;

#[derive(Debug, Clone)]
pub enum Token {
    PrimitiveInstruction(InstructionKind),
    SubroutineJump(String),
    DataLiteral(Vec<u8>),
    LabelDefinition(String),
    LabelLiteral(String),
    OpeningBrace,
    ClosingBrace,
}

impl_spanning!(Token);
