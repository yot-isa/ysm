use super::super::instruction::PrimitiveInstruction;
use super::{Span, Spanned, Spanning};
use crate::impl_spanning;

#[derive(Debug, Clone)]
pub enum Token {
    PrimitiveInstruction(PrimitiveInstruction),
    SubroutineJump(String),
    DataLiteral(DataLiteral),
    AddressLiteral(u64),
    LabelDeclaration(String),
    LabelLiteral(String),
}

#[derive(Debug, Clone, Copy)]
pub enum DataLiteral {
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
}

impl_spanning!(Token);
impl_spanning!(DataLiteral);
