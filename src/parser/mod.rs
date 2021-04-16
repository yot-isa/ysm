pub use ast::{Scope, Statement};
use super::span::{Span, Spanned, Spanning};
use super::Token;
pub use error::Error;
use super::InstructionKind;

mod ast;
mod error;

pub(super) fn parse(tokens: &[Spanned<Token>]) -> Result<Spanned<Scope>, Vec<Error>> {
    todo!()
}

fn parse_block(tokens: &[Spanned<Token>], braced: bool) -> Result<Spanned<Scope>, Vec<Error>> {
    todo!()
}
