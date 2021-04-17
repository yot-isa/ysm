use super::{Span, Spanned, Spanning};
use crate::impl_spanning;
use super::InstructionKind;

#[derive(Debug, Clone)]
pub enum Token {
    PrimitiveInstruction(InstructionKind),
    SubroutineJump(String),
    DataLiteral(Vec<u8>),
    LabelDefinition(String),
    LabelLiteral(String),
}

impl_spanning!(Token);
