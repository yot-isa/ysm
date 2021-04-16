use super::{Span, Spanned, Spanning};
use crate::impl_spanning;
use super::InstructionKind;

#[derive(Clone)]
pub enum Statement {
    PrimitiveInstruction(InstructionKind),
    SubroutineJump(String),
    DataLiteral(Vec<u8>),
    LabelDefinition(String),
    LabelLiteral(String),
    Scope(Scope),
}

#[derive(Clone)]
pub struct Scope {
    pub children: Vec<Spanned<Statement>>
}

impl_spanning!(Statement);
impl_spanning!(Scope);
