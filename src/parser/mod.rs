pub use ast::{Scope, Statement};
use super::span::{Span, Spanned, Spanning};
use super::{Token, DataLiteral};
pub use error::Error;

mod ast;
mod error;

pub(super) fn parse(tokens: &[Spanned<Token>]) -> Result<Spanned<Scope>, Vec<Error>> {
    todo!()
}
