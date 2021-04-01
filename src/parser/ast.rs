use super::{Token, DataLiteral};
use super::{Span, Spanned, Spanning};
use crate::impl_spanning;

#[derive(Clone)]
pub enum Statement {
    PrimitiveInstruction(u8),
    SubroutineJump(String),
    DataLiteral(DataLiteral),
    AddressLiteral(u64),
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
